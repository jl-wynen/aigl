use super::icon_button;
use super::widget::add_enabled_with_colors;
use eframe::egui;
use game_installer_theme::Theme;

pub fn navbar(ui: &mut egui::Ui, next: NavNext, back: NavBack, exit: NavExit) -> NavResponse {
    let mut clicked = NavClicked::None;
    let layout = egui::Layout::left_to_right(egui::Align::Center);
    let inner_layout = egui::Layout::right_to_left(egui::Align::Center);

    let container_response = ui.with_layout(layout, |ui| {
        let exit_response = add_exit_button(ui, exit);
        if exit_response.as_ref().is_some_and(|r| r.clicked()) {
            clicked = NavClicked::Exit;
        }

        let mut response = ui
            .with_layout(inner_layout, |ui| {
                let next_response = add_next_button(ui, next);
                let back_response = add_back_button(ui, back);
                if next_response.clicked() {
                    clicked = NavClicked::Next;
                }
                if back_response.as_ref().is_some_and(|r| r.clicked()) {
                    clicked = NavClicked::Back;
                }

                let mut response = next_response;
                if let Some(r) = back_response {
                    response = response.union(r);
                }
                response
            })
            .inner;

        if let Some(r) = exit_response {
            response = response.union(r);
        }
        response
    });

    NavResponse {
        clicked,
        inner: container_response.inner,
        response: container_response.response,
    }
}

#[derive(Copy, Clone)]
pub enum NavNext {
    Next(bool),
    Install(bool),
    Finish,
}

#[derive(Copy, Clone)]
pub enum NavExit {
    Exit,
    Cancel,
    No,
}

#[derive(Copy, Clone)]
pub enum NavBack {
    Back,
    No,
}

#[derive(Clone, Debug)]
pub struct NavResponse {
    clicked: NavClicked,
    pub inner: egui::Response,
    pub response: egui::Response,
}

impl NavResponse {
    pub fn clicked(&self) -> NavClicked {
        self.clicked
    }
}

#[derive(Copy, Clone, Debug)]
pub enum NavClicked {
    Next,
    Back,
    Exit,
    None,
}

fn add_next_button(ui: &mut egui::Ui, spec: NavNext) -> egui::Response {
    let theme = Theme::get_selected();
    match spec {
        NavNext::Next(enabled) => add_enabled_with_colors(
            ui,
            enabled,
            theme.highlight_widget_visuals(),
            icon_button("Next", egui_phosphor::regular::CARET_RIGHT),
        ),
        NavNext::Install(enabled) => add_enabled_with_colors(
            ui,
            enabled,
            theme.highlight_widget_visuals(),
            icon_button("Install", egui_phosphor::regular::CHECK),
        ),
        NavNext::Finish => add_enabled_with_colors(
            ui,
            true,
            theme.highlight_widget_visuals(),
            icon_button("Finish", egui_phosphor::regular::SIGN_OUT),
        ),
    }
}

fn add_back_button(ui: &mut egui::Ui, spec: NavBack) -> Option<egui::Response> {
    match spec {
        NavBack::Back => Some(ui.add(icon_button("Back", egui_phosphor::regular::CARET_LEFT))),
        NavBack::No => None,
    }
}

fn add_exit_button(ui: &mut egui::Ui, spec: NavExit) -> Option<egui::Response> {
    match spec {
        NavExit::Exit => Some(ui.add(icon_button("Exit", egui_phosphor::regular::SIGN_OUT))),
        NavExit::Cancel => Some(ui.add(icon_button(
            "Cancel",
            egui_phosphor::regular::PROHIBIT_INSET,
        ))),
        NavExit::No => None,
    }
}
