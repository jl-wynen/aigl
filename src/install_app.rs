use crate::components;
use crate::game_config::GameConfig;
use eframe::egui::{self};

pub struct GameInstallApp {
    stage: Stage,
    // Store all states at the same time so we have access to all data as needed.
    select_game_state: SelectGameState,
    configure_player_state: ConfigurePlayerState,
    // None until installation starts, then there is no going back.
    install_state: Option<InstallState>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    SelectGame,
    ConfigurePlayer,
    Install,
    Finished,
}

#[derive(Debug, Default)]
struct SelectGameState {
    game_code: String,
    game_config: Option<GameConfig>,
    install_location: String,
}

#[derive(Debug, Default)]
struct ConfigurePlayerState {}

#[derive(Debug, Default)]
struct InstallState {}

#[derive(Debug)]
enum Action {
    Remain,
    NextState,
    Cancel,
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
        cc.egui_ctx.style_mut(crate::theme::set_style);

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        Self {
            stage: Stage::SelectGame,
            select_game_state: Default::default(),
            configure_player_state: Default::default(),
            install_state: None,
        }
    }

    fn next_state(&mut self, ui: &mut egui::Ui) {
        match self.stage {
            Stage::SelectGame => self.stage = Stage::ConfigurePlayer,
            Stage::ConfigurePlayer => self.stage = Stage::Install,
            Stage::Install => self.stage = Stage::Finished,
            Stage::Finished => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui) {
        let action = match self.stage {
            Stage::SelectGame => self.show_select_game_central_panel(ui),
            Stage::ConfigurePlayer => self.show_configure_player_central_panel(ui),
            Stage::Install => self.show_install_central_panel(ui),
            Stage::Finished => self.show_finished_central_panel(ui),
        };
        match action {
            Action::Remain => {}
            Action::NextState => self.next_state(ui),
            Action::Cancel => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }

    fn show_select_game_central_panel(&mut self, ui: &mut egui::Ui) -> Action {
        let state = &mut self.select_game_state;

        ui.heading("Select Game");
        ui.horizontal(|ui| {
            let label = ui.label("Game code:");
            ui.text_edit_singleline(&mut state.game_code)
                .labelled_by(label.id);
        });
        if ui
            .add_enabled(!state.game_code.is_empty(), egui::Button::new("Fetch"))
            .clicked()
        {
            state.game_config = Some(GameConfig {
                name: "Test game".to_owned(),
            });
        }
        if let Some(game_config) = &state.game_config {
            ui.label(format!("Game name: {}", game_config.name));
        }
        ui.add_space(50.0);

        let path_label = ui.label("Install location:");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut state.install_location)
                .labelled_by(path_label.id);
            if ui.button("Browse").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    state.install_location = path.display().to_string();
                }
            }
        });

        let mut action = Action::Remain;
        ui.horizontal(|ui| {
            if ui.add(components::exit_button()).clicked() {
                action = Action::Cancel
            }

            let next_enabled = state.game_config.is_some() && !state.install_location.is_empty();
            if ui
                .add_enabled(next_enabled, components::next_button())
                .clicked()
            {
                action = Action::NextState
            }
        });
        action
    }

    fn show_configure_player_central_panel(&mut self, ui: &mut egui::Ui) -> Action {
        ui.heading("Select player");
        if ui.add(components::next_button()).clicked() {
            Action::NextState
        } else {
            Action::Remain
        }
    }

    fn show_install_central_panel(&mut self, ui: &mut egui::Ui) -> Action {
        ui.heading("Ready to install");
        if ui.button("Install").clicked() {
            Action::NextState
        } else {
            Action::Remain
        }
    }

    fn show_finished_central_panel(&mut self, ui: &mut egui::Ui) -> Action {
        ui.heading("Finished");
        if ui.button("Close").clicked() {
            Action::NextState
        } else {
            Action::Remain
        }
    }
}

impl eframe::App for GameInstallApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        egui::CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
    }
}
