//! Turns what an MPRIS player reports into the port's `NowPlaying`. Pure, so
//! it is tested without a bus.

use std::collections::HashMap;

use lumen_core::NowPlaying;
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

pub(crate) fn now_playing(
    status: &str,
    volume: Option<f64>,
    metadata: &HashMap<String, OwnedValue>,
    can_previous: bool,
    can_next: bool,
) -> NowPlaying {
    NowPlaying {
        title: title(metadata),
        artist: artist(metadata),
        playing: status == "Playing",
        volume,
        can_previous,
        can_next,
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn title_and_artists_are_read_from_the_metadata() {
        let meta = metadata(vec![
            ("xesam:title", Value::from("Song")),
            ("xesam:artist", Value::from(vec!["Ana", "Luis"])),
        ]);
        let now = now_playing("Playing", Some(0.5), &meta, true, true);
        assert_eq!(now.title, "Song");
        assert_eq!(now.artist, "Ana, Luis");
        assert!(now.playing);
        assert_eq!(now.volume, Some(0.5));
        assert!(now.can_previous && now.can_next);
    }

    #[test]
    fn only_the_playing_status_counts_as_playing() {
        let meta = metadata(vec![]);
        assert!(now_playing("Playing", None, &meta, false, false).playing);
        assert!(!now_playing("Paused", None, &meta, false, false).playing);
        assert!(!now_playing("Stopped", None, &meta, false, false).playing);
    }

    #[test]
    fn a_single_artist_string_is_accepted() {
        let meta = metadata(vec![("xesam:artist", Value::from("Solo"))]);
        assert_eq!(
            now_playing("Paused", None, &meta, false, false).artist,
            "Solo"
        );
    }

    #[test]
    fn missing_fields_become_empty_text() {
        let now = now_playing("Paused", None, &metadata(vec![]), false, false);
        assert_eq!(now.title, "");
        assert_eq!(now.artist, "");
        assert_eq!(now.volume, None);
    }

    #[test]
    fn a_field_of_the_wrong_type_is_ignored() {
        let meta = metadata(vec![
            ("xesam:title", Value::from(7_u32)),
            ("xesam:artist", Value::from(7_u32)),
        ]);
        let now = now_playing("Paused", None, &meta, false, false);
        assert_eq!((now.title.as_str(), now.artist.as_str()), ("", ""));
    }
}
