// This enhanced gui.rs includes the following improvements:
// 1. Balance display and refresh functionality.
// 2. New address generation.
// 3. Transaction creation interface with address and amount inputs.
// 4. Transaction signing and broadcasting capabilities.
// Status messages for user feedback.



use egui::{Context, Ui, RichText, Color32, ScrollArea};
use crate::{Wallet, PolicyEngine};
use bdk::bitcoin::{Address, Amount, Txid};
use bdk::wallet::{AddressIndex, PartiallySignedTransaction};
use std::str::FromStr;
use std::path::PathBuf;

pub struct WalletApp {
    wallet: Wallet,
    policy_engine: PolicyEngine,
    balance: u64,
    new_address: String,
    send_to_address: String,
    send_amount: String,
    transaction_hex: String,
    status_message: String,
    policy_string: String,
    transaction_history: Vec<TransactionInfo>,
    backup_path: String,
    restore_path: String,
}

struct TransactionInfo {
    txid: Txid,
    amount: i64,
    confirmations: u32,
}

impl WalletApp {
    pub fn new(wallet: Wallet, policy_engine: PolicyEngine) -> Self {
        Self {
            wallet,
            policy_engine,
            balance: 0,
            new_address: String::new(),
            send_to_address: String::new(),
            send_amount: String::new(),
            transaction_hex: String::new(),
            status_message: String::new(),
            policy_string: String::new(),
            transaction_history: Vec::new(),
            backup_path: String::new(),
            restore_path: String::new(),
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Enterprise Custody Wallet");
            
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Refresh").clicked() {
                        self.refresh_wallet_data();
                    }
                    if ui.button("Backup Wallet").clicked() {
                        self.backup_wallet();
                    }
                    if ui.button("Restore Wallet").clicked() {
                        self.restore_wallet();
                    }
                });
            });

            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                self.show_balance(ui);
                ui.add_space(20.0);
                self.show_new_address(ui);
                ui.add_space(20.0);
                self.show_policy_management(ui);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                self.show_send_transaction(ui);
                ui.add_space(20.0);
                self.show_transaction_hex(ui);
                ui.add_space(20.0);
                self.show_transaction_history(ui);
            });

            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                self.show_status(ui);
            });
        });
    }

    fn refresh_wallet_data(&mut self) {
        if let Err(e) = self.wallet.sync() {
            self.status_message = format!("Failed to sync wallet: {}", e);
            return;
        }

        match self.wallet.get_balance() {
            Ok(balance) => self.balance = balance,
            Err(e) => self.status_message = format!("Failed to get balance: {}", e),
        }

        match self.wallet.get_transaction_history() {
            Ok(history) => self.transaction_history = history,
            Err(e) => self.status_message = format!("Failed to get transaction history: {}", e),
        }
    }

    fn show_balance(&mut self, ui: &mut Ui) {
        ui.label(format!("Current Balance: {} satoshis", self.balance));
    }

    fn show_new_address(&mut self, ui: &mut Ui) {
        if ui.button("Generate New Address").clicked() {
            match self.wallet.get_address(AddressIndex::New) {
                Ok(address) => {
                    self.new_address = address.to_string();
                    self.status_message = "New address generated".to_string();
                },
                Err(e) => self.status_message = format!("Failed to generate address: {}", e),
            }
        }
        ui.text_edit_singleline(&mut self.new_address);
    }

    fn show_policy_management(&mut self, ui: &mut Ui) {
        ui.label("Policy Management");
        ui.text_edit_multiline(&mut self.policy_string);
        if ui.button("Update Policy").clicked() {
            match self.policy_engine.update_policy(&self.policy_string) {
                Ok(_) => self.status_message = "Policy updated successfully".to_string(),
                Err(e) => self.status_message = format!("Failed to update policy: {}", e),
            }
        }
    }

    fn show_send_transaction(&mut self, ui: &mut Ui) {
        ui.label("Send Transaction");
        ui.horizontal(|ui| {
            ui.label("Address:");
            ui.text_edit_singleline(&mut self.send_to_address);
        });
        ui.horizontal(|ui| {
            ui.label("Amount (satoshis):");
            ui.text_edit_singleline(&mut self.send_amount);
        });
        if ui.button("Create Transaction").clicked() {
            self.create_transaction();
        }
    }

    fn create_transaction(&mut self) {
        match (Address::from_str(&self.send_to_address), self.send_amount.parse::<u64>()) {
            (Ok(address), Ok(amount)) => {
                match self.wallet.create_tx(address, Amount::from_sat(amount)) {
                    Ok(psbt) => {
                        self.transaction_hex = psbt.to_string();
                        self.status_message = "Transaction created successfully".to_string();
                    },
                    Err(e) => self.status_message = format!("Failed to create transaction: {}", e),
                }
            },
            _ => self.status_message = "Invalid address or amount".to_string(),
        }
    }

    fn show_transaction_hex(&mut self, ui: &mut Ui) {
        ui.label("Transaction Hex:");
        ui.text_edit_multiline(&mut self.transaction_hex);
        if ui.button("Sign Transaction").clicked() {
            match self.wallet.sign_tx(&self.transaction_hex) {
                Ok(signed_psbt) => {
                    self.transaction_hex = signed_psbt.to_string();
                    self.status_message = "Transaction signed successfully".to_string();
                },
                Err(e) => self.status_message = format!("Failed to sign transaction: {}", e),
            }
        }
        if ui.button("Broadcast Transaction").clicked() {
            match self.wallet.broadcast_tx(&self.transaction_hex) {
                Ok(txid) => self.status_message = format!("Transaction broadcasted: {}", txid),
                Err(e) => self.status_message = format!("Failed to broadcast transaction: {}", e),
            }
        }
    }

    fn show_transaction_history(&self, ui: &mut Ui) {
        ui.label("Transaction History");
        ScrollArea::vertical().show(ui, |ui| {
            for tx in &self.transaction_history {
                ui.label(format!("TXID: {}, Amount: {}, Confirmations: {}", 
                                 tx.txid, tx.amount, tx.confirmations));
            }
        });
    }

    fn show_status(&mut self, ui: &mut Ui) {
        ui.label(RichText::new(&self.status_message).color(Color32::GREEN));
    }

    fn backup_wallet(&mut self) {
        match self.wallet.backup(&PathBuf::from(&self.backup_path)) {
            Ok(_) => self.status_message = "Wallet backed up successfully".to_string(),
            Err(e) => self.status_message = format!("Failed to backup wallet: {}", e),
        }
    }

    fn restore_wallet(&mut self) {
        match Wallet::restore(&PathBuf::from(&self.restore_path)) {
            Ok(wallet) => {
                self.wallet = wallet;
                self.status_message = "Wallet restored successfully".to_string();
            },
            Err(e) => self.status_message = format!("Failed to restore wallet: {}", e),
        }
    }
}