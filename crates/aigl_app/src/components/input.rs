use eframe::egui::{self, InnerResponse, Response};

use crate::components;
use aigl_project::config::project::{BotArg, BotArgValue};

pub fn text_input(ui: &mut egui::Ui, label: &str, text: &mut String) -> InnerResponse<Response> {
    ui.horizontal(|ui| {
        let label_response = ui.label(label);
        ui.text_edit_singleline(text).labelled_by(label_response.id)
    })
}

pub fn color_input(ui: &mut egui::Ui, label: &str, color: &mut [u8; 4]) -> InnerResponse<Response> {
    ui.horizontal(|ui| {
        let label_response = ui.label(label);
        ui.color_edit_button_srgba_unmultiplied(color)
            .labelled_by(label_response.id)
    })
}

pub fn path_input(ui: &mut egui::Ui, label: &str, path: &mut String) -> InnerResponse<Response> {
    ui.horizontal(|ui| {
        let label_response = ui.label(label);
        let edit = ui.text_edit_singleline(path).labelled_by(label_response.id);
        if ui.button("Browse").clicked() {
            if let Some(selected) = rfd::FileDialog::new().pick_folder() {
                *path = selected.display().to_string();
            }
        }
        edit
    })
}

pub fn bot_arg_input(ui: &mut egui::Ui, arg: &mut BotArg) -> InnerResponse<Response> {
    match &mut arg.value {
        BotArgValue::String(value) => text_input(ui, &arg.display, value),
        BotArgValue::Color(value) => color_input(ui, &arg.display, value),
        BotArgValue::Path(value) => path_input(ui, &arg.display, value),
    }
}

pub fn button_input(
    ui: &mut egui::Ui,
    text: &mut String,
    button_label: &str,
    button_icon: &str,
) -> InnerResponse<ButtonInputResponse> {
    ui.horizontal(|ui| {
        let input = ui.text_edit_singleline(text);
        let button = ui.add_enabled(
            !text.is_empty(),
            components::icon_button(button_label, button_icon),
        );
        ButtonInputResponse {
            // lost_focus happens when enter is pressed
            accepted: input.lost_focus() || button.clicked(),
        }
    })
}

#[derive(Clone, Debug)]
pub struct ButtonInputResponse {
    pub accepted: bool,
}
