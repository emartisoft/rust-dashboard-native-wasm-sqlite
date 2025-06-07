use eframe::egui;
use super::View;
use crate::app::AppSettings;
use serde_json;
use egui_extras; // egui_extras'ı doğrudan kullanacağız
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use reqwest; // Native HTTP istekleri için

#[cfg(target_arch = "wasm32")]
use {
    poll_promise::Promise,
    eframe::wasm_bindgen::JsCast, // eframe üzerinden JsCast
    wasm_bindgen_futures::JsFuture,
    web_sys::{Request, RequestInit, RequestMode, Response},
};



// Sunucudan gelen müşteri verisi için bir struct tanımlayalım
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")] // JSON anahtarlarının PascalCase olduğunu belirtir
struct Customer {
    customer_name: String,
    address: String,
    #[serde(rename = "CustomerID")] // JSON'daki 'CustomerID' anahtarıyla eşleştir
    customer_id: i64, 
}

pub const WINDOW_TITLE: &str = "Connect Sqlite Database";

#[cfg(not(target_arch = "wasm32"))]
fn fetch_customer_data_from_server() -> Result<String, String> {
    match reqwest::blocking::get("http://localhost:3000/customers") {
        Ok(response) => {
            if response.status().is_success() {
                response.text().map_err(|e| format!("Failed to read response text: {}", e))
            } else {
                Err(format!("Request failed with status: {}", response.status()))
            }
        }
        Err(e) => Err(format!("HTTP request failed: {}", e)),
    }
}

#[cfg(target_arch = "wasm32")]
async fn fetch_customer_data_from_server_wasm() -> Result<String, String> {
    let opts = RequestInit::new(); // Derleyici uyarısına göre 'mut' kaldırıldı
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors); // CORS gerekli olabilir

    let request = Request::new_with_str_and_init("http://localhost:3000/customers", &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let window = web_sys::window().ok_or_else(|| "Failed to get window object".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;

    // Yanıtın Response türünde olduğundan emin ol
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|e| format!("Failed to cast to Response: {:?}", e))?;

    if resp.ok() { // status 200-299
        let text = JsFuture::from(resp.text().map_err(|e| format!("Failed to get text from response: {:?}", e))?)
            .await
            .map_err(|e| format!("Failed to convert text promise: {:?}", e))?;
        text.as_string().ok_or_else(|| "Response text was not a string".to_string())
    } else {
        Err(format!("Request failed with status: {}", resp.status()))
    }
}

#[derive(Default)]
pub struct SqliteData {
    customer_data_json: String, // Çekilen JSON verisini saklamak için
    error_message: Option<String>, // Hata mesajlarını saklamak için
    #[cfg(target_arch = "wasm32")]
    data_promise: Option<Promise<Result<String, String>>>,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))] // Native'de kullanılmayacak ama WASM için gerekli
    parsed_customers: Vec<Customer>, // Parse edilmiş müşteri verilerini saklamak için
    selected_customer_id_for_label: Option<i64>, // Tıklanan müşterinin ID'sini saklamak için
    data_fetched_on_open: bool, // Pencere açıldığında verinin çekilip çekilmediğini takip eder
}

