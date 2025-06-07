use eframe::egui;

pub mod info;
pub mod testwindow;
pub mod sqlitedata;

use crate::app::AppSettings;
pub trait View {
    fn title(&self) -> String;
    fn show(&mut self, ctx: &egui::Context, id: egui::Id, open: &mut bool, settings: &AppSettings);
    fn ui(&mut self, ui: &mut egui::Ui);
}
