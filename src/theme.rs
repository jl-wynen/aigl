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

    let stroke = Stroke {
        color: GRASS[11],
        ..Default::default()
    };
    style.visuals.widgets.inactive.fg_stroke = stroke;
    style.visuals.widgets.hovered.fg_stroke = stroke;
    style.visuals.widgets.active.fg_stroke = stroke;
}
