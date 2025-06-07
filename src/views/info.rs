use eframe::egui;
use super::View;
use crate::app::AppSettings;

pub const WINDOW_TITLE: &str = "README";

#[derive(Default)]
pub struct Info;

impl View for Info {
    fn title(&self) -> String {
        WINDOW_TITLE.to_string()
    }

    fn show(&mut self, ctx: &egui::Context, id: egui::Id, open: &mut bool, settings: &AppSettings) {
        egui::Window::new(self.title())
            .id(id)
            .default_width(320.0)
            .frame(egui::Frame::window(&*ctx.style())
                .corner_radius(settings.global_rounding)
                //.fill(settings.window_background_fill)
            )
            .open(open)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.style_mut().spacing.interact_size.y = 0.0;
        ui.heading("Info");
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("This is a demo application. It is based on the Egui framework and is intended to be a simple example of how to use Egui. Visit the ");
            ui.hyperlink_to("emarti GitHUB Repositories", "https://github.com/emartisoft");           
        });
        ui.add_space(20.0); 
    }
}
