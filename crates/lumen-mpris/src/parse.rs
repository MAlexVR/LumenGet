//! Turns what an MPRIS player reports into the port's `NowPlaying`. Pure, so
//! it is tested without a bus.

use std::collections::HashMap;

use lumen_core::{NowPlaying, RepeatMode};
use zbus::zvariant::{OwnedValue, Value};

/// `xesam:artist` is a list of names; a player that sends one string is accepted too.
fn artist(metadata: &HashMap<String, OwnedValue>) -> String {
    let Some(value) = metadata.get("xesam:artist") else {
        return String::new();
    };
    if let Ok(one) = <&str>::try_from(&**value) {
        return one.to_string();
    }
    match &**value {
        Value::Array(names) => names
            .iter()
            .filter_map(|n| <&str>::try_from(n).ok())
            .collect::<Vec<_>>()
            .join(", "),
        _ => String::new(),
    }
}

fn title(metadata: &HashMap<String, OwnedValue>) -> String {
    metadata
        .get("xesam:title")
        .and_then(|v| <&str>::try_from(&**v).ok())
        .unwrap_or_default()
        .to_string()
}

/// What the player reported on its own properties, beside the metadata.
pub(crate) struct Reading<'a> {
    pub status: &'a str,
    pub volume: Option<f64>,
    /// MPRIS counts in microseconds.
    pub position_us: Option<i64>,
    pub shuffle: bool,
    pub loop_status: &'a str,
    pub can_previous: bool,
    pub can_next: bool,
}

/// Microseconds as seconds; a negative count is nonsense and is dropped.
fn seconds(micros: i64) -> Option<f64> {
    (micros >= 0).then(|| micros as f64 / 1_000_000.0)
}

/// `mpris:length` is a signed or an unsigned 64-bit count, depending on the player.
fn length_secs(metadata: &HashMap<String, OwnedValue>) -> Option<f64> {
    let value = &**metadata.get("mpris:length")?;
    let micros = i64::try_from(value).ok().or_else(|| {
        u64::try_from(value)
            .ok()
            .and_then(|n| i64::try_from(n).ok())
    })?;
    seconds(micros)
}

/// Only an `https` cover is shown; anything else (a local file, a script URL) is dropped.
fn art_url(metadata: &HashMap<String, OwnedValue>) -> Option<String> {
    let url = <&str>::try_from(&**metadata.get("mpris:artUrl")?).ok()?;
    url.starts_with("https://").then(|| url.to_string())
}

fn repeat(loop_status: &str) -> RepeatMode {
    match loop_status {
        "Track" => RepeatMode::Track,
        "Playlist" => RepeatMode::Playlist,
        _ => RepeatMode::None,
    }
}

pub(crate) fn now_playing(
    reading: &Reading<'_>,
    metadata: &HashMap<String, OwnedValue>,
) -> NowPlaying {
    NowPlaying {
        title: title(metadata),
        artist: artist(metadata),
        art_url: art_url(metadata),
        playing: reading.status == "Playing",
        position_secs: reading.position_us.and_then(seconds),
        length_secs: length_secs(metadata),
        shuffle: reading.shuffle,
        repeat: repeat(reading.loop_status),
        volume: reading.volume,
        can_previous: reading.can_previous,
        can_next: reading.can_next,
    }
}

#[cfg(test)]
mod tests {
    use lumen_core::RepeatMode;

    use super::*;

    fn owned(value: Value<'_>) -> OwnedValue {
        value.try_to_owned().unwrap()
    }

    fn metadata(pairs: Vec<(&str, Value<'_>)>) -> HashMap<String, OwnedValue> {
        pairs
            .into_iter()
            .map(|(k, v)| (k.to_string(), owned(v)))
            .collect()
    }

    fn reading(status: &str) -> Reading<'_> {
        Reading {
            status,
            volume: None,
            position_us: None,
            shuffle: false,
            loop_status: "None",
            can_previous: false,
            can_next: false,
        }
    }

