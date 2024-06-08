use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use eframe::egui;
use egui_extras::{Column, Table, TableBuilder};
use crate::file_manager;
use crate::file_manager::{check_dir_exists, evaluate_path_vars, FileInfo};
use chrono::offset::Utc;
use chrono::{DateTime, Local};
use eframe::egui::TextStyle;

pub struct FileNewerGui {
    active_path: PathBuf,
    user_facing_path: String,
    error_message: Option<String>,
    files_in_cur_path: Vec<FileInfo>,
}

impl Default for FileNewerGui {
    fn default() -> Self {
        let path = PathBuf::from(Path::new(&env::var("USERPROFILE").unwrap()));
        Self {
            files_in_cur_path: file_manager::get_files_in_dir(&path).expect("REASON"),
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
            .show(ctx,|ui| { self.build_side_panel_left(ui)});

        egui::SidePanel::right("File_Utils")
            .resizable(true)
            .default_width(250.0)
            .max_width(max_side_panel_width)
            .show(ctx, |ui| self.build_side_panel_right(ui));

        egui::TopBottomPanel::top("File_Path")
            .show(ctx, |ui|{
                ui.horizontal(|ui| self.build_top_panel(ui) );
            });

        egui::CentralPanel::default().show(ctx, |ui| self.build_main_frame(ui, ctx));

        ctx.request_repaint();
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
        });
    }

    fn build_side_panel_right(&mut self, ui: &mut egui::Ui) {
        ui.heading("Utils");
        ui.vertical_centered(|ui| {
            ui.heading("Right Panel");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {

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

    fn update_working_dir(&mut self, ui: &mut egui::Ui) {
        let path = match evaluate_path_vars(&self.user_facing_path) {
            Ok(path) => path,
            Err(e) => {
                self.error_message = Some(format!("{}", e));
                return;
            }
        };

        if !check_dir_exists(&path) {
            self.error_message = Some(format!("Cannot open folder, as cannot find {}", path));
            return;
        }

        match file_manager::get_files_in_dir(&path) {
            Ok(files) => {
                self.user_facing_path = path.clone();
                self.active_path = PathBuf::from(path);
                self.files_in_cur_path = files;
            },
            Err(e) => {
                self.error_message = Some(format!("Cannot read contents of folder as {}", e));
            }
        }
    }

    fn build_files_table(&mut self, ui: &mut egui::Ui, ctx: &egui::Context){
        let height_available = ui.available_height();
        let mut width_available = ui.available_width();
        let mut table = TableBuilder::new(ui)
            .resizable(true)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .min_scrolled_height(0.0)
            .max_scroll_height(height_available);

        // TODO naturally adapt this
        //let extension_col_len = estimate_str_len_as_label("EXTN", ctx);
        let mut file_ext_col = Column::auto();//.at_most(extension_col_len);
        //width_available -= extension_col_len;

        // this can be a hardcoded string because its always formated like this
        //let timestamp_col_len = estimate_str_len_as_label("XXXX-XX-XX XX:XX:XX", ctx);
        let create_date_col = Column::auto();//.at_most(timestamp_col_len);
        let edit_date_col = Column::auto();//.at_most(timestamp_col_len);
        //width_available -= timestamp_col_len;
        //width_available -= timestamp_col_len;


        table = table.column(Column::exact(10.0))// File Icon
            .column(Column::auto())
                //.at_most(width_available))// File Name
            .column(file_ext_col)// File type
            .column(create_date_col)// File creation date
            .column(edit_date_col);// File last edit date

        table = table.sense(egui::Sense::click());

        table
            .header(20.0 ,|mut header|{
                header.col(|ui| { ui.strong(""); }); // Icon
                header.col(|ui| { ui.strong("File Name"); });
                header.col(|ui| { ui.strong("File Type"); });
                header.col(|ui| { ui.strong("Creation Date"); });
                header.col(|ui| { ui.strong("Modified Date"); });
            })
            .body(|mut body| {
                for file in self.files_in_cur_path.iter(){
                    const ROW_HEIGHT:f32 = 18.0;
                    body.row(ROW_HEIGHT, |mut row| {
                        //TODO check how selctiong works row.set_selected(self.selection.contains(&row_index));
                        row.col(|ui|{ui.label(match (file.is_dir, file.can_be_written, file.is_link){
                            (true, false, false) => {"D".to_string()}
                            (true, true, false)  => {"d".to_string()}
                            (false, false, true)  => {"L".to_string()}
                            (false, true, true)   => {"l".to_string()}
                            (false, false, false)=> {"F".to_string()}
                            (false, true, false) => {"f".to_string()}
                            _ => {"?".to_string()}
                        });});
                        row.col(|ui|{ui.label(match file.file_name.to_str() {
                            Some(s) => s.to_string(),
                            None => "!!ERROR!!".to_string(),
                        });});
                        row.col(|ui|{ui.label(file.file_ext.as_ref().unwrap_or(&"".to_string()));});
                        row.col(|ui|{ui.label(
                            file.creation_time
                                .map(|t|
                                    DateTime::<Local>::from(t)
                                        .format("%F %T")
                                        .to_string())
                                .unwrap_or_else(|| "XXXX-XX-XX XX:XX:XX".to_string())
                        );});
                        row.col(|ui|{ui.label(
                            file.last_modification
                                .map(|t|
                                    DateTime::<Local>::from(t)
                                        .format("%F %T")
                                        .to_string())
                                .unwrap_or_else(|| "XXXX-XX-XX XX:XX:XX".to_string())
                        );});
                    });
                }
            })

    }
    fn build_main_frame(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        //ui.set_min_width(300.0); // Set the minimum width to 300.0
        //self.build_files_table(ui, ctx);

        let body_text_size = TextStyle::Body.resolve(ui.style()).size;
        use egui_extras::{Size, StripBuilder};
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .size(Size::exact(body_text_size)) // for the source code link
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.build_files_table(ui, ctx);
                    });
                });
            });
    }
}
