use crate::color::Color;
use anyhow::{Context, Result};
use eframe::{
    egui::{self, CornerRadius, Stroke, style},
    epaint,
};
use std::cell::OnceCell;

#[cfg_attr(feature = "load", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct Theme {
    pub dark_mode: bool,
    pub base: Scale,
    pub highlight: Scale,
    pub warning: Scale,
    pub error: Scale,

    pub stroke_width: f32,
    pub element_border_width: f32,
    pub corner_radius: u8,

    #[cfg_attr(feature = "load", serde(skip))]
    base_widget_visuals: OnceCell<style::Widgets>,
    #[cfg_attr(feature = "load", serde(skip))]
    highlight_widget_visuals: OnceCell<style::Widgets>,
    #[cfg_attr(feature = "load", serde(skip))]
    warning_widget_visuals: OnceCell<style::Widgets>,
    #[cfg_attr(feature = "load", serde(skip))]
    error_widget_visuals: OnceCell<style::Widgets>,
}

#[cfg_attr(feature = "load", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct Scale {
    pub bg: Color,
    pub bg_subtle: Color,
    pub bg_element: Color,
    pub bg_element_hovered: Color,
    pub bg_element_active: Color,
    pub border: Color,
    pub border_element: Color,
    pub border_element_hovered: Color,
    pub bg_solid: Color,
    pub bg_solid_hovered: Color,
    pub fg_low_contrast: Color,
    pub fg_high_contrast: Color,
}

impl Theme {
    #[cfg(feature = "load")]
    pub fn load(path: &std::path::Path) -> Result<Self> {
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        let mut ron_string = String::new();
        file.read_to_string(&mut ron_string)?;
        ron::from_str(&ron_string).context("When parsing theme")
    }

    #[cfg(feature = "load")]
    pub fn get_selected() -> Self {
        // TODO cache
        Self::load(
            &std::path::Path::new(env!("CARGO_WORKSPACE_DIR"))
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

    pub fn base_widget_visuals(&self) -> &style::Widgets {
        self.base_widget_visuals
            .get_or_init(|| self.make_widgets(&self.base))
    }

    pub fn highlight_widget_visuals(&self) -> &style::Widgets {
        self.highlight_widget_visuals
            .get_or_init(|| self.make_widgets(&self.highlight))
    }

    pub fn warning_widget_visuals(&self) -> &style::Widgets {
        self.warning_widget_visuals
            .get_or_init(|| self.make_widgets(&self.warning))
    }

    pub fn error_widget_visuals(&self) -> &style::Widgets {
        self.error_widget_visuals
            .get_or_init(|| self.make_widgets(&self.error))
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let old = ctx.style().visuals.clone();
        ctx.set_visuals(egui::Visuals {
            dark_mode: self.dark_mode,

            window_fill: self.base.bg.0,
            panel_fill: self.base.bg.0,
            faint_bg_color: self.base.bg_subtle.0,
            extreme_bg_color: self.base.bg_element.0,
            code_bg_color: self.base.bg_subtle.0,

            hyperlink_color: self.highlight.fg_high_contrast.0,
            warn_fg_color: self.warning.fg_high_contrast.0,
            error_fg_color: self.error.fg_high_contrast.0,

            window_shadow: epaint::Shadow {
                color: self.base.bg_subtle.0,
                ..old.window_shadow
            },
            popup_shadow: epaint::Shadow {
                color: self.base.bg_subtle.0,
                ..old.popup_shadow
            },
            text_cursor: style::TextCursorStyle {
                stroke: Stroke {
                    color: self.base.fg_high_contrast.0,
                    ..old.text_cursor.stroke
                },
                ..old.text_cursor
            },
            selection: style::Selection {
                bg_fill: self.highlight.bg_solid.0,
                stroke: Stroke {
                    color: self.highlight.border_element.0,
                    ..old.selection.stroke
                },
            },
            widgets: self.make_widgets(&self.base),
            ..old
        });
    }

    fn make_widgets(&self, scale: &Scale) -> style::Widgets {
        let base_visuals = style::WidgetVisuals {
            bg_fill: scale.bg_element.0,
            weak_bg_fill: scale.bg_element.0,
            bg_stroke: Stroke {
                color: scale.border_element.0,
                width: self.element_border_width,
            },
            corner_radius: CornerRadius::same(self.corner_radius),
            fg_stroke: Stroke {
                color: scale.fg_high_contrast.0,
                width: self.stroke_width,
            },
            expansion: 0.0,
        };
        let hovered = style::WidgetVisuals {
            bg_fill: scale.bg_element_hovered.0,
            weak_bg_fill: scale.bg_element_hovered.0,
            bg_stroke: Stroke {
                color: scale.border_element_hovered.0,
                ..base_visuals.bg_stroke
            },
            ..base_visuals
        };
        let active = style::WidgetVisuals {
            bg_fill: scale.bg_element_active.0,
            weak_bg_fill: scale.bg_element_active.0,
            bg_stroke: Stroke {
                color: scale.border_element_hovered.0,
                ..base_visuals.bg_stroke
            },
            ..base_visuals
        };
        style::Widgets {
            noninteractive: base_visuals,
            inactive: base_visuals,
            hovered,
            active,
            open: active,
        }
    }
}
