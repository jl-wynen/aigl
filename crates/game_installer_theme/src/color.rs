use eframe::egui::{Color32, ecolor::ParseHexColorError};
use serde::{
    Deserialize, Deserializer,
    de::{self, Visitor},
};
use std::fmt::Formatter;

/// Custom color type that can be deserialized from a string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color(Color32);

impl Color {
    pub fn from_hex(hex: &str) -> Result<Self, ParseHexColorError> {
        Color32::from_hex(hex).map(Self)
    }
}

// Custom deserializer for Color / Color32 because the implementation in
// egui cannot parse hex strings directly.
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ColorVisitor)
    }
}

struct ColorVisitor;

impl Visitor<'_> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string with a hex color code")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Color::from_hex(value).map_err(|err| E::custom(format!("invalid hex color code: {err:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_deserialize_color_from_ron() {
        let color_str = "\"#ff0000\"";
        assert_eq!(
            ron::from_str::<Color>(color_str).unwrap(),
            Color(Color32::from_rgb(255, 0, 0))
        );
        let color_str = "\"#0f0\"";
        assert_eq!(
            ron::from_str::<Color>(color_str).unwrap(),
            Color(Color32::from_rgb(0, 255, 0))
        );
    }

    #[test]
    fn deserialize_color_requires_hash() {
        let color_str = "\"00ff00\"";
        assert!(ron::from_str::<Color>(color_str).is_err());
    }
}
