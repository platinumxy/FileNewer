use std::path::PathBuf;
use eframe::egui;

use egui_extras::{Column, TableBuilder};
use crate::file_manager::{check_dir_exists, evaluate_path_vars, FileInfo, get_files_in_dir};
use crate::file_ordering::{DisplayOptions, SortBy};

// CONSTS
const MIN_CENTRAL_PANEL_WIDTH:f32 = 600.0;
const DEFAULT_SIDE_BAR_WIDTH:f32 = 150.0;

pub struct FileNewerGui {
    user_facing_path: String,
    error_message: Option<String>,
    files_in_cur_path: Vec<FileInfo>,
    selected_file: Option<usize>,
    display_options: DisplayOptions,
    update_files_this_loop:bool,
}

impl Default for FileNewerGui {
    fn default() -> Self {
        // shouldn't fail
        let pth = evaluate_path_vars("%USERPROFILE%").unwrap();
        Self {
            files_in_cur_path: get_files_in_dir(&pth, &false).expect("REASON"),
            user_facing_path: pth,
            error_message: None,
            selected_file: None,
            display_options: DisplayOptions::default(),
            update_files_this_loop:false
        }
    }
}

impl eframe::App for FileNewerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let max_side_panel_width =
            (ctx.available_rect().width() - MIN_CENTRAL_PANEL_WIDTH) / 2.0;
        self.update_files_this_loop = false;
        self.display_menu_bar(ctx);
        self.display_left_side_panel(ctx, &max_side_panel_width);
        self.display_right_side_panel(ctx, &max_side_panel_width);
        self.display_top_panel(ctx);
        self.display_main_panel(ctx);
        self.display_error_msg(ctx);
        ctx.request_repaint();

        if self.update_files_this_loop {
            self.selected_file = None;
            self.update_working_dir();
        }

    }
}

// DISPLAYS
impl FileNewerGui {
    fn display_error_msg(&mut self, ctx: &egui::Context){
        if let Some(error_message) = self.error_message.clone() {
            let mut open = true;
            egui::Window::new("Error")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error_message);
                    if ui.button("OK").clicked() || ui.input(|i| {
                        i.key_pressed(egui::Key::Escape)}) { self.error_message = None; }
                });
            if !open { self.error_message = None; }
        }
    }

    fn display_left_side_panel(&mut self, ctx: &egui::Context, max_width:&f32){
        egui::SidePanel::left("File_Tree")
            .resizable(true)
            .default_width(DEFAULT_SIDE_BAR_WIDTH)
            .max_width(*max_width)
            .show(ctx,|ui| {
                self.build_side_panel_left(ui)
            });
    }

    fn display_right_side_panel(&mut self, ctx: &egui::Context, max_width:&f32){
        egui::SidePanel::right("File_Utils")
            .resizable(true)
            .default_width(DEFAULT_SIDE_BAR_WIDTH)
            .max_width(*max_width)
            .show(ctx, |ui| {
                self.build_side_panel_right(ui)
            });
    }

    fn display_top_panel(&mut self, ctx: &egui::Context){
        egui::TopBottomPanel::top("File_Path")
            .show(ctx, |ui|{
                ui.horizontal(|ui| {
                    self.build_top_panel(ui)
                });
            });
    }

    fn display_main_panel(&mut self, ctx: &egui::Context){
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.build_main_frame(ui)
            });
    }
    fn display_menu_bar(&mut self, ctx: &egui::Context){
        egui::TopBottomPanel::top("menu_bar")
            .show(ctx, |ui|{
                self.build_menu(ui);
            });
    }
}

