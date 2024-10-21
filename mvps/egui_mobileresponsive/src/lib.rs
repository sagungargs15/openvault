use eframe::egui;
use egui::{Rect, Vec2};

pub struct BitcoinWalletApp {
    balance: f64,
    address: String,
    transactions: Vec<Transaction>,
}

struct Transaction {
    amount: f64,
    date: String,
    description: String,
}

impl Default for BitcoinWalletApp {
    fn default() -> Self {
        Self {
            balance: 0.05234,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            transactions: vec![
                Transaction {
                    amount: 0.01,
                    date: "2023-06-01".to_string(),
                    description: "Received payment".to_string(),
                },
                Transaction {
                    amount: -0.005,
                    date: "2023-05-28".to_string(),
                    description: "Coffee purchase".to_string(),
                },
            ],
        }
    }
}

impl eframe::App for BitcoinWalletApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_mobile = ctx.screen_rect().width() < 600.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bitcoin Wallet");

            if is_mobile {
                self.mobile_layout(ui);
            } else {
                self.desktop_layout(ui);
            }
        });
    }
}

impl BitcoinWalletApp {
    fn mobile_layout(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.balance_section(ui);
            ui.add_space(20.0);
            self.address_section(ui);
            ui.add_space(20.0);
            self.action_buttons(ui);
            ui.add_space(20.0);
            self.transaction_list(ui);
        });
    }

    fn desktop_layout(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                self.balance_section(ui);
                ui.add_space(20.0);
                self.address_section(ui);
                ui.add_space(20.0);
                self.action_buttons(ui);
            });
            ui.add_space(40.0);
            self.transaction_list(ui);
        });
    }

    fn balance_section(&self, ui: &mut egui::Ui) {
        ui.heading(format!("{:.8} BTC", self.balance));
        ui.label("Current Balance");
    }

    fn address_section(&self, ui: &mut egui::Ui) {
        ui.label("Your Bitcoin Address:");
        ui.text_edit_singleline(&mut self.address.clone());
    }

    fn action_buttons(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Send").clicked() {
                // Implement send functionality
            }
            if ui.button("Receive").clicked() {
                // Implement receive functionality
            }
        });
    }

    fn transaction_list(&self, ui: &mut egui::Ui) {
        ui.heading("Recent Transactions");
        for transaction in &self.transactions {
            ui.horizontal(|ui| {
                ui.label(&transaction.date);
                ui.label(&transaction.description);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.8} BTC", transaction.amount));
                });
            });
            ui.separator();
        }
    }
}

// Web support
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        canvas_id,
        web_options,
        Box::new(|_cc| Box::new(BitcoinWalletApp::default())),
    )
}

// Native support
pub fn run_native() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Bitcoin Wallet",
        native_options,
        Box::new(|_cc| Box::new(BitcoinWalletApp::default())),
    )
}