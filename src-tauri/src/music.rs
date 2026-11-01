//! The music control the player drives: a thin layer over the `MusicControl`
//! port that gives the UI stable `{code, message}` errors.

use std::sync::Arc;

use lumen_core::{validated_volume, MusicControl, MusicError, NowPlaying, RepeatMode};
use serde::Serialize;

/// Stable machine-readable error codes for the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MusicCode {
    /// Spotify is not running.
    Unavailable,
    InvalidVolume,
    Failed,
}

/// Serialized to the UI as `{ "code": "...", "message": "..." }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, thiserror::Error)]
#[error("{message}")]
pub struct MusicCommandError {
    pub code: MusicCode,
    pub message: String,
}

impl From<MusicError> for MusicCommandError {
    fn from(e: MusicError) -> Self {
        let code = match e {
            MusicError::Unavailable => MusicCode::Unavailable,
            MusicError::InvalidVolume => MusicCode::InvalidVolume,
            MusicError::Failed(_) => MusicCode::Failed,
        };
        Self {
            code,
            message: e.to_string(),
        }
    }
}

pub struct MusicService {
    control: Arc<dyn MusicControl>,
}

impl MusicService {
    pub fn new(control: Arc<dyn MusicControl>) -> Self {
        Self { control }
    }

    pub async fn now_playing(&self) -> Result<Option<NowPlaying>, MusicCommandError> {
        Ok(self.control.now_playing().await?)
    }

    pub async fn play_pause(&self) -> Result<(), MusicCommandError> {
        Ok(self.control.play_pause().await?)
    }

    pub async fn next(&self) -> Result<(), MusicCommandError> {
        Ok(self.control.next().await?)
    }

    pub async fn previous(&self) -> Result<(), MusicCommandError> {
        Ok(self.control.previous().await?)
    }

    pub async fn set_volume(&self, volume: f64) -> Result<(), MusicCommandError> {
        Ok(self.control.set_volume(validated_volume(volume)?).await?)
    }

    pub async fn set_shuffle(&self, enabled: bool) -> Result<(), MusicCommandError> {
        Ok(self.control.set_shuffle(enabled).await?)
    }

    pub async fn set_repeat(&self, mode: RepeatMode) -> Result<(), MusicCommandError> {
        Ok(self.control.set_repeat(mode).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;

    /// Records what the service asked of the player.
    #[derive(Default)]
    struct Fake {
        playing: Option<NowPlaying>,
        calls: Mutex<Vec<String>>,
    }

    impl Fake {
        fn log(&self, what: &str) -> Result<(), MusicError> {
            self.calls.lock().unwrap().push(what.to_string());
            if self.playing.is_some() {
                Ok(())
            } else {
                Err(MusicError::Unavailable)
            }
        }
    }

    #[async_trait]
    impl MusicControl for Fake {
        async fn now_playing(&self) -> Result<Option<NowPlaying>, MusicError> {
            Ok(self.playing.clone())
        }
        async fn play_pause(&self) -> Result<(), MusicError> {
            self.log("play_pause")
        }
        async fn next(&self) -> Result<(), MusicError> {
            self.log("next")
        }
        async fn previous(&self) -> Result<(), MusicError> {
            self.log("previous")
        }
        async fn set_volume(&self, volume: f64) -> Result<(), MusicError> {
            self.log(&format!("volume {volume}"))
        }
        async fn set_shuffle(&self, enabled: bool) -> Result<(), MusicError> {
            self.log(&format!("shuffle {enabled}"))
        }
        async fn set_repeat(&self, mode: RepeatMode) -> Result<(), MusicError> {
            self.log(&format!("repeat {mode:?}"))
        }
    }

    fn song() -> NowPlaying {
        NowPlaying {
            title: "Song".into(),
            artist: "Band".into(),
            art_url: None,
            playing: true,
            position_secs: Some(12.0),
            length_secs: Some(200.0),
            shuffle: false,
            repeat: RepeatMode::None,
            volume: Some(0.5),
            can_previous: true,
            can_next: true,
        }
    }

    fn service(playing: Option<NowPlaying>) -> (MusicService, Arc<Fake>) {
        let fake = Arc::new(Fake {
            playing,
            ..Fake::default()
        });
        (MusicService::new(fake.clone()), fake)
    }

    #[tokio::test]
    async fn what_is_playing_is_passed_through_and_no_player_is_not_an_error() {
        let (with, _) = service(Some(song()));
        assert_eq!(with.now_playing().await.unwrap(), Some(song()));
        let (without, _) = service(None);
        assert_eq!(without.now_playing().await.unwrap(), None);
    }

    #[tokio::test]
    async fn the_buttons_reach_the_player() {
        let (svc, fake) = service(Some(song()));
        svc.previous().await.unwrap();
        svc.play_pause().await.unwrap();
        svc.next().await.unwrap();
        svc.set_volume(0.25).await.unwrap();
        assert_eq!(
            *fake.calls.lock().unwrap(),
            ["previous", "play_pause", "next", "volume 0.25"]
        );
    }

    #[tokio::test]
    async fn shuffle_and_repeat_reach_the_player() {
        let (svc, fake) = service(Some(song()));
        svc.set_shuffle(true).await.unwrap();
        svc.set_repeat(RepeatMode::Track).await.unwrap();
        assert_eq!(
            *fake.calls.lock().unwrap(),
            ["shuffle true", "repeat Track"]
        );
    }

    #[tokio::test]
    async fn a_closed_player_is_an_unavailable_error() {
        let (svc, _) = service(None);
        let err = svc.play_pause().await.unwrap_err();
        assert_eq!(err.code, MusicCode::Unavailable);
        assert!(!err.message.is_empty());
    }

    #[tokio::test]
    async fn a_bad_volume_never_reaches_the_player() {
        let (svc, fake) = service(Some(song()));
        let err = svc.set_volume(1.5).await.unwrap_err();
        assert_eq!(err.code, MusicCode::InvalidVolume);
        assert!(fake.calls.lock().unwrap().is_empty());
    }

    #[test]
    fn errors_serialize_as_code_and_message() {
        let err = MusicCommandError::from(MusicError::Failed("boom".into()));
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["code"], "failed");
        assert!(json["message"].as_str().unwrap().contains("boom"));
    }
}