impl SqliteData {
    fn process_fetched_json(&mut self, raw_json_result: Result<String, String>) {
        match raw_json_result {
            Ok(raw_json) => {
                match serde_json::from_str::<serde_json::Value>(&raw_json) {
                    Ok(parsed_json) => { // Değişken adı düzeltildi
                        match serde_json::to_string_pretty(&parsed_json) {
                            Ok(pretty_json) => {
                                self.customer_data_json = pretty_json;
                            }
                            Err(_) => {
                                self.customer_data_json = raw_json;
                                self.error_message = Some("Warning: Could not pretty-print JSON.".to_string());
                            }
                        }
                        // Şimdi de Customer listesi olarak parse etmeye çalışalım
                        match serde_json::from_value::<Vec<Customer>>(parsed_json.clone()) { // parsed_json kullanıldı ve clone eklendi
                            Ok(customers) => {
                                self.parsed_customers = customers;
                            }
                            Err(e) => {
                                self.parsed_customers.clear();
                                self.error_message = Some(format!("Warning: Could not parse JSON into Customer list: {}", e));
                            }
                        }
                    }
                    Err(_) => {
                        self.customer_data_json = raw_json;
                        self.parsed_customers.clear();
                        self.error_message = Some("Warning: Fetched data is not valid JSON.".to_string());
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(e);
            }
            
        }
    }

    // Bu metod SqliteData'ya özel olduğu için impl SqliteData bloğunda kalmalı.

    // Veri çekme işlemini başlatan yardımcı fonksiyon
    fn trigger_fetch_data(&mut self) {
        self.error_message = None;
        self.customer_data_json.clear();
        self.parsed_customers.clear();

        #[cfg(not(target_arch = "wasm32"))]
        {
            let result = fetch_customer_data_from_server();
            self.process_fetched_json(result);
            self.data_fetched_on_open = true; // Native'de işlem senkron olduğu için hemen true yapabiliriz
        }

        #[cfg(target_arch = "wasm32")]
        {
            let (sender, promise) = poll_promise::Promise::new();
            wasm_bindgen_futures::spawn_local(async move {
                let result = fetch_customer_data_from_server_wasm().await;
                let _ = sender.send(result);
            });
            self.data_promise = Some(promise);
            // data_fetched_on_open WASM'da promise tamamlandığında true yapılmalı
        }
    }
}

impl View for SqliteData {
    fn title(&self) -> String {
        WINDOW_TITLE.to_string()
    }

    fn show(&mut self, ctx: &egui::Context, id: egui::Id, open: &mut bool, settings: &AppSettings) {
        
        egui::Window::new(self.title())
            .id(id)
            .default_width(480.0)
            .frame(egui::Frame::window(&*ctx.style())
                .corner_radius(settings.global_rounding)
                //.fill(settings.window_background_fill)
            )
            .open(open) // Doğrudan 'open' değişkenini kullan
            .show(ctx, |ui| {
                self.ui(ui);
                // Pencere ilk kez açılıyorsa ve veri henüz çekilmemişse veriyi çek
                // `open` burada pencerenin o anki görünürlüğünü değil, bir sonraki karede açık olup olmayacağını belirtir.
                // Bu yüzden, veri çekme işlemini ui() içinde veya burada daha dikkatli yönetmek gerekebilir.
                // Şimdilik, eğer data_promise None ise ve data_fetched_on_open false ise tetikleyelim.
                // Veya daha basitçe, eğer parsed_customers boşsa ve data_promise yoksa.
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Customer Data from Server");
        ui.separator();

        // Pencere ilk açıldığında veya veri henüz çekilmemişse veriyi çek
        #[cfg(not(target_arch = "wasm32"))]
        let should_fetch = !self.data_fetched_on_open && self.parsed_customers.is_empty();
        #[cfg(target_arch = "wasm32")]
        let mut should_fetch = !self.data_fetched_on_open && self.parsed_customers.is_empty();
        #[cfg(target_arch = "wasm32")]
        {
            
            should_fetch = should_fetch && self.data_promise.is_none();
        }
        if should_fetch {
            self.trigger_fetch_data();
        }

        if ui.button("Fetch Customer Data").clicked() {
            self.trigger_fetch_data(); // Butona tıklandığında da veri çekme işlemini tetikle
        }

        #[cfg(target_arch = "wasm32")]
        if let Some(promise) = &self.data_promise {
            if let Some(result) = promise.ready() {
                self.process_fetched_json(result.clone());
                self.data_promise = None;
                self.data_fetched_on_open = true; // WASM'da promise tamamlandığında true yap
            } else {
                ui.spinner();
                ui.label("Fetching data from server (WASM)...");
            }
        }

        if let Some(err_msg) = &self.error_message {
            ui.colored_label(egui::Color32::RED, err_msg);
        }

        ui.add_space(10.0);

        // Tablo ve TextEdit arasında geçiş için bir sekme yapısı veya ayırıcı kullanılabilir.
        // Şimdilik ikisini de gösterelim.
        ui.collapsing("Raw JSON Data", |ui| {
            ui.add_sized(
                ui.available_size() - egui::vec2(0.0, 0.0), // İçerik için tüm alanı kullan
                egui::TextEdit::multiline(&mut self.customer_data_json)
                    .hint_text("Click 'Fetch Customer Data' to load data from http://localhost:3000/customers")
                    .desired_width(f32::INFINITY)
                    .desired_rows(5)
            );
        });

        ui.separator();
        
        egui::CollapsingHeader::new("Customer Data Table")
            .default_open(true) // Başlangıçta açık olmasını sağlar
            .show(ui, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .column(egui_extras::Column::initial(100.0).at_least(40.0))
                    .column(egui_extras::Column::initial(200.0).at_least(80.0))
                    .column(egui_extras::Column::remainder().at_least(100.0))
                    .header(20.0, |mut header| {
                        header.col(|ui| { ui.strong("Customer ID"); });
                        header.col(|ui| { ui.strong("Customer Name"); });
                        header.col(|ui| { ui.strong("Address"); });
                    })
                    .body(|mut body| {
                        for customer_data in &self.parsed_customers {
                            body.row(30.0, |mut row| {
                                row.col(|ui| { ui.label(customer_data.customer_id.to_string()); });
                                row.col(|ui| {
                                    if ui.link(&customer_data.customer_name).clicked() {
                                        self.selected_customer_id_for_label = Some(customer_data.customer_id);
                                    }
                                });
                                row.col(|ui| { ui.label(&customer_data.address); });
                            });
                        }
                    });
            });

        ui.separator();

        if let Some(customer_id) = self.selected_customer_id_for_label {
            ui.colored_label(
                egui::Color32::from_rgb(100, 200, 100), // Yeşilimsi bir renk
                format!("Selected Customer ID: {}", customer_id)
            );
        }

        ui.add_space(20.0); 
    }
}
