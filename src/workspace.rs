use crate::views::*;
use eframe::egui;
use crate::app::AppSettings;
use eframe::egui::containers::panel::Side;
use eframe::egui::{Color32, Id, RichText};

#[derive(PartialEq, Default, Clone, Copy)]
enum ConfirmDeleteState {
    #[default]
    Idle,
    Pending,
}

#[derive(PartialEq, Clone, Copy)]
pub enum WorkspaceAction {
    None,
    AddWorkspace,
}

pub struct Workspace {
    pub name: String,
    confirm_delete_state: ConfirmDeleteState,
    info: Option<info::Info>,
    views: Vec<Box<dyn View>>,
}

impl Workspace {
    pub fn ui(&mut self, parent_id: Id, open: &mut bool, ctx: &egui::Context, settings: &AppSettings) -> WorkspaceAction {
        let mut action_to_take = WorkspaceAction::None;
        egui::SidePanel::new(Side::Left, parent_id.with("workspace_right_panel"))
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(5.);

                ui.label(RichText::new("Current workspace").strong());
                ui.heading(RichText::new(&self.name).strong()); // Çalışma alanı adını kullan
                ui.separator();
                ui.label(RichText::new("Menu").strong());

                if ui
                    .selectable_label(self.info.is_some(), info::WINDOW_TITLE)
                    .clicked()
                {
                    self.info = match self.info {
                        Some(_) => None,
                        None => Some(info::Info::default()),
                    };
                }

                if ui.button(sqlitedata::WINDOW_TITLE).clicked() {
                    let mut sqlite_window_exists = false;
                    for view in self.views.iter() {
                        if view.title() == sqlitedata::WINDOW_TITLE {
                            sqlite_window_exists = true;
                            break;
                        }
                    }
                    if !sqlite_window_exists {
                        self.views.push(Box::new(sqlitedata::SqliteData::default()));
                    }
                }
              

                if ui.button(testwindow::WINDOW_TITLE).clicked() {
                    let mut test_window_exists = false;
                    for view in self.views.iter() {
                        if view.title() == testwindow::WINDOW_TITLE {
                            test_window_exists = true;
                            break;
                        }
                    }
                    if !test_window_exists {
                        self.views.push(Box::new(testwindow::TestWindow::default()));
                    }
                }

                ui.separator();
                ui.label(RichText::new("Workspace").strong());

                let (button_text, fill_color) = match self.confirm_delete_state {
                    ConfirmDeleteState::Idle => ("🗑 Delete workspace", None),
                    ConfirmDeleteState::Pending => ("🗑 Are you sure?", Some(Color32::LIGHT_RED)),
                };

                let mut delete_button = egui::Button::new(button_text);
                if let Some(color) = fill_color {
                    delete_button = delete_button.fill(color);
                }

                if ui.add(delete_button).clicked() {
                    match self.confirm_delete_state {
                        ConfirmDeleteState::Idle => {
                            self.confirm_delete_state = ConfirmDeleteState::Pending;
                        }
                        ConfirmDeleteState::Pending => {
                            *open = false;
                            // Workspace silineceği için durumu sıfırlamaya gerek yok,
                            // eğer silinmezse reset_confirm_delete çağrılacak.
                        }
                    }
                }

                if ui.button("➕ Add workspace").clicked() {
                    action_to_take = WorkspaceAction::AddWorkspace;
                }

                ui.separator();
                ui.label(RichText::new("Windows").strong());

                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| {
                        mem.reset_areas();
                    });
                    ui.close_menu();
                }

                if ui.button("Close all windows").clicked() {
                    self.info = None;
                    self.views.clear();
                }
            });

        let mut to_delete = Vec::new();
        egui::CentralPanel::default().show(ctx, |_ui| {
            let mut open = true;

            if let Some(info) = self.info.as_mut() {
                info.show(ctx, parent_id.with("info"), &mut open, settings);
                if open == false {
                    self.info = None;
                }
            }

            for (i, view) in self.views.iter_mut().enumerate() {
                let mut open = true;
                view.show(ctx, parent_id.with(i), &mut open, settings);
                if open == false {
                    to_delete.push(i);
                }
            }
        });

        let mut removed = 0;
        for i in to_delete {
            self.views.remove(i - removed);
            removed += 1;
        }

        action_to_take
    }

    pub fn reset_confirm_delete(&mut self) {
        self.confirm_delete_state = ConfirmDeleteState::Idle;
    }

    pub fn new_with_name(name: String) -> Self {
        Self {
            name,
            confirm_delete_state: ConfirmDeleteState::Idle,
            // Yeni oluşturulan çalışma alanlarında varsayılan olarak Info penceresi
            // açık olmayabilir, isteğe bağlı olarak None veya Some(info::Info::default())
            // ayarlanabilir. Şimdilik None olarak bırakıyorum.
            info: None,
            views: Default::default(),
        }
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            name: "Workspace".to_string(),
            confirm_delete_state: ConfirmDeleteState::Idle,
            // Varsayılan olarak Info penceresi açık olsun mu?
            // new_with_name ile tutarlı olması için None veya Some olarak ayarlanabilir.
            info: None, // Veya Some(info::Info::default())
            views: Default::default(),
        }
    }
}
