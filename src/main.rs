#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod game_config;
mod install_app;

fn main() {
    install_app::GameInstallApp::run();
}

// use eframe::egui::{
//     self, containers::Frame, style::Selection, Align, Color32, Direction, Layout, Stroke,
// };
//
// fn main() -> eframe::Result {
//     let options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default(),
//         centered: true,
//         ..Default::default()
//     };
//     eframe::run_native(
//         "Install AI Game",
//         options,
//         Box::new(|cc| Ok(Box::new(GameInstallApp::new(cc)))),
//     )
// }
//
// const GRAY: [Color32; 12] = [
//     Color32::from_rgb(17, 17, 17),
//     Color32::from_rgb(25, 25, 25),
//     Color32::from_rgb(34, 34, 34),
//     Color32::from_rgb(42, 42, 42),
//     Color32::from_rgb(49, 49, 49),
//     Color32::from_rgb(58, 58, 58),
//     Color32::from_rgb(72, 72, 72),
//     Color32::from_rgb(96, 96, 96),
//     Color32::from_rgb(110, 110, 110),
//     Color32::from_rgb(123, 123, 123),
//     Color32::from_rgb(180, 180, 180),
//     Color32::from_rgb(238, 238, 238),
// ];
// const MAUVE: [Color32; 12] = [
//     Color32::from_rgb(18, 17, 19),
//     Color32::from_rgb(26, 25, 27),
//     Color32::from_rgb(35, 34, 37),
//     Color32::from_rgb(43, 41, 45),
//     Color32::from_rgb(50, 48, 53),
//     Color32::from_rgb(60, 57, 63),
//     Color32::from_rgb(73, 71, 78),
//     Color32::from_rgb(98, 95, 105),
//     Color32::from_rgb(111, 109, 120),
//     Color32::from_rgb(124, 122, 133),
//     Color32::from_rgb(181, 178, 188),
//     Color32::from_rgb(238, 238, 240),
// ];
// const RUBY: [Color32; 12] = [
//     Color32::from_rgb(25, 17, 19),
//     Color32::from_rgb(30, 21, 23),
//     Color32::from_rgb(58, 20, 30),
//     Color32::from_rgb(78, 19, 37),
//     Color32::from_rgb(94, 26, 46),
//     Color32::from_rgb(111, 37, 57),
//     Color32::from_rgb(136, 52, 71),
//     Color32::from_rgb(179, 68, 90),
//     Color32::from_rgb(229, 70, 102),
//     Color32::from_rgb(236, 90, 114),
//     Color32::from_rgb(255, 148, 157),
//     Color32::from_rgb(254, 210, 225),
// ];
// const GRASS: [Color32; 12] = [
//     Color32::from_rgb(14, 21, 17),
//     Color32::from_rgb(20, 26, 21),
//     Color32::from_rgb(27, 42, 30),
//     Color32::from_rgb(29, 58, 36),
//     Color32::from_rgb(37, 72, 45),
//     Color32::from_rgb(45, 87, 54),
//     Color32::from_rgb(54, 103, 64),
//     Color32::from_rgb(62, 121, 73),
//     Color32::from_rgb(70, 167, 88),
//     Color32::from_rgb(83, 179, 101),
//     Color32::from_rgb(113, 208, 131),
//     Color32::from_rgb(194, 240, 194),
// ];
//
// fn set_style(style: &mut egui::Style) {
//     style.visuals.panel_fill = GRAY[1];
//
//     style.visuals.widgets.inactive.weak_bg_fill = GRASS[2];
//     style.visuals.widgets.hovered.weak_bg_fill = GRASS[3];
//     style.visuals.widgets.active.weak_bg_fill = GRASS[4];
//
//     let stroke = Stroke {
//         color: GRASS[11],
//         ..Default::default()
//     };
//     style.visuals.widgets.inactive.fg_stroke = stroke;
//     style.visuals.widgets.hovered.fg_stroke = stroke;
//     style.visuals.widgets.active.fg_stroke = stroke;
// }
//
// struct GameInstallApp {
//     game_code: String,
//     game_name: Option<String>,
//     install_location: String,
// }
//
// impl GameInstallApp {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         cc.egui_ctx.style_mut(set_style);
//         Self {
//             game_code: String::new(),
//             game_name: None,
//             install_location: String::new(),
//         }
//     }
// }
//
// impl eframe::App for GameInstallApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         ctx.set_pixels_per_point(2.0);
//         egui::TopBottomPanel::bottom("progress_panel").show(ctx, |ui| {
//             ui.horizontal(|ui| {
//                 ui.add(
//                     egui::widgets::ProgressBar::new(0.2)
//                         .desired_width(ui.available_width() * 0.8)
//                         .desired_height(ui.available_height() * 0.2)
//                         .corner_radius(0.5),
//                 );
//
//                 let install_enabled = self.game_name.is_some() && !self.install_location.is_empty();
//                 ui.add_enabled(install_enabled, egui::Button::new("Install"))
//             })
//         });
//
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Install Game");
//             ui.horizontal(|ui| {
//                 let label = ui.label("Game code:");
//                 ui.text_edit_singleline(&mut self.game_code)
//                     .labelled_by(label.id);
//             });
//             if ui
//                 .add_enabled(!self.game_code.is_empty(), egui::Button::new("Fetch"))
//                 .clicked()
//             {
//                 self.game_name = Some("Test game".to_owned());
//             }
//             if let Some(game_name) = &self.game_name {
//                 ui.label(format!("Game name: {game_name}"));
//             }
//             ui.add_space(50.0);
//
//             let path_label = ui.label("Install location:");
//             ui.horizontal(|ui| {
//                 ui.text_edit_singleline(&mut self.install_location)
//                     .labelled_by(path_label.id);
//                 if ui.button("Browse").clicked() {
//                     if let Some(path) = rfd::FileDialog::new().pick_folder() {
//                         self.install_location = path.display().to_string();
//                     }
//                 }
//             })
//         });
//     }
// }
