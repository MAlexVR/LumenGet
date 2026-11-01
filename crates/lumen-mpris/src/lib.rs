//! Controls the owner's Spotify through MPRIS, the D-Bus interface Linux
//! media players expose. No network, no account: only the local session bus.

mod parse;
mod player;

pub use player::MprisSpotify;
