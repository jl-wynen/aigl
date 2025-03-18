use eframe::egui;

pub fn add_enabled_with_colors(
    ui: &mut egui::Ui,
    enabled: bool,
    visuals: &egui::style::Widgets,
    widget: impl egui::Widget,
) -> egui::Response {
    // TODO use some way to scope the visuals to only this button
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        if enabled {
            ui.visuals_mut().widgets = visuals.clone();
        }
        ui.add_enabled(enabled, widget)
    })
    .inner
}
