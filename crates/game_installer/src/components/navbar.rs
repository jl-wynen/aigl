use super::icon_button;
use eframe::egui;

pub fn navbar(ui: &mut egui::Ui, next: NavNext, back: NavBack, exit: NavExit) -> NavResponse {
    let mut clicked = NavClicked::None;

    let container_response = ui.horizontal(|ui| {
        let exit_response = add_exit_button(ui, exit);
        let back_response = add_back_button(ui, back);
        let next_response = add_next_button(ui, next);

        if exit_response.as_ref().is_some_and(|r| r.clicked()) {
            clicked = NavClicked::Exit;
        }
        if back_response.as_ref().is_some_and(|r| r.clicked()) {
            clicked = NavClicked::Back;
        }
        if next_response.clicked() {
            clicked = NavClicked::Next;
        }

        let mut response = next_response;
        if let Some(r) = back_response {
            response = response.union(r);
        }
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
    Install,
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
    match spec {
        NavNext::Next(enabled) => ui.add_enabled(
            enabled,
            icon_button("Next", egui_phosphor::regular::CARET_RIGHT),
        ),
        NavNext::Install => ui.add(icon_button("Install", egui_phosphor::regular::CHECK)),
        NavNext::Finish => ui.add(icon_button("Finish", egui_phosphor::regular::SIGN_OUT)),
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
