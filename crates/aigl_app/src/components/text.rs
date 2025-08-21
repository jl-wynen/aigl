use eframe::egui::{
    self, Response,
    text::{LayoutJob, TextFormat},
};

use crate::theme::Theme;
use aigl_project::config::game::GameConfig;

pub fn game_info_text(ui: &mut egui::Ui, config: &GameConfig) -> Response {
    let theme = Theme::get_selected();
    let mut job = LayoutJob::default();
    job.append("Game name: ", 0.0, TextFormat::default());
    job.append(
        &config.name,
        0.0,
        TextFormat {
            color: theme.highlight.fg_low_contrast.0,
            ..Default::default()
        },
    );
    ui.label(job)
}
