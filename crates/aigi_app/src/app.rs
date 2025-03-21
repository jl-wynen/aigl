use crate::components;
use crate::game_config::GameConfig;
use eframe::egui::{self};

pub struct GameInstallApp {
    screen: Screen,
    game_config: Option<GameConfig>,
    // Store all states at the same time so we have access to all data as needed.
    select_game_state: SelectGameState,
    configure_player_state: ConfigurePlayerState,
    select_location_state: SelectLocationState,
    install_state: InstallState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Screen {
    SelectGame,
    ConfigurePlayer,
    SelectLocation,
    Overview,
    Installing,
    Finished,
}

#[derive(Debug, Default)]
struct SelectGameState {
    game_code: String,
}

#[derive(Debug, Default)]
struct ConfigurePlayerState {}

#[derive(Debug, Default)]
struct SelectLocationState {
    install_location: String,
}

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
        crate::theme::Theme::get_selected().apply(&cc.egui_ctx);

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        Self {
            screen: Screen::SelectGame,
            game_config: None,
            select_game_state: Default::default(),
            configure_player_state: Default::default(),
            select_location_state: Default::default(),
            install_state: Default::default(),
        }
    }

    fn next_screen(&mut self, ui: &mut egui::Ui) {
        match self.screen {
            Screen::SelectGame => self.screen = Screen::ConfigurePlayer,
            Screen::ConfigurePlayer => self.screen = Screen::SelectLocation,
            Screen::SelectLocation => self.screen = Screen::Overview,
            Screen::Overview => self.screen = Screen::Installing,
            Screen::Installing => self.screen = Screen::Finished,
            Screen::Finished => self.exit(ui),
        }
    }

    fn previous_screen(&mut self, ui: &mut egui::Ui) {
        match self.screen {
            Screen::SelectGame => self.exit(ui),
            Screen::ConfigurePlayer => self.screen = Screen::SelectGame,
            Screen::SelectLocation => self.screen = Screen::ConfigurePlayer,
            Screen::Overview => self.screen = Screen::SelectLocation,
            Screen::Installing => self.exit(ui),
            Screen::Finished => self.exit(ui),
        }
    }

    fn exit(&self, ui: &mut egui::Ui) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui) {
        match self.screen {
            Screen::SelectGame => self.show_select_game_central_panel(ui),
            Screen::ConfigurePlayer => self.show_configure_player_central_panel(ui),
            Screen::SelectLocation => self.show_select_location_central_panel(ui),
            Screen::Overview => self.show_overview_central_panel(ui),
            Screen::Installing => self.show_installing_central_panel(ui),
            Screen::Finished => self.show_finished_central_panel(ui),
        };
    }

    fn show_select_game_central_panel(&mut self, ui: &mut egui::Ui) {
        let state = &mut self.select_game_state;

        let label = ui.label("Enter game code:");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut state.game_code)
                .labelled_by(label.id);
            if ui
                .add_enabled(
                    !state.game_code.is_empty(),
                    components::icon_button("Fetch", egui_phosphor::regular::DOWNLOAD_SIMPLE),
                )
                .clicked()
            {
                self.game_config = Some(GameConfig {
                    name: "Test game".to_owned(),
                });
            }
        });

        if let Some(game_config) = &self.game_config {
            ui.label(format!("Game name: {}", game_config.name));
        }
        ui.add_space(50.0);
    }

    fn show_configure_player_central_panel(&mut self, ui: &mut egui::Ui) {}

    fn show_select_location_central_panel(&mut self, ui: &mut egui::Ui) {
        let state = &mut self.select_location_state;
        let path_label = ui.label("Select a folder to install into:");
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

    fn show_overview_central_panel(&mut self, ui: &mut egui::Ui) {}

    fn show_installing_central_panel(&mut self, ui: &mut egui::Ui) {}

    fn show_finished_central_panel(&mut self, ui: &mut egui::Ui) {}

    fn show_top_panel(&mut self, ui: &mut egui::Ui) {
        let heading = match self.screen {
            Screen::SelectGame => "Select game",
            Screen::ConfigurePlayer => "Configure player",
            Screen::SelectLocation => "Install location",
            Screen::Overview => "Overview",
            Screen::Installing => "Installing",
            Screen::Finished => "Installation finished",
        };
        ui.heading(heading);
    }

    fn show_bottom_panel(&mut self, ui: &mut egui::Ui) {
        let exit_button_spec = match self.screen {
            Screen::Finished => components::NavExit::No,
            Screen::Installing => components::NavExit::Cancel,
            _ => components::NavExit::Exit,
        };
        let back_button_spec = match self.screen {
            Screen::Finished | Screen::Installing | Screen::SelectGame => components::NavBack::No,
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
            components::NavClicked::Next => self.next_screen(ui),
            components::NavClicked::Back => self.previous_screen(ui),
            components::NavClicked::Exit => self.exit(ui),
            components::NavClicked::None => {}
        }
    }

    fn next_button_spec(&self) -> components::NavNext {
        match self.screen {
            Screen::SelectGame => components::NavNext::Next(self.game_config.is_some()),
            Screen::ConfigurePlayer => components::NavNext::Next(true),
            Screen::SelectLocation => {
                components::NavNext::Next(!self.select_location_state.install_location.is_empty())
            }
            Screen::Overview => components::NavNext::Install(true),
            Screen::Installing => components::NavNext::Install(false),
            Screen::Finished => components::NavNext::Finish,
        }
    }
}

impl eframe::App for GameInstallApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        egui::TopBottomPanel::top("heading").show(ctx, |ui| self.show_top_panel(ui));
        egui::TopBottomPanel::bottom("nav_buttons").show(ctx, |ui| self.show_bottom_panel(ui));
        egui::CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
    }
}
