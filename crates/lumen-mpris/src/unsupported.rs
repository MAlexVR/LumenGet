use async_trait::async_trait;
use lumen_core::{MusicControl, MusicError, NowPlaying};

/// The music control on a system without MPRIS: Spotify is never found, so
/// the player's control stays hidden.
#[derive(Default)]
#[cfg_attr(target_os = "linux", allow(dead_code))]
pub struct NoMusicPlayer;

#[async_trait]
impl MusicControl for NoMusicPlayer {
    async fn now_playing(&self) -> Result<Option<NowPlaying>, MusicError> {
        Ok(None)
    }
    async fn play_pause(&self) -> Result<(), MusicError> {
        Err(MusicError::Unavailable)
    }
    async fn next(&self) -> Result<(), MusicError> {
        Err(MusicError::Unavailable)
    }
    async fn previous(&self) -> Result<(), MusicError> {
        Err(MusicError::Unavailable)
    }
    async fn set_volume(&self, volume: f64) -> Result<(), MusicError> {
        lumen_core::validated_volume(volume)?;
        Err(MusicError::Unavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn no_player_is_ever_found_and_commands_say_so() {
        let none = NoMusicPlayer;
        assert_eq!(none.now_playing().await, Ok(None));
        assert_eq!(none.play_pause().await, Err(MusicError::Unavailable));
        assert_eq!(none.set_volume(0.5).await, Err(MusicError::Unavailable));
        assert_eq!(none.set_volume(2.0).await, Err(MusicError::InvalidVolume));
    }
}
