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
        crate::theme::set_theme(&cc.egui_ctx);

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

    fn previous_state(&mut self, ui: &mut egui::Ui) {
        match self.stage {
            Stage::SelectGame => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
            Stage::ConfigurePlayer => self.stage = Stage::SelectGame,
            Stage::Install => self.stage = Stage::ConfigurePlayer,
            Stage::Finished => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }

    fn exit(&self, ui: &mut egui::Ui) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui) {
        match self.stage {
            Stage::SelectGame => self.show_select_game_central_panel(ui),
            Stage::ConfigurePlayer => self.show_configure_player_central_panel(ui),
            Stage::Install => self.show_install_central_panel(ui),
            Stage::Finished => self.show_finished_central_panel(ui),
        };
    }

    fn show_select_game_central_panel(&mut self, ui: &mut egui::Ui) {
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
    }

    fn show_configure_player_central_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Select player");
    }

    fn show_install_central_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Ready to install");
    }

    fn show_finished_central_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Finished");
    }

    fn show_bottom_panel(&mut self, ui: &mut egui::Ui) {
        let exit_button_spec = match self.stage {
            Stage::Finished => components::NavExit::No,
            Stage::Install => components::NavExit::Cancel, // TODO use Stage::Installing
            _ => components::NavExit::Exit,
        };
        let back_button_spec = match self.stage {
            // TODO use Stage::Installing
            Stage::Finished | Stage::Install | Stage::SelectGame => components::NavBack::No,
            _ => components::NavBack::Back,
        };

        match components::navbar(
            ui,
            self.next_button_spec(),
            back_button_spec,
            exit_button_spec,
        )
        .clicked()
        {
            components::NavClicked::Next => self.next_state(ui),
            components::NavClicked::Back => self.previous_state(ui),
            components::NavClicked::Exit => self.exit(ui),
            components::NavClicked::None => {}
        }
    }

    fn next_button_spec(&self) -> components::NavNext {
        match self.stage {
            Stage::SelectGame => components::NavNext::Next(
                self.select_game_state.game_config.is_some()
                    && !self.select_game_state.install_location.is_empty(),
            ),
            Stage::ConfigurePlayer => components::NavNext::Next(true),
            Stage::Install => components::NavNext::Install,
            Stage::Finished => components::NavNext::Finish,
        }
    }
}

impl eframe::App for GameInstallApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        egui::TopBottomPanel::bottom("nav_buttons").show(ctx, |ui| self.show_bottom_panel(ui));
        egui::CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
    }
}
