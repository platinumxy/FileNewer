#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::env;
use std::env::VarError;
use std::path::{Path, PathBuf};
use eframe::{App, egui};
use eframe::egui::TextBuffer;
use std::error::Error;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0,500.0])
            .with_min_inner_size([800.0,500.0]),
        follow_system_theme: true,
        persist_window: false,
        ..Default::default()
    };
    eframe::run_native(
        "File Newer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<FileNewerGui>::default()
        }),
    )
}

struct FileNewerGui {
    active_path: PathBuf,
    user_facing_path: String,
    error_message: Option<String>,
}

impl Default for FileNewerGui {
    fn default() -> Self {
        let path = PathBuf::from(Path::new(&env::var("APPDATA").unwrap()));
        Self {
            user_facing_path: path.to_str().unwrap().to_string(),
            active_path: path,
            error_message: None,
        }
    }
}

impl eframe::App for FileNewerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let min_central_panel_width = 500.0; // Set this to your preferred minimum width
        let max_side_panel_width = (ctx.available_rect().width() - min_central_panel_width) / 2.0;

        egui::SidePanel::left("File_Tree")
            .resizable(true)
            .default_width(250.0)
            .max_width(max_side_panel_width)
            .show(ctx,|ui| self.build_side_panel_left(ui));

        egui::SidePanel::right("File_Utils")
            .resizable(true)
            .default_width(250.0)
            .max_width(max_side_panel_width)
            .show(ctx, |ui| self.build_side_panel_right(ui));

        egui::TopBottomPanel::top("File_Path")
            .show(ctx, |ui|{
            ui.horizontal(|ui| self.build_top_panel(ui) );
        });

        egui::CentralPanel::default().show(ctx, |ui| self.build_main_frame(ui));

        ctx.request_repaint();

/*
        if let Some(ref error_message) = self.error_message {
            let mut open = true;
            let error_message = error_message.clone();
            egui::Window::new("Error")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .close_button(&mut open)
                .show(ctx, |ui| {
                    ui.label(&error_message);
                    if ui.button("OK").clicked() {
                        // Clear the error message
                        self.error_message = None;
                    }
                });
            if !open {
                // Clear the error message when the window is closed
                self.error_message = None;
            }
        }*/
        if let Some(error_message) = self.error_message.clone() {
            let mut open = true;
            egui::Window::new("Error")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error_message);
                    if ui.button("OK").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        // Clear the error message
                        self.error_message = None;
                    }
                });
            if !open { self.error_message = None; }
        }

    }
}

impl FileNewerGui {
    fn build_side_panel_left(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Left Panel");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            Self::lorem_ipsum(ui);
        });
    }

    fn build_side_panel_right(&mut self, ui: &mut egui::Ui) {
        ui.heading("Utils");
        ui.vertical_centered(|ui| {
            ui.heading("Left Panel");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            Self::lorem_ipsum(ui);
        });
    }

    fn build_top_panel(&mut self, ui: &mut egui::Ui) {
        let path_label = ui.label("Active Path:");

        let new_path = ui.text_edit_singleline(&mut self.user_facing_path)
            .labelled_by(path_label.id);

        if new_path.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.update_working_dir(ui);
        }
    }

    fn update_working_dir(&mut self, ui: &mut egui::Ui){
        match evaluate_path_vars(&self.user_facing_path) {
            Ok(path) => {
                if check_dir_exists(&path){
                    self.user_facing_path = path.clone();
                    self.active_path = PathBuf::from(path);
                }
                else{
                    self.error_message = Some(format!("Cannot open folder, as cannot find {}", path));
                    return;
                }
            }
            Err(e) => { self.error_message = Some(format!("{}", e)); return;}
        };
        // we will only hit this if the path we are loading is working

    }


    fn build_main_frame(&mut self, ui: &mut egui::Ui) {
        ui.set_min_width(300.0); // Set the minimum width to 300.0

    }

    fn lorem_ipsum(ui: &mut egui::Ui) {
        ui.with_layout(
            egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| {
                ui.label(egui::RichText::new(lorem_ipsum_generator::lorem_ipsum_generator(10)).small().weak());
                ui.add(egui::Separator::default().grow(8.0));
                ui.label(egui::RichText::new(lorem_ipsum_generator::lorem_ipsum_generator(10)).small().weak());
            },
        );
    }
}

fn evaluate_path_vars(user_facing_path: &str) -> Result<String, VarError> {
    let win_env_path:fn(&str) -> Result<String, VarError> = |path: &str| {
        let mut path_str = env::var(path)?;
        path_str.push_str("\\");
        Ok(path_str)
    };
    let mut new_path = user_facing_path.replace("/", "\\");
    if new_path.starts_with("~")  { new_path = new_path.replacen("~","%USERPROFILE%", 1) }
    if new_path.starts_with("\\") { new_path = new_path.replacen("\\","%SYSTEMDRIVE%",1) }

    let mut final_path = if new_path.starts_with('%') {
        let parts: Vec<&str> = new_path.split('%').collect();
        if parts.len() < 2 {
            String::from(new_path)
        } else {
            let mut path = win_env_path(parts[1])?;
            for part in &parts[2..] {
                path.push_str(part);
            }
            path
        }
    } else {
        String::from(new_path)
    };

    // Ensure the path ends with a /
    if !final_path.ends_with("\\") {
        final_path.push_str("\\");
    }

    Ok(final_path.to_string().replace("\\\\", "\\"))
}

fn check_dir_exists(path: &String) -> bool{
    Path::is_dir(path.as_ref())
}
