use egui::{self, RichText, Color32};
use super::*;

pub struct HomeScreen;
pub struct SendScreen {
    address: String,
    amount: String,
}
pub struct ReceiveScreen;
pub struct TransactionsScreen;
pub struct SettingsScreen;
pub struct BackupScreen;
pub struct SecurityScreen;

impl Default for HomeScreen {
    fn default() -> Self {
        Self
    }
}

impl ScreenTrait for HomeScreen {
    fn ui(&mut self, ui: &mut egui::Ui, wallet: &mut Wallet) {
        ui.vertical_centered(|ui| {
            ui.heading("Balance");
            ui.label(RichText::new(format!("{:.8} BTC", wallet.balance)).size(24.0));
            ui.add_space(20.0);
            
            if ui.button("Send").clicked() {
                // Implement send action
            }
            if ui.button("Receive").clicked() {
                // Implement receive action
            }
        });
    }

    fn name(&self) -> &'static str {
        "Home"
    }
}

impl Default for SendScreen {
    fn default() -> Self {
        Self {
            address: String::new(),
            amount: String::new(),
        }
    }
}

impl ScreenTrait for SendScreen {
    fn ui(&mut self, ui: &mut egui::Ui, wallet: &mut Wallet) {
        ui.heading("Send Bitcoin");
        
        ui.horizontal(|ui| {
            ui.label("Address:");
            ui.text_edit_singleline(&mut self.address);
        });

        ui.horizontal(|ui| {
            ui.label("Amount (BTC):");
            ui.text_edit_singleline(&mut self.amount);
        });

        if ui.button("Send").clicked() {
            println!("Sending {} BTC to {}", self.amount, self.address);
            self.address.clear();
            self.amount.clear();
        }

        ui.label(format!("Current Balance: {} BTC", wallet.balance));
    }

    fn name(&self) -> &'static str {
        "Send"
    }
}

impl Default for ReceiveScreen {
    fn default() -> Self {
        Self
    }
}

impl ScreenTrait for ReceiveScreen {
    fn ui(&mut self, ui: &mut egui::Ui, wallet: &mut Wallet) {
        ui.heading("Receive Bitcoin");
        ui.label("Your address:");
        ui.text_edit_singleline(&mut wallet.address.clone());
        // Here you would typically show a QR code of the address
        if ui.button("Copy Address").clicked() {
            // Implement copy to clipboard
        }
    }

    fn name(&self) -> &'static str {
        "Receive"
    }
}

impl Default for TransactionsScreen {
    fn default() -> Self {
        Self
    }
}

impl ScreenTrait for TransactionsScreen {
    fn ui(&mut self, ui: &mut egui::Ui, wallet: &mut Wallet) {
        ui.heading("Transactions");
        for transaction in &wallet.transactions {
            ui.horizontal(|ui| {
                ui.label(transaction.date.format("%Y-%m-%d %H:%M").to_string());
                ui.label(&transaction.description);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let amount_text = format!("{:.8} BTC", transaction.amount);
                    let amount_color = if transaction.amount >= 0.0 { Color32::GREEN } else { Color32::RED };
                    ui.colored_label(amount_color, amount_text);
                });
            });
            ui.separator();
        }
    }

    fn name(&self) -> &'static str {
        "Transactions"
    }
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self
    }
}

impl ScreenTrait for SettingsScreen {
    fn ui(&mut self, ui: &mut egui::Ui, _wallet: &mut Wallet) {
        ui.heading("Settings");
        if ui.button("Backup Wallet").clicked() {
            // Implement backup action
        }
        if ui.button("Security Settings").clicked() {
            // Navigate to security settings
        }
        if ui.button("Network Settings").clicked() {
            // Implement network settings
        }
    }

    fn name(&self) -> &'static str {
        "Settings"
    }
}

impl Default for BackupScreen {
    fn default() -> Self {
        Self
    }
}

impl ScreenTrait for BackupScreen {
    fn ui(&mut self, ui: &mut egui::Ui, _wallet: &mut Wallet) {
        ui.heading("Backup Wallet");
        ui.label("Your recovery phrase:");
        // In a real app, you'd never display the actual seed phrase like this
        ui.label("word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12");
        if ui.button("Copy Recovery Phrase").clicked() {
            // Implement copy to clipboard
        }
    }

    fn name(&self) -> &'static str {
        "Backup"
    }
}

impl Default for SecurityScreen {
    fn default() -> Self {
        Self
    }
}

impl ScreenTrait for SecurityScreen {
    fn ui(&mut self, ui: &mut egui::Ui, _wallet: &mut Wallet) {
        ui.heading("Security Settings");
        ui.checkbox(&mut true, "Enable Face ID");
        ui.checkbox(&mut false, "Enable PIN");
        if ui.button("Change PIN").clicked() {
            // Implement PIN change
        }
    }

    fn name(&self) -> &'static str {
        "Security"
    }
}
