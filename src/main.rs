#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod game_config;
mod install_app;
mod theme;

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
