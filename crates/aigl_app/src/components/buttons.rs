use eframe::egui::{self, Button};

pub fn icon_button<'a>(text: &str, icon: &str) -> Button<'a> {
    Button::new(egui::RichText::new(format!("{icon} {text}")))
}
