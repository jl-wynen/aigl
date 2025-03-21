use eframe::egui;

pub fn add_enabled_with_colors(
    ui: &mut egui::Ui,
    enabled: bool,
    visuals: &egui::style::Widgets,
    widget: impl egui::Widget,
) -> egui::Response {
    if enabled {
        let old_widgets = std::mem::replace(&mut ui.visuals_mut().widgets, visuals.clone());
        ui.visuals_mut().widgets = visuals.clone();
        let response = ui.add(widget);
        ui.visuals_mut().widgets = old_widgets;
        response
    } else {
        ui.add_enabled(false, widget)
    }
}
