use eframe::egui::{self, Button};

pub fn icon_button<'a>(text: &str, icon: &str) -> Button<'a> {
    Button::new(egui::RichText::new(format!("{icon} {text}")))
}

pub fn next_button<'a>() -> Button<'a> {
    icon_button("Next", egui_phosphor::regular::CARET_RIGHT)
}

pub fn back_button<'a>() -> Button<'a> {
    icon_button("Back", egui_phosphor::regular::CARET_LEFT)
}

pub fn exit_button<'a>() -> Button<'a> {
    icon_button("Exit", egui_phosphor::regular::PROHIBIT_INSET)
}