    #[test]
    fn title_and_artists_are_read_from_the_metadata() {
        let meta = metadata(vec![
            ("xesam:title", Value::from("Song")),
            ("xesam:artist", Value::from(vec!["Ana", "Luis"])),
        ]);
        let now = now_playing(
            &Reading {
                volume: Some(0.5),
                can_previous: true,
                can_next: true,
                ..reading("Playing")
            },
            &meta,
        );
        assert_eq!(now.title, "Song");
        assert_eq!(now.artist, "Ana, Luis");
        assert!(now.playing);
        assert_eq!(now.volume, Some(0.5));
        assert!(now.can_previous && now.can_next);
    }

    #[test]
    fn only_the_playing_status_counts_as_playing() {
        let meta = metadata(vec![]);
        assert!(now_playing(&reading("Playing"), &meta).playing);
        assert!(!now_playing(&reading("Paused"), &meta).playing);
        assert!(!now_playing(&reading("Stopped"), &meta).playing);
    }

    #[test]
    fn a_single_artist_string_is_accepted() {
        let meta = metadata(vec![("xesam:artist", Value::from("Solo"))]);
        assert_eq!(now_playing(&reading("Paused"), &meta).artist, "Solo");
    }

    #[test]
    fn missing_fields_become_empty_or_absent() {
        let now = now_playing(&reading("Paused"), &metadata(vec![]));
        assert_eq!(now.title, "");
        assert_eq!(now.artist, "");
        assert_eq!(now.volume, None);
        assert_eq!(now.art_url, None);
        assert_eq!(now.position_secs, None);
        assert_eq!(now.length_secs, None);
    }

    #[test]
    fn a_field_of_the_wrong_type_is_ignored() {
        let meta = metadata(vec![
            ("xesam:title", Value::from(7_u32)),
            ("xesam:artist", Value::from(7_u32)),
            ("mpris:artUrl", Value::from(7_u32)),
            ("mpris:length", Value::from("long")),
        ]);
        let now = now_playing(&reading("Paused"), &meta);
        assert_eq!((now.title.as_str(), now.artist.as_str()), ("", ""));
        assert_eq!((now.art_url, now.length_secs), (None, None));
    }

    #[test]
    fn the_cover_is_kept_only_when_it_is_an_https_link() {
        let cover = |url: &str| {
            now_playing(
                &reading("Paused"),
                &metadata(vec![("mpris:artUrl", Value::from(url.to_string()))]),
            )
            .art_url
        };
        assert_eq!(
            cover("https://i.scdn.co/image/abc"),
            Some("https://i.scdn.co/image/abc".to_string())
        );
        assert_eq!(cover("http://i.scdn.co/image/abc"), None);
        assert_eq!(cover("file:///home/me/.cache/cover.jpg"), None);
        assert_eq!(cover("javascript:alert(1)"), None);
        assert_eq!(cover(""), None);
    }

    #[test]
    fn position_and_length_come_in_microseconds_and_leave_in_seconds() {
        let signed = metadata(vec![("mpris:length", Value::from(263_000_000_i64))]);
        let unsigned = metadata(vec![("mpris:length", Value::from(263_000_000_u64))]);
        let at = Reading {
            position_us: Some(37_500_000),
            ..reading("Playing")
        };
        let now = now_playing(&at, &signed);
        assert_eq!(now.position_secs, Some(37.5));
        assert_eq!(now.length_secs, Some(263.0));
        assert_eq!(now_playing(&at, &unsigned).length_secs, Some(263.0));
    }

    #[test]
    fn a_negative_position_or_length_is_not_reported() {
        let meta = metadata(vec![("mpris:length", Value::from(-1_i64))]);
        let at = Reading {
            position_us: Some(-5),
            ..reading("Playing")
        };
        let now = now_playing(&at, &meta);
        assert_eq!((now.position_secs, now.length_secs), (None, None));
    }

    #[test]
    fn shuffle_and_repeat_are_read_from_the_player() {
        let meta = metadata(vec![]);
        for (loop_status, mode) in [
            ("None", RepeatMode::None),
            ("Track", RepeatMode::Track),
            ("Playlist", RepeatMode::Playlist),
            ("Something new", RepeatMode::None),
        ] {
            let at = Reading {
                shuffle: true,
                loop_status,
                ..reading("Playing")
            };
            let now = now_playing(&at, &meta);
            assert!(now.shuffle);
            assert_eq!(now.repeat, mode, "{loop_status}");
        }
    }
}
