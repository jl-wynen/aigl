use eframe::egui::{self, Color32, Stroke};

const GRAY: [Color32; 12] = [
    Color32::from_rgb(17, 17, 17),
    Color32::from_rgb(25, 25, 25),
    Color32::from_rgb(34, 34, 34),
    Color32::from_rgb(42, 42, 42),
    Color32::from_rgb(49, 49, 49),
    Color32::from_rgb(58, 58, 58),
    Color32::from_rgb(72, 72, 72),
    Color32::from_rgb(96, 96, 96),
    Color32::from_rgb(110, 110, 110),
    Color32::from_rgb(123, 123, 123),
    Color32::from_rgb(180, 180, 180),
    Color32::from_rgb(238, 238, 238),
];
const MAUVE: [Color32; 12] = [
    Color32::from_rgb(18, 17, 19),
    Color32::from_rgb(26, 25, 27),
    Color32::from_rgb(35, 34, 37),
    Color32::from_rgb(43, 41, 45),
    Color32::from_rgb(50, 48, 53),
    Color32::from_rgb(60, 57, 63),
    Color32::from_rgb(73, 71, 78),
    Color32::from_rgb(98, 95, 105),
    Color32::from_rgb(111, 109, 120),
    Color32::from_rgb(124, 122, 133),
    Color32::from_rgb(181, 178, 188),
    Color32::from_rgb(238, 238, 240),
];
const RUBY: [Color32; 12] = [
    Color32::from_rgb(25, 17, 19),
    Color32::from_rgb(30, 21, 23),
    Color32::from_rgb(58, 20, 30),
    Color32::from_rgb(78, 19, 37),
    Color32::from_rgb(94, 26, 46),
    Color32::from_rgb(111, 37, 57),
    Color32::from_rgb(136, 52, 71),
    Color32::from_rgb(179, 68, 90),
    Color32::from_rgb(229, 70, 102),
    Color32::from_rgb(236, 90, 114),
    Color32::from_rgb(255, 148, 157),
    Color32::from_rgb(254, 210, 225),
];
const GRASS: [Color32; 12] = [
    Color32::from_rgb(14, 21, 17),
    Color32::from_rgb(20, 26, 21),
    Color32::from_rgb(27, 42, 30),
    Color32::from_rgb(29, 58, 36),
    Color32::from_rgb(37, 72, 45),
    Color32::from_rgb(45, 87, 54),
    Color32::from_rgb(54, 103, 64),
    Color32::from_rgb(62, 121, 73),
    Color32::from_rgb(70, 167, 88),
    Color32::from_rgb(83, 179, 101),
    Color32::from_rgb(113, 208, 131),
    Color32::from_rgb(194, 240, 194),
];

pub fn set_style(style: &mut egui::Style) {
    style.visuals.panel_fill = GRAY[1];

    style.visuals.widgets.inactive.weak_bg_fill = GRASS[2];
    style.visuals.widgets.hovered.weak_bg_fill = GRASS[3];
    style.visuals.widgets.active.weak_bg_fill = GRASS[4];

    style.visuals.widgets.noninteractive.bg_fill = GRAY[1];
    style.visuals.widgets.noninteractive.fg_stroke = Stroke {
        color: GRAY[11],
        ..Default::default()
    };
    style.visuals.widgets.noninteractive.bg_stroke = Stroke {
        color: GRAY[5],
        width: 1.0,
    };

    let stroke = Stroke {
        color: GRASS[11],
        ..Default::default()
    };
    style.visuals.widgets.inactive.fg_stroke = stroke;
    style.visuals.widgets.hovered.fg_stroke = stroke;
    style.visuals.widgets.active.fg_stroke = stroke;
}

/*
consider
use egui::{epaint, style, Color32};

/// Apply the given theme to a [`Context`](egui::Context).
pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(egui::Visuals {
        override_text_color: Some(theme.text),
        hyperlink_color: theme.rosewater,
        faint_bg_color: theme.surface0,
        extreme_bg_color: theme.crust,
        code_bg_color: theme.mantle,
        warn_fg_color: theme.peach,
        error_fg_color: theme.maroon,
        window_fill: theme.base,
        panel_fill: theme.base,
        window_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.window_stroke
        },
        widgets: style::Widgets {
            noninteractive: make_widget_visual(old.widgets.noninteractive, &theme, theme.base),
            inactive: make_widget_visual(old.widgets.inactive, &theme, theme.surface0),
            hovered: make_widget_visual(old.widgets.hovered, &theme, theme.surface2),
            active: make_widget_visual(old.widgets.active, &theme, theme.surface1),
            open: make_widget_visual(old.widgets.open, &theme, theme.surface0),
        },
        selection: style::Selection {
            bg_fill: theme
                .blue
                .linear_multiply(if theme == LATTE { 0.4 } else { 0.2 }),
            stroke: egui::Stroke {
                color: theme.overlay1,
                ..old.selection.stroke
            },
        },
        window_shadow: epaint::Shadow {
            color: theme.base,
            ..old.window_shadow
        },
        popup_shadow: epaint::Shadow {
            color: theme.base,
            ..old.popup_shadow
        },
        ..old
    });
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Theme {
    pub rosewater: Color32,
    pub flamingo: Color32,
    pub pink: Color32,
    pub mauve: Color32,
    pub red: Color32,
    pub maroon: Color32,
    pub peach: Color32,
    pub yellow: Color32,
    pub green: Color32,
    pub teal: Color32,
    pub sky: Color32,
    pub sapphire: Color32,
    pub blue: Color32,
    pub lavender: Color32,
    pub text: Color32,
    pub subtext1: Color32,
    pub subtext0: Color32,
    pub overlay2: Color32,
    pub overlay1: Color32,
    pub overlay0: Color32,
    pub surface2: Color32,
    pub surface1: Color32,
    pub surface0: Color32,
    pub base: Color32,
    pub mantle: Color32,
    pub crust: Color32,
}

pub const LATTE: Theme = Theme {
    rosewater: Color32::from_rgb(220, 138, 120),
    flamingo: Color32::from_rgb(221, 120, 120),
    pink: Color32::from_rgb(234, 118, 203),
    mauve: Color32::from_rgb(136, 57, 239),
    red: Color32::from_rgb(210, 15, 57),
    maroon: Color32::from_rgb(230, 69, 83),
    peach: Color32::from_rgb(254, 100, 11),
    yellow: Color32::from_rgb(223, 142, 29),
    green: Color32::from_rgb(64, 160, 43),
    teal: Color32::from_rgb(23, 146, 153),
    sky: Color32::from_rgb(4, 165, 229),
    sapphire: Color32::from_rgb(32, 159, 181),
    blue: Color32::from_rgb(30, 102, 245),
    lavender: Color32::from_rgb(114, 135, 253),
    text: Color32::from_rgb(76, 79, 105),
    subtext1: Color32::from_rgb(92, 95, 119),
    subtext0: Color32::from_rgb(108, 111, 133),
    overlay2: Color32::from_rgb(124, 127, 147),
    overlay1: Color32::from_rgb(140, 143, 161),
    overlay0: Color32::from_rgb(156, 160, 176),
    surface2: Color32::from_rgb(172, 176, 190),
    surface1: Color32::from_rgb(188, 192, 204),
    surface0: Color32::from_rgb(204, 208, 218),
    base: Color32::from_rgb(239, 241, 245),
    mantle: Color32::from_rgb(230, 233, 239),
    crust: Color32::from_rgb(220, 224, 232),
};

 */
