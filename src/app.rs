use eframe::egui::{Id};
use eframe::{App, Frame, egui};

use crate::workspace::Workspace;
use crate::workspace::WorkspaceAction; // WorkspaceAction'ı import et

#[derive(Clone)] // Workspace'e kopyalanabilmesi için
pub struct AppSettings {
    pub global_rounding: egui::CornerRadius,
    // Tek bir pencere arka plan rengi ayarı
    //pub window_background_fill: egui::Color32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            global_rounding: egui::CornerRadius {
                nw: 8, // KuzeyBatı
                ne: 8, // KuzeyDoğu
                sw: 16, // GüneyBatı
                se: 24, // GüneyDoğu
            },
            //window_background_fill: egui::Color32::from_rgba_unmultiplied(233, 238, 235, 255), // Varsayılan pencere arka plan rengi
        }
    }
}
pub struct Application {
    selected_workspace: usize,
    workspaces: Vec<Workspace>,
    show_last_workspace_delete_warning: bool,
    next_workspace_id_counter: usize,
    settings: AppSettings,
}

impl App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);

                ui.separator();

                for (i, workspace) in self.workspaces.iter_mut().enumerate() {
                    if ui
                        .selectable_label(self.selected_workspace == i, &workspace.name)
                        .clicked()
                    {
                        workspace.reset_confirm_delete();
                        self.selected_workspace = i;
                    }
                }

                /*
                if ui.button("Add workspace").clicked() {
                    self.workspaces.push(Workspace::default());
                    self.selected_workspace = self.workspaces.len().saturating_sub(1);
                }
                */

                ui.separator();
            });
        });

        self.selected_workspace = self
            .selected_workspace
            .min(self.workspaces.len().saturating_sub(1));

        // Bir çalışma alanını potansiyel olarak değiştirilebilir şekilde ödünç almadan önce
        // çalışma alanı sayısını al. Bu uzunluk, son çalışma alanının silinip silinmeyeceğine
        // karar vermek için kullanılır.
        let num_workspaces_at_start_of_update = self.workspaces.len();

        match self.workspaces.get_mut(self.selected_workspace) {
            Some(workspace) => {
                let mut open = true;
                let action = workspace.ui(Id::new(self.selected_workspace), &mut open, ctx, &self.settings);

                if !open {
                    if num_workspaces_at_start_of_update > 1 {
                        self.workspaces.remove(self.selected_workspace);
                        // Çalışma alanı kaldırıldıktan sonra seçili çalışma alanını yeniden ayarla
                        self.selected_workspace = self
                            .selected_workspace
                            .min(self.workspaces.len().saturating_sub(1));
                    } else {
                        // Son çalışma alanı silinemez, silme onayını sıfırla
                        workspace.reset_confirm_delete();
                        self.show_last_workspace_delete_warning = true;
                    }
                } else {
                    // Sadece çalışma alanı silinmediyse eylemi işle
                    match action {
                        WorkspaceAction::AddWorkspace => {
                            let new_workspace_name = format!("Workspace{}", self.next_workspace_id_counter);
                            self.workspaces.push(Workspace::new_with_name(new_workspace_name));
                            self.next_workspace_id_counter += 1;
                            self.selected_workspace = self.workspaces.len().saturating_sub(1);
                        }
                        WorkspaceAction::None => {}
                    }
                }
            }
            None => {
                // Tüm çalışma alanları silindiğinde tetiklenebilir. 
                // Ancak bir adet çalışma alanı kaldığında silinmeyecek şekilde
                // kod oluşturulduğu için bu kol çalışmaz.
            }
        }

        if self.show_last_workspace_delete_warning {
            let mut window_is_still_open = true; // Pencerenin kendi 'X' butonu için
            let mut close_warning_requested = false;

            // 1. Arka planı karartma ve etkileşimleri yutma katmanı
            egui::Area::new(Id::new("last_workspace_delete_warning_modal_layer"))
                .fixed_pos(egui::Pos2::ZERO) // Tüm ekranı kapla
                .order(egui::Order::Foreground) // Diğer tüm UI elemanlarının üzerinde çiz
                .show(ctx, |ui| {
                    let screen_rect = ui.ctx().screen_rect();
                    // Tüm ekranı kaplayan alanı etkileşimli hale getirerek
                    // tıklama ve sürüklemeleri tüketmesini sağla.
                    // Bu, alttaki UI elemanlarıyla etkileşimi engeller.
                    let backdrop_response = ui.allocate_rect(screen_rect, egui::Sense::click_and_drag());

                    // Eğer karartılmış arka plana tıklanırsa (uyarı penceresinin dışına),
                    // bu uyarı için özel bir işlem yapmıyoruz (pencere kapanmayacak),
                    // ancak etkileşim burada tüketilmiş olacak.
                    if backdrop_response.clicked() {
                        // İsteğe bağlı: Arka plana tıklama olayını loglayabilirsiniz.
                    }
                    // Arka planı karartma katmanını çiz
                    ui.painter().rect_filled(
                        screen_rect,
                        egui::CornerRadius::ZERO, // Yarı saydam siyah
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 150), // Yarı saydam siyah
                    );
                });

            // 2. Uyarı penceresini göster (arka planın üzerinde)
            egui::Window::new("Warning")
                .id(Id::new("last_workspace_delete_warning_window"))
                .title_bar(false) // Başlık çubuğunu kaldırır
                .order(egui::Order::Tooltip) // Her zaman en üstte olması için Tooltip katmanını kullan
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .frame(egui::Frame::window(&*ctx.style())
                    .corner_radius(self.settings.global_rounding)
                    //.fill(self.settings.window_background_fill)
                    )
                .collapsible(false)
                .resizable(false)
                .open(&mut window_is_still_open)
                .show(ctx, |ui_win| {
                    ui_win.label(""); // Üstte biraz boşluk bırakmak için
                    ui_win.label("The last remaining workspace cannot be deleted.");
                    ui_win.add_space(10.0); // Etiket ile buton arasına biraz boşluk
                    ui_win.with_layout(egui::Layout::top_down(egui::Align::Max), |ui_button_area| {
                        if ui_button_area.button("Ok").clicked() {
                            close_warning_requested = true;
                        }
                    });
                });

            if !window_is_still_open || close_warning_requested {
                self.show_last_workspace_delete_warning = false;
            }
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            selected_workspace: 0,
            workspaces: vec![Workspace::new_with_name("Welcome".to_string())],
            show_last_workspace_delete_warning: false,
            // İlk "Workspace1" için sayaç 1'den başlar.
            // "Welcome" özel bir durum olduğu için sayacı etkilemez.
            next_workspace_id_counter: 1,
            settings: AppSettings::default(),
        }
    }
}
