use eframe::egui;
use eframe::egui::RichText;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::components;
use crate::game_config::fetch_game_config;
use crate::install::{InstallThreadData, install};
use crate::theme::Theme;
use aigl_project::{BotArg, config::game::GameConfig, dir_is_incomplete};
use aigl_system::fs::path_available_as_output_directory;

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
    error: Option<String>,
}

#[derive(Debug, Default)]
struct ConfigurePlayerState {
    id: String,
    name: String,
    args: Vec<BotArg>,
    custom_id: bool,
}

#[derive(Debug, Default)]
struct SelectLocationState {
    install_location: String,
    error: Option<String>,
}

#[derive(Debug, Default)]
struct InstallState {
    thread: Option<std::thread::JoinHandle<()>>,
    thread_data: Arc<RwLock<InstallThreadData>>,
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
        Theme::get_selected().apply(&cc.egui_ctx);

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
            Screen::Overview => {
                self.start_installation();
                self.screen = Screen::Installing
            }
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
            Screen::Installing => {
                self.cancel_installation();
                self.screen = Screen::Overview;
            }
            Screen::Finished => self.exit(ui),
        }
    }

    fn exit(&mut self, ui: &mut egui::Ui) {
        match self.screen {
            Screen::Installing => {
                self.cancel_installation();
                self.previous_screen(ui);
            }
            _ => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }

    fn cancel_installation(&mut self) {
        if matches!(self.screen, Screen::Installing) {
            if let Some(thread) = self.install_state.thread.take() {
                let _ = thread.join();
            }
            let path = PathBuf::from(&self.select_location_state.install_location);
            if dir_is_incomplete(&path) {
                let _ = std::fs::remove_dir_all(path);
            }
            self.install_state.thread_data.write().unwrap().error = None;
        }
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

        ui.label("Enter game code:");
        ui.horizontal(|ui| {
            let input = components::button_input(
                ui,
                &mut state.game_code,
                "Fetch",
                egui_phosphor::regular::DOWNLOAD_SIMPLE,
            );
            if input.inner.accepted {
                match fetch_game_config(&state.game_code) {
                    Ok(config) => {
                        self.configure_player_state.args = config
                            .bot
                            .template_args
                            .values()
                            .map(|arg| BotArg::default_from_template_arg(arg.clone()))
                            .collect();
                        self.game_config = Some(config);
                        state.error = None;
                    }
                    Err(err) => {
                        self.game_config = None;
                        state.error = Some(err.to_string());
                    }
                }
            }
        });

        if let Some(error) = &state.error {
            ui.colored_label(ui.visuals().error_fg_color, format!("Error: {error}"));
        } else if let Some(game_config) = &self.game_config {
            components::game_info_text(ui, game_config);
        }
        ui.add_space(100.0);
        let theme = Theme::get_selected();
        ui.label(
            RichText::from("Your game master will provide the game code.")
                .color(theme.base.fg_low_contrast.0),
        );
    }

    fn show_configure_player_central_panel(&mut self, ui: &mut egui::Ui) {
        let state = &mut self.configure_player_state;

        ui.label("Give your bot a unique name:");
        let name_response = components::text_input(ui, "Name", &mut state.name);
        if name_response.inner.changed() && !state.custom_id {
            state.id = default_bot_id(&state.name);
        }

        ui.label("Name your Python package (optional):");
        let id_response = components::text_input(ui, "Package", &mut state.id);
        if id_response.inner.changed() {
            state.custom_id = true;
        }

        for arg in state.args.iter_mut() {
            components::bot_arg_input(ui, arg);
        }
    }

    fn show_select_location_central_panel(&mut self, ui: &mut egui::Ui) {
        let state = &mut self.select_location_state;
        let path_label = ui.label("Select a folder to install into:");
        let response = ui.horizontal(|ui| {
            let edit_response = ui
                .text_edit_singleline(&mut state.install_location)
                .labelled_by(path_label.id);
            if ui.button("Browse").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    state.install_location = path.display().to_string();
                }
            }
            edit_response
        });

        if response.inner.changed() {
            match path_available_as_output_directory(&PathBuf::from(&state.install_location)) {
                Ok(_) => {
                    state.error = None;
                }
                Err(err) => {
                    state.error = Some(err.to_string());
                }
            }
        }
        if let Some(error) = &state.error {
            ui.colored_label(ui.visuals().error_fg_color, error);
        }
    }

    fn show_overview_central_panel(&mut self, ui: &mut egui::Ui) {
        ui.label(format!(
            "About to install the game into {}",
            self.select_location_state.install_location
        ));
    }

    fn show_installing_central_panel(&mut self, ui: &mut egui::Ui) {
        ui.label(format!(
            "Installing into {}",
            self.select_location_state.install_location
        ));
        if let Ok(data) = self.install_state.thread_data.read() {
            if let Some(error) = &data.error {
                ui.colored_label(ui.visuals().error_fg_color, format!("Error: {error}"));
            }
        }

        let Some(thread) = &self.install_state.thread else {
            ui.colored_label(ui.visuals().error_fg_color, "Install thread missing");
            return;
        };
        if thread.is_finished() {
            if self
                .install_state
                .thread_data
                .read()
                .is_ok_and(|data| data.error.is_none())
            {
                self.next_screen(ui);
            }
        } else {
            ui.spinner();
        }
    }

    fn show_finished_central_panel(&mut self, ui: &mut egui::Ui) {
        let theme = Theme::get_selected();

        ui.label(format!(
            "Finished installing {} into {}",
            self.game_config.as_ref().unwrap().name,
            self.select_location_state.install_location
        ));

        ui.add_space(10.0);
        ui.colored_label(theme.highlight.fg_high_contrast.0, "Running the game");
        ui.label("1. Open a terminal in the game folder");
        ui.label("2. Activate the virtual environment:");
        #[cfg(target_os = "windows")]
        ui.code(".venv\\Scripts\\activate");
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        ui.code("source .venv/bin/activate");
        ui.label("3. Run the game using:");
        ui.code(format!(
            "{} config.toml",
            self.game_config.as_ref().unwrap().name.to_lowercase()
        ));
        ui.label("4. Edit your AI");

        ui.add_space(10.0);
        ui.colored_label(theme.highlight.fg_high_contrast.0, "Uninstalling");
        ui.label("Simply remove the game folder at");
        ui.label(&self.select_location_state.install_location);
    }

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
            Screen::ConfigurePlayer => {
                components::NavNext::Next(!self.configure_player_state.name.is_empty())
            }
            Screen::SelectLocation => {
                let state = &self.select_location_state;
                components::NavNext::Next(
                    !state.install_location.is_empty() && state.error.is_none(),
                )
            }
            Screen::Overview => components::NavNext::Install(true),
            Screen::Installing => components::NavNext::Install(false),
            Screen::Finished => components::NavNext::Finish,
        }
    }

    fn start_installation(&mut self) {
        let data = self.install_state.thread_data.clone();
        let target_path = PathBuf::from(&self.select_location_state.install_location);
        let config = self.game_config.as_ref().unwrap().clone();
        let player_bot_id = self.configure_player_state.id.clone();
        let player_bot_name = self.configure_player_state.name.clone();
        let player_bot_args = self.configure_player_state.args.clone();

        // Safety: This is single threaded code.
        unsafe {
            aigl_project::config::init_environment(&target_path);
        }

        self.install_state.thread = Some(std::thread::spawn(move || {
            let _ = install(
                data,
                target_path,
                config,
                player_bot_id,
                player_bot_name,
                player_bot_args,
            );
        }));
    }
}

impl eframe::App for GameInstallApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.5);
        egui::TopBottomPanel::top("heading").show(ctx, |ui| self.show_top_panel(ui));
        egui::TopBottomPanel::bottom("nav_buttons").show(ctx, |ui| self.show_bottom_panel(ui));
        egui::CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
    }
}

fn default_bot_id(name: &str) -> String {
    format!("bot_{}", name.to_lowercase())
}
