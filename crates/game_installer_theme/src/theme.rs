use crate::color::Color;
use anyhow::{Context, Result};
use eframe::egui;
use std::io::Read;
use std::path::Path;

#[cfg_attr(feature = "load", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct Theme {
    pub app_background: Color,
}

impl Theme {
    #[cfg(feature = "load")]
    pub fn load(path: &Path) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut ron_string = String::new();
        file.read_to_string(&mut ron_string)?;
        ron::from_str(&ron_string).context("When parsing theme")
    }

    #[cfg(feature = "load")]
    pub fn get_selected() -> Self {
        Self::load(
            &Path::new(env!("CARGO_WORKSPACE_DIR"))
                .join("resources")
                .join("themes")
                .join("radix.ron"),
        )
        .expect("Failed to load theme")
    }

    #[cfg(not(feature = "load"))]
    pub const fn get_selected() -> &'static Self {
        &crate::radix::RADIX_THEME
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let old = ctx.style().visuals.clone();
        ctx.set_visuals(egui::Visuals {
            panel_fill: self.app_background.0,
            ..old
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn deserialize_theme() {
        let theme = Theme::load(
            &Path::new(env!("CARGO_WORKSPACE_DIR"))
                .join("resources")
                .join("themes")
                .join("radix.ron"),
        )
        .unwrap();
        assert_eq!(theme.app_background, Color::from_hex("#ff0000").unwrap());
    }
}
