#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

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
    name: String,
    age: u32,
}

impl Default for FileNewerGui {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
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

    }
}

impl FileNewerGui{
    fn build_side_panel_left(&mut self, ui:&mut egui::Ui){
        ui.vertical_centered(|ui| {
            ui.heading("Left Panel");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            Self::lorem_ipsum(ui);
        });
    }

    fn build_side_panel_right(&mut self, ui:&mut egui::Ui){
        ui.heading("Utils");
        ui.vertical_centered(|ui| {
            ui.heading("Left Panel");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            Self::lorem_ipsum(ui);
        });
    }

    fn build_top_panel(&mut self, ui:&mut egui::Ui) {
        let name_label = ui.label("Tmp");
        ui.text_edit_singleline(&mut self.name)
            .labelled_by(name_label.id);
    }

    fn build_main_frame(&mut self, ui:&mut egui::Ui) {
        ui.set_min_width(300.0); // Set the minimum width to 300.0
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut self.name)
                .labelled_by(name_label.id);
        });
        ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        if ui.button("Increment").clicked() {
            self.age += 1;
        }
        ui.label(format!("Hello '{}', age {}", self.name, self.age));
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

