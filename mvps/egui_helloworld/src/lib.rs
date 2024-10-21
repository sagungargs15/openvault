use eframe::egui;

pub struct HelloWorldApp {
    name: String,
}

impl Default for HelloWorldApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
        }
    }
}

impl eframe::App for HelloWorldApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World App");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(format!("Hello, {}!", self.name));
        });
    }
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    eframe::start_web(canvas_id, Box::new(|cc| Box::new(HelloWorldApp::default())))
}

pub fn run_native() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello World",
        native_options,
        Box::new(|_cc| Box::new(HelloWorldApp::default())),
    )
}