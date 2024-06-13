// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod gui;
mod file_manager;
mod tests;
mod file_ordering;

use crate::gui::FileNewerGui;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0,700.0])
            .with_min_inner_size([1000.0,700.0]),
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
