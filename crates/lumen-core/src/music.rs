//! Port of the music player a workout can drive. Adapters talk to the
//! player the owner already runs (Spotify); Lumen never plays music itself.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// What the player does when a song or the list ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepeatMode {
    #[default]
    None,
    Track,
    Playlist,
}

/// What the player is doing right now.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub title: String,
    pub artist: String,
    /// The album cover, an `https` link; `None` when there is none.
    pub art_url: Option<String>,
    pub playing: bool,
    /// Where the song is, in seconds; `None` when the player does not say.
    pub position_secs: Option<f64>,
    /// How long the song lasts, in seconds.
    pub length_secs: Option<f64>,
    pub shuffle: bool,
    pub repeat: RepeatMode,
    /// 0.0 to 1.0; `None` when the player does not expose its volume.
    pub volume: Option<f64>,
    pub can_previous: bool,
    pub can_next: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MusicError {
    #[error("no music player is running")]
    Unavailable,
    #[error("the volume must be a number from 0 to 1")]
    InvalidVolume,
    #[error("the music player refused the command: {0}")]
    Failed(String),
}

#[async_trait]
pub trait MusicControl: Send + Sync {
    /// `Ok(None)` when no supported player is running.
    async fn now_playing(&self) -> Result<Option<NowPlaying>, MusicError>;
    async fn play_pause(&self) -> Result<(), MusicError>;
    async fn next(&self) -> Result<(), MusicError>;
    async fn previous(&self) -> Result<(), MusicError>;
    /// `volume` is 0.0 to 1.0.
    async fn set_volume(&self, volume: f64) -> Result<(), MusicError>;
    async fn set_shuffle(&self, enabled: bool) -> Result<(), MusicError>;
    async fn set_repeat(&self, mode: RepeatMode) -> Result<(), MusicError>;
}

/// The volume a command may ask for: 0.0 to 1.0, never NaN or infinite.
pub fn validated_volume(volume: f64) -> Result<f64, MusicError> {
    if (0.0..=1.0).contains(&volume) {
        Ok(volume)
    } else {
        Err(MusicError::InvalidVolume)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_volume_from_zero_to_one_is_accepted() {
        assert_eq!(validated_volume(0.0), Ok(0.0));
        assert_eq!(validated_volume(0.42), Ok(0.42));
        assert_eq!(validated_volume(1.0), Ok(1.0));
    }

    #[test]
    fn a_volume_outside_the_range_or_not_a_number_is_refused() {
        for v in [-0.01, 1.01, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(validated_volume(v), Err(MusicError::InvalidVolume), "{v}");
        }
    }

    #[test]
    fn now_playing_is_camel_case_json() {
        let now = NowPlaying {
            title: "Song".into(),
            artist: "Band".into(),
            art_url: Some("https://i.scdn.co/image/x".into()),
            playing: true,
            position_secs: Some(37.5),
            length_secs: Some(263.0),
            shuffle: true,
            repeat: RepeatMode::Playlist,
            volume: Some(0.5),
            can_previous: false,
            can_next: true,
        };
        let json = serde_json::to_value(&now).unwrap();
        assert_eq!(json["canPrevious"], false);
        assert_eq!(json["canNext"], true);
        assert_eq!(json["volume"], 0.5);
        assert_eq!(json["artUrl"], "https://i.scdn.co/image/x");
        assert_eq!(json["positionSecs"], 37.5);
        assert_eq!(json["lengthSecs"], 263.0);
        assert_eq!(json["shuffle"], true);
        assert_eq!(json["repeat"], "playlist");
    }

    #[test]
    fn a_repeat_mode_is_spelled_in_lowercase_both_ways() {
        for (mode, text) in [
            (RepeatMode::None, "none"),
            (RepeatMode::Track, "track"),
            (RepeatMode::Playlist, "playlist"),
        ] {
            assert_eq!(serde_json::to_value(mode).unwrap(), text);
            assert_eq!(
                serde_json::from_value::<RepeatMode>(text.into()).unwrap(),
                mode
            );
        }
        assert!(serde_json::from_value::<RepeatMode>("always".into()).is_err());
    }
}
