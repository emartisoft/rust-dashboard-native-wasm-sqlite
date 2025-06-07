mod app;
mod views;
mod workspace;

pub use app::Application;
#[cfg(target_arch = "wasm32")] // When compiling for web
use {
    eframe::wasm_bindgen::{self, prelude::*, JsCast},
    web_sys::HtmlCanvasElement,
};
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> std::result::Result<(), eframe::wasm_bindgen::JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let web_options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(|_cc| Ok(Box::new(Application::default()))),
        )
        .await?;
    Ok(())
}
