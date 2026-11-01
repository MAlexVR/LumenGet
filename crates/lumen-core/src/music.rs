//! Port of the music player a workout can drive. Adapters talk to the
//! player the owner already runs (Spotify); Lumen never plays music itself.

use async_trait::async_trait;
use serde::Serialize;

/// What the player is doing right now.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub title: String,
    pub artist: String,
    pub playing: bool,
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
            playing: true,
            volume: Some(0.5),
            can_previous: false,
            can_next: true,
        };
        let json = serde_json::to_value(&now).unwrap();
        assert_eq!(json["canPrevious"], false);
        assert_eq!(json["canNext"], true);
        assert_eq!(json["volume"], 0.5);
    }
}
