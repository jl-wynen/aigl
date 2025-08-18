use eframe::egui::{self, InnerResponse, Response};

pub fn text_field<'input>(
    ui: &mut egui::Ui,
    label: &str,
    text: &'input mut String,
) -> InnerResponse<Response> {
    ui.horizontal(|ui| {
        let label_response = ui.label(label);
        ui.text_edit_singleline(text).labelled_by(label_response.id)
    })
}
