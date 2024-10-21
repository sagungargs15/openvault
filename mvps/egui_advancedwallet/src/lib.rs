use eframe::egui;
use egui::{Color32, RichText, Rounding, Stroke, Vec2};
use std::collections::HashMap;

mod screens;
use screens::*;

const PADDING: f32 = 10.0;
const CORNER_RADIUS: f32 = 5.0;

pub struct AdvancedWalletApp {
    current_screen: Screen,
    wallet: Wallet,
    screens: HashMap<Screen, Box<dyn ScreenTrait>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Screen {
    Home,
    Send,
    Receive,
    Transactions,
    Settings,
    Backup,
    Security,
}

pub trait ScreenTrait {
    fn ui(&mut self, ui: &mut egui::Ui, wallet: &mut Wallet);
    fn name(&self) -> &'static str;
}

pub struct Wallet {
    balance: f64,
    address: String,
    transactions: Vec<Transaction>,
}

pub struct Transaction {
    amount: f64,
    date: chrono::DateTime<chrono::Utc>,
    description: String,
    status: TransactionStatus,
}

pub enum TransactionStatus {
    Pending,
    Confirmed,
}

impl Default for AdvancedWalletApp {
    fn default() -> Self {
        let mut screens: HashMap<Screen, Box<dyn ScreenTrait>> = HashMap::new();
        screens.insert(Screen::Home, Box::new(HomeScreen::default()));
        screens.insert(Screen::Send, Box::new(SendScreen::default()));
        screens.insert(Screen::Receive, Box::new(ReceiveScreen::default()));
        screens.insert(Screen::Transactions, Box::new(TransactionsScreen::default()));
        screens.insert(Screen::Settings, Box::new(SettingsScreen::default()));
        screens.insert(Screen::Backup, Box::new(BackupScreen::default()));
        screens.insert(Screen::Security, Box::new(SecurityScreen::default()));

        Self {
            current_screen: Screen::Home,
            wallet: Wallet::default(),
            screens,
        }
    }
}

impl eframe::App for AdvancedWalletApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame::none()
            .fill(Color32::from_rgb(240, 240, 240))
            .inner_margin(PADDING);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            self.top_bar(ui);
            ui.add_space(PADDING);
            
            if let Some(screen) = self.screens.get_mut(&self.current_screen) {
                screen.ui(ui, &mut self.wallet);
            }
            
            ui.add_space(PADDING);
            self.bottom_bar(ui);
        });
    }
}

impl AdvancedWalletApp {
    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("≡").clicked() {
                // Open menu
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙").clicked() {
                    self.current_screen = Screen::Settings;
                }
            });
        });
    }

    fn bottom_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.nav_button(ui, "Home", Screen::Home) {
                self.current_screen = Screen::Home;
            }
            if self.nav_button(ui, "Send", Screen::Send) {
                self.current_screen = Screen::Send;
            }
            if self.nav_button(ui, "Receive", Screen::Receive) {
                self.current_screen = Screen::Receive;
            }
            if self.nav_button(ui, "Transactions", Screen::Transactions) {
                self.current_screen = Screen::Transactions;
            }
        });
    }

    fn nav_button(&self, ui: &mut egui::Ui, text: &str, screen: Screen) -> bool {
        ui.selectable_value(&mut self.current_screen.clone(), screen, text).clicked()
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self {
            balance: 0.12345678,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            transactions: vec![
                Transaction {
                    amount: 0.01,
                    date: chrono::Utc::now() - chrono::Duration::days(1),
                    description: "Received payment".to_string(),
                    status: TransactionStatus::Confirmed,
                },
                Transaction {
                    amount: -0.005,
                    date: chrono::Utc::now() - chrono::Duration::days(3),
                    description: "Coffee purchase".to_string(),
                    status: TransactionStatus::Confirmed,
                },
                Transaction {
                    amount: 0.02,
                    date: chrono::Utc::now() - chrono::Duration::hours(2),
                    description: "Incoming transfer".to_string(),
                    status: TransactionStatus::Pending,
                },
            ],
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
        Box::new(|_cc| Box::new(AdvancedWalletApp::default())),
    )
}

// Native support
pub fn run_native() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(390.0, 844.0)), // iPhone 12 Pro dimensions
        ..Default::default()
    };
    eframe::run_native(
        "Advanced Bitcoin Wallet",
        native_options,
        Box::new(|_cc| Box::new(AdvancedWalletApp::default())),
    )
}
