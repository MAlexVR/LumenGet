//! Controls the owner's Spotify through MPRIS, the D-Bus interface Linux
//! media players expose. No network, no account: only the local session bus.
//! Other systems get a control that never finds a player.

#[cfg(target_os = "linux")]
mod parse;
#[cfg(target_os = "linux")]
mod player;
mod unsupported;

#[cfg(target_os = "linux")]
pub use player::MprisSpotify;
#[cfg(not(target_os = "linux"))]
pub use unsupported::NoMusicPlayer as MprisSpotify;
