#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Application Title",
        native_options,
            Box::new(|_cc| Ok(Box::new(emartident_rust::Application::default()))),
    )
}