// BUILD ITEMS
impl FileNewerGui {
    fn build_menu(&mut self, ui: &mut egui::Ui){
        egui::menu::bar(ui, |ui|{
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    self.error_message = Some("Saving".to_string());
                }
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Settings", |ui|{
                ui.label("SHOW");
                if ui.checkbox(&mut self.display_options.show_hidden, "Hidden Files").changed(){
                    self.update_files_this_loop = true;
                };
                ui.checkbox(&mut self.display_options.show_file_ext, "File Extension");
                ui.checkbox(&mut self.display_options.show_file_size, "File Size");
                ui.checkbox(&mut self.display_options.show_creation, "Creation Time");
                ui.checkbox(&mut self.display_options.show_last_mod, "Last Modification Time");
                ui.checkbox(&mut self.display_options.show_last_acc, "Last Access Time");
                ui.checkbox(&mut self.display_options.show_file_type, "Show Type Letter");
                ui.separator();
            });
            ui.menu_button("Sort by", |ui| {
                if ui.checkbox(&mut self.display_options.filter_dec, "Sort Descending").changed(){
                    self.update_files_this_loop = true;
                };
                let sort_options = [
                    ("File Name", SortBy::Name),
                    ("File Type", SortBy::Type),
                    ("File Extension", SortBy::Ext),
                    ("Creation Date", SortBy::CreateDate),
                    ("Modification Date", SortBy::ModDate),
                    ("View Date", SortBy::ViewDate),
                    ("File Size", SortBy::Size),
                    ("Nan", SortBy::Nan),
                ];
                for (label, sort_by) in sort_options.iter() {
                    if ui.button(format!("{}{}", if self.display_options.sort_by == *sort_by {"*"} else {""}, label)).clicked() {
                        self.display_options.sort_by = sort_by.clone();
                        self.update_files_this_loop = true;
                    }
                }
            });
            ui.label(format!("Files in Current DIR {}", self.files_in_cur_path.len()))
        });
    }

    fn build_side_panel_left(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Left Panel");
        });
        egui::ScrollArea::vertical().show(ui, |_ui| {});
    }

    fn build_side_panel_right(&mut self, ui: &mut egui::Ui) {

        ui.vertical_centered(|ui| {
            ui.heading(
                if !(self.selected_file == None || self.update_files_this_loop){
                    &self.files_in_cur_path[self.selected_file.unwrap()].file_name.to_str().unwrap()}
                else {""});
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.selected_file == None || self.update_files_this_loop{
                ui.label("Type: -");
                ui.label("Has Write Perms: -");
                ui.label("File Name: -");
                ui.label("File Extension: -");

                ui.label("Last Read : -");
                ui.label("Last Write: -");
                ui.label("Created   : -");
            }
            else {
                let active = &self.files_in_cur_path[self.selected_file.unwrap()];
                ui.label(format!("Type: {}", active.type_to_basic_str()));
                ui.label(format!("Has Write Perms: {}", active.can_be_written));
                ui.label(format!("File Name: {}", active.file_name.to_str().unwrap()));
                ui.label(format!("File Extension: {}",
                    match &active.file_ext{ Some(ext) => {ext} None => {"-"}}));

                ui.label(format!("Last Write: {}", active.last_mod_formated()));
                ui.label(format!("Last Read : {}", active.last_access_formated()));
                ui.label(format!("Created   : {}", active.creation_time_formated()));
            }
        });
    }

    fn build_top_panel(&mut self, ui: &mut egui::Ui) {
        let path_label = ui.label("Active Path:");

        let new_path = ui.text_edit_singleline(&mut self.user_facing_path)
            .labelled_by(path_label.id);

        if new_path.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.update_working_dir();
        }
    }

    fn update_working_dir(&mut self) {
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

        match get_files_in_dir(&path, &self.display_options.show_hidden) {
            Ok(mut files) => {
                self.display_options.sort(&mut files);
                self.user_facing_path = path.clone();
                self.files_in_cur_path = files;
            },
            Err(e) => {
                self.error_message = Some(format!("Cannot read contents of folder as {}", e));
            }
        }
    }

    fn build_files_table(&mut self, ui: &mut egui::Ui) {
        let height_available = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .resizable(true)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .min_scrolled_height(0.0)
            .max_scroll_height(height_available);

        table = table.column(Column::exact(10.0)) // File Icon
            .column(Column::auto().clip(true).at_least(80.0)) // File Name
            .column(Column::auto().clip(true).at_least(40.0)) // File type
            .column(Column::auto().clip(true).at_least(100.0)) // File creation date
            .column(Column::auto().clip(true).at_least(100.0)) // File last edit date
            .column(Column::auto().clip(true).at_least(40.0)); //File Size

        table = table.sense(egui::Sense::click());

        table
            .header(20.0, |mut header| {
                header.col(|ui| { ui.strong(""); }); // Icon
                header.col(|ui| { ui.strong("File Name"); });
                header.col(|ui| { ui.strong("File Type"); });
                header.col(|ui| { ui.strong("Creation Date"); });
                header.col(|ui| { ui.strong("Modified Date"); });
                header.col(|ui| { ui.strong("File Size"); });
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, self.files_in_cur_path.len(), |mut row| {
                    let row_index = row.index();
                    let file = &self.files_in_cur_path[row_index];

                    row.col(|ui| {
                        ui.label(file.single_char_desc());
                    });
                    row.col(|ui| {
                        ui.label(match file.file_name.to_str() {
                            Some(s) => s.to_string(),
                            None => "!!ERROR!!".to_string(),
                        });
                    });
                    row.col(|ui| { ui.label(file.file_ext.as_ref().unwrap_or(&"".to_string())); });
                    row.col(|ui| {
                        ui.label(file.creation_time_formated());
                    });
                    row.col(|ui| {
                        ui.label(file.last_mod_formated());
                    });

                    row.col(|ui| {
                        ui.label(
                            if !file.is_dir(){format!("{}", file.file_size)}else {"-".to_owned()});
                    });

                    let rr = row.response();
                    if rr.clicked(){
                        if self.selected_file == Some(row_index) {
                            if file.is_dir() {
                                self.user_facing_path.push_str(file.file_name.to_str().unwrap());
                                self.user_facing_path.push_str(&*"\\");
                                self.update_files_this_loop = true;
                            }
                            else {
                                let mut pth = PathBuf::from(&self.user_facing_path);
                                pth.push(&file.file_name);
                                let _ = open::that_detached(pth);
                            }
                                //self.error_message = Some("Currently not supported".to_string());}
                        }
                        else{ self.selected_file = Some(row_index); }
                    }
                    row.set_selected(self.selected_file == Some(row_index));
                });
            });
    }

    fn build_main_frame(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            self.build_files_table(ui);
        });
    }
}