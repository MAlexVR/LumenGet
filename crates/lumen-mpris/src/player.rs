use std::collections::HashMap;

use async_trait::async_trait;
use lumen_core::{validated_volume, MusicControl, MusicError, NowPlaying, RepeatMode};
use tokio::sync::OnceCell;
use zbus::proxy::CacheProperties;
use zbus::zvariant::OwnedValue;
use zbus::Connection;

use crate::parse::{now_playing, Reading};

/// Spotify's bus name; other players (a browser, Lumen's own webview) are
/// never controlled, so a workout cannot pause the wrong thing.
const SPOTIFY: &str = "org.mpris.MediaPlayer2.spotify";

#[zbus::proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait Player {
    fn play_pause(&self) -> zbus::Result<()>;
    fn next(&self) -> zbus::Result<()>;
    fn previous(&self) -> zbus::Result<()>;
    #[zbus(property)]
    fn playback_status(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn metadata(&self) -> zbus::Result<HashMap<String, OwnedValue>>;
    #[zbus(property)]
    fn volume(&self) -> zbus::Result<f64>;
    #[zbus(property)]
    fn set_volume(&self, volume: f64) -> zbus::Result<()>;
    #[zbus(property)]
    fn position(&self) -> zbus::Result<i64>;
    #[zbus(property)]
    fn shuffle(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_shuffle(&self, enabled: bool) -> zbus::Result<()>;
    #[zbus(property)]
    fn loop_status(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_loop_status(&self, status: &str) -> zbus::Result<()>;
    #[zbus(property)]
    fn can_go_next(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn can_go_previous(&self) -> zbus::Result<bool>;
}

fn failed(e: &zbus::Error) -> MusicError {
    MusicError::Failed(e.to_string())
}

/// The owner's Spotify on the D-Bus session bus.
#[derive(Default)]
pub struct MprisSpotify {
    connection: OnceCell<Connection>,
}

impl MprisSpotify {
    pub fn new() -> Self {
        Self::default()
    }

    async fn bus(&self) -> Result<&Connection, MusicError> {
        self.connection
            .get_or_try_init(|| async { Connection::session().await })
            .await
            .map_err(|e| failed(&e))
    }

    /// The running Spotify, or `None` when it is closed.
    async fn player(&self) -> Result<Option<PlayerProxy<'static>>, MusicError> {
        let bus = self.bus().await?;
        let names = zbus::fdo::DBusProxy::new(bus)
            .await
            .map_err(|e| failed(&e))?
            .list_names()
            .await
            .map_err(zbus::Error::from)
            .map_err(|e| failed(&e))?;
        let Some(name) = names.iter().find(|n| n.starts_with(SPOTIFY)) else {
            return Ok(None);
        };
        let proxy = PlayerProxy::builder(bus)
            .destination(name.to_string())
            .map_err(|e| failed(&e))?
            // Every call is a fresh read of what Spotify says now.
            .cache_properties(CacheProperties::No)
            .build()
            .await
            .map_err(|e| failed(&e))?;
        Ok(Some(proxy))
    }

    async fn running(&self) -> Result<PlayerProxy<'static>, MusicError> {
        self.player().await?.ok_or(MusicError::Unavailable)
    }
}

#[async_trait]
impl MusicControl for MprisSpotify {
    async fn now_playing(&self) -> Result<Option<NowPlaying>, MusicError> {
        let Some(player) = self.player().await? else {
            return Ok(None);
        };
        let status = player.playback_status().await.map_err(|e| failed(&e))?;
        let metadata = player.metadata().await.map_err(|e| failed(&e))?;
        let loop_status = player.loop_status().await.unwrap_or_default();
        Ok(Some(now_playing(
            &Reading {
                status: &status,
                volume: player.volume().await.ok(),
                position_us: player.position().await.ok(),
                shuffle: player.shuffle().await.unwrap_or(false),
                loop_status: &loop_status,
                can_previous: player.can_go_previous().await.unwrap_or(false),
                can_next: player.can_go_next().await.unwrap_or(false),
            },
            &metadata,
        )))
    }

    async fn play_pause(&self) -> Result<(), MusicError> {
        self.running()
            .await?
            .play_pause()
            .await
            .map_err(|e| failed(&e))
    }

    async fn next(&self) -> Result<(), MusicError> {
        self.running().await?.next().await.map_err(|e| failed(&e))
    }

    async fn previous(&self) -> Result<(), MusicError> {
        self.running()
            .await?
            .previous()
            .await
            .map_err(|e| failed(&e))
    }

    async fn set_volume(&self, volume: f64) -> Result<(), MusicError> {
        let volume = validated_volume(volume)?;
        self.running()
            .await?
            .set_volume(volume)
            .await
            .map_err(|e| failed(&e))
    }

    async fn set_shuffle(&self, enabled: bool) -> Result<(), MusicError> {
        self.running()
            .await?
            .set_shuffle(enabled)
            .await
            .map_err(|e| failed(&e))
    }

    async fn set_repeat(&self, mode: RepeatMode) -> Result<(), MusicError> {
        let status = match mode {
            RepeatMode::None => "None",
            RepeatMode::Track => "Track",
            RepeatMode::Playlist => "Playlist",
        };
        self.running()
            .await?
            .set_loop_status(status)
            .await
            .map_err(|e| failed(&e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn an_invalid_volume_is_refused_before_touching_the_bus() {
        let err = MprisSpotify::new().set_volume(1.5).await.unwrap_err();
        assert_eq!(err, MusicError::InvalidVolume);
    }

    /// Reads the owner's running Spotify; changes nothing.
    /// `cargo test -p lumen-mpris -- --ignored --nocapture`
    #[tokio::test]
    #[ignore = "needs Spotify running on the session bus"]
    async fn reads_the_real_spotify() {
        let now = MprisSpotify::new().now_playing().await.unwrap();
        println!("{now:?}");
        assert!(now.is_some(), "open Spotify first");
    }

    /// Flips shuffle and repeat on the owner's Spotify and puts them back at once.
    /// `cargo test -p lumen-mpris -- --ignored --nocapture round_trip`
    #[tokio::test]
    #[ignore = "briefly changes shuffle and repeat on the running Spotify"]
    async fn shuffle_and_repeat_round_trip_on_the_real_spotify() {
        let spotify = MprisSpotify::new();
        let before = spotify
            .now_playing()
            .await
            .unwrap()
            .expect("open Spotify first");
        let (flipped_shuffle, flipped_repeat) = (
            !before.shuffle,
            if before.repeat == RepeatMode::Playlist {
                RepeatMode::None
            } else {
                RepeatMode::Playlist
            },
        );
        let changed = async {
            spotify.set_shuffle(flipped_shuffle).await?;
            spotify.set_repeat(flipped_repeat).await?;
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            spotify.now_playing().await
        }
        .await;
        // Always put the owner's settings back, whatever happened above.
        let restored_shuffle = spotify.set_shuffle(before.shuffle).await;
        let restored_repeat = spotify.set_repeat(before.repeat).await;
        let after = changed.unwrap().unwrap();
        println!(
            "flipped: shuffle {} repeat {:?}",
            after.shuffle, after.repeat
        );
        assert_eq!(after.shuffle, flipped_shuffle);
        assert_eq!(after.repeat, flipped_repeat);
        restored_shuffle.unwrap();
        restored_repeat.unwrap();
    }
}
