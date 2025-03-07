use eframe::egui::{self, Button};

pub fn next_button<'a>() -> Button<'a> {
    Button::new(egui::RichText::new(format!(
        "{} Next",
        egui_phosphor::regular::CARET_RIGHT
    )))
}

pub fn back_button<'a>() -> Button<'a> {
    Button::new(egui::RichText::new(format!(
        "{} Back",
        egui_phosphor::regular::CARET_LEFT
    )))
}

pub fn exit_button<'a>() -> Button<'a> {
    Button::new(egui::RichText::new(format!(
        "{} Exit",
        egui_phosphor::regular::PROHIBIT_INSET
    )))
}
