use eframe::egui::{self};

pub enum GameInstallApp {
    SelectGame(SelectGameState),
    ConfigurePlayer(ConfigurePlayerState),
    Install(InstallState),
}

#[derive(Debug, Default)]
struct SelectGameState {}

#[derive(Debug)]
struct ConfigurePlayerState {}

#[derive(Debug)]
struct InstallState {}

#[derive(Debug)]
enum Action {
    Remain,
    NextState,
}

impl GameInstallApp {
    pub fn run() {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            centered: true,
            ..Default::default()
        };
        eframe::run_native(
            "Install AI Game",
            options,
            Box::new(|cc| Ok(Box::new(Self::new(cc)))),
        )
        .unwrap();
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cc.egui_ctx.style_mut(set_style);
        Self::SelectGame(SelectGameState::default())
    }

    fn next_state(&mut self, ui: &mut egui::Ui) {
        match self {
            GameInstallApp::SelectGame(state) => {
                *self = GameInstallApp::ConfigurePlayer(ConfigurePlayerState {})
            }
            GameInstallApp::ConfigurePlayer(state) => {
                *self = GameInstallApp::Install(InstallState {})
            }
            GameInstallApp::Install(state) => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close)
            }
        }
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui) {
        let action = match self {
            GameInstallApp::SelectGame(state) => show_select_game_central_panel(ui, state),
            GameInstallApp::ConfigurePlayer(state) => {
                show_configure_player_central_panel(ui, state)
            }
            GameInstallApp::Install(state) => show_install_central_panel(ui, state),
        };
        match action {
            Action::Remain => {}
            Action::NextState => self.next_state(ui),
        }
    }
}

fn show_select_game_central_panel(ui: &mut egui::Ui, state: &mut SelectGameState) -> Action {
    ui.heading("Select Game");
    if ui.button("next").clicked() {
        Action::NextState
    } else {
        Action::Remain
    }
}

fn show_configure_player_central_panel(
    ui: &mut egui::Ui,
    state: &mut ConfigurePlayerState,
) -> Action {
    ui.heading("Select player");
    if ui.button("install").clicked() {
        Action::NextState
    } else {
        Action::Remain
    }
}

fn show_install_central_panel(ui: &mut egui::Ui, state: &mut InstallState) -> Action {
    ui.heading("Install");
    if ui.button("finish").clicked() {
        Action::NextState
    } else {
        Action::Remain
    }
}

impl eframe::App for GameInstallApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
    }
}
