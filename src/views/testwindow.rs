use eframe::egui;
use super::View;
use crate::app::AppSettings;

pub const WINDOW_TITLE: &str = "Test Window";

#[derive(Default)] // close_button_pressed varsayılan olarak false olacaktır
pub struct TestWindow {
    close_button_pressed: bool,
}

impl View for TestWindow {
    fn title(&self) -> String {
        WINDOW_TITLE.to_string()
    }

    fn show(&mut self, ctx: &egui::Context, id: egui::Id, open: &mut bool, settings: &AppSettings) {
        self.close_button_pressed = false; // Her gösterimde bayrağı sıfırla
        let mut window_is_open_for_egui = *open;

        egui::Window::new(self.title())
            .id(id)
            .default_width(320.0)
            .frame(egui::Frame::window(&*ctx.style())
                .corner_radius(settings.global_rounding)
                //.fill(settings.window_background_fill)
            )
            .open(&mut window_is_open_for_egui)
            .show(ctx, |ui| {
                self.ui(ui);
            });

        if self.close_button_pressed {
            *open = false;
        } else {
            *open = window_is_open_for_egui;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        // Orijinal stil ayarını kaldırabilir veya ihtiyaca göre tutabilirsiniz.
        // ui.style_mut().spacing.interact_size.y = 0.0;

        ui.heading("Test Window");
        ui.separator();

        ui.label("This is a test window with a special label.");
        ui.add_space(10.0); // Etiket ile buton arasına biraz boşluk ekleyelim

        // Butonu sağa dayalı olarak yerleştirmek için
        ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui_button_area| {
            if ui_button_area.button("Close").clicked() {
                self.close_button_pressed = true;
            }
        });
        ui.add_space(20.0); 
    }
}
