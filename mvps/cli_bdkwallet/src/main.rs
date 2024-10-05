#![allow(unused)]

use bdk::template::Bip84;
use bdk::keys::{
    bip39::{Language, Mnemonic, WordCount},
    DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
};
use bdk::{
    bitcoin::secp256k1::Secp256k1,
    descriptor::{Descriptor, DescriptorPublicKey},
    KeychainKind,
    FeeRate, SignOptions, SyncOptions, Wallet
};
use std::str::FromStr;
use bdk::bitcoin::{Address, Network, PrivateKey};
use bdk::bitcoin::Network::Testnet;
use bdk::blockchain::{Blockchain, ElectrumBlockchain};
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bdk::wallet::AddressIndex;
use bdk::{miniscript};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // // Initialize Secp256k1 context
    let secp = Secp256k1::new();

    let network = Network::Testnet;
 
    // Generate a new mnemonic
    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    let mnemonic_words = mnemonic.to_string();
    let mnemonic = Mnemonic::parse(&mnemonic_words).unwrap();
    println!("Generated mnemonic Generated {}", mnemonic_words);

    // Generate the extended key
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    println!("Extended key Generated" );
    
    // Get xprv from the extended key
    let xprv = xkey.into_xprv(network).unwrap();
    println!("Xprv: {}", xprv);

    // // testNet generated private key for demo purposes. 
    // let private_key = PrivateKey::from_wif("cNR1rFRvrXU6UWrhvfrpwtoG7zLdTEzgAfMcZifaF5vDgiDPrVtX").unwrap();
    // let xpub = key::PublicKey::from_private_key(&secp, &private_key);
    
    // Function to create descriptor based on keychain kind for BIP84
    fn create_descriptor_bip84(
        keychain_kind: KeychainKind,
        xpub: &DescriptorPublicKey,
    ) -> Result<String, Box<dyn std::error::Error>> {

        // Here we're assuming BIP84 which uses wpkh (witness program with a public key hash)
        // You can adjust this based on your needs (like BIP86 for Taproot)
       
        // Convert KeychainKind to the appropriate number
        let keychain_num = match keychain_kind {
            KeychainKind::External => 0,
            KeychainKind::Internal => 1,
        };
        let descriptor = Descriptor::new_wpkh(xpub.clone())?;

        Ok(descriptor.to_string())
    }

    // Generate external descriptor for BIP84 using Templates in BDK
    let external_descriptor_bip84 = Bip84(xprv, KeychainKind::External);
    println!("Wallet Information:\nExternal Descriptor (BIP84)");

    // Generate internal descriptor for BIP84 using Templates in BDK
    let internal_descriptor_bip84 = Bip84(xprv, KeychainKind::Internal);
    println!("Wallet Information:\nInternal Descriptor (BIP84)");
    // Descriptor Template: Changed from BIP84 to BIP86. 
    // BIP86 uses a different derivation path that starts with m/86' instead of m/84' for Taproot.

    // let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    // let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";

    // Certainly, adjusting for BIP86 (which uses Taproot) involves using different descriptor templates and potentially modifying the derivation path. 
    // You can modify the code to generate descriptors with BIP86:
    // This modification ensures that your wallet uses the more modern Taproot script type, 
    // which is beneficial for privacy, efficiency, and is part of Bitcoin's ongoing development towards more advanced transaction formats.


    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let wallet = Wallet::new(
        external_descriptor_bip84,
        Some(internal_descriptor_bip84),
        network,
        MemoryDatabase::default(),
    )
    .unwrap();

    let address = wallet.get_address(AddressIndex::New)?;
    println!("Generated Address: {}", address);

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    // wallet.sync(&blockchain, SyncOptions::default())?;
    wallet.sync(&blockchain, SyncOptions::default()).expect("Wallet did not sync");

    let balance = wallet.get_balance()?;
    println!("Wallet balance in SAT: {}", balance);

    //The Address::from_str method might return a Result, so you should handle potential errors. 
    // let faucet_address = Address::from_str("tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6")?;
    let faucet_address = Address::from_str("mv4rnyY3Su5gjcDNzbMLKBQkBicCtHUtFB")
        .map_err(|e| format!("Invalid address: {}", e))?
        .require_network(Testnet)
        .map_err(|e| format!("Network mismatch: {}", e))?;

    // create a transaction
    let mut tx_builder = wallet.build_tx();
    tx_builder
        .add_recipient(faucet_address.script_pubkey(),50_000)
        .enable_rbf()
        .fee_rate(FeeRate::from_sat_per_vb(5.0));
    let (mut psbt, tx_details) = tx_builder.finish()?;

    println!("Transaction details: {:#?}", tx_details);

    // Sign the transaction
    let finalized =  wallet.sign(&mut psbt, SignOptions::default()).expect("Transaction couldn't be finalized");    
    // let finalized = wallet.sign(&mut psbt,SignOptions::default())?;
    assert!(finalized,"Tx has not been finalized");
    println!("Transaction Signed: {}", finalized);

    // Broadcast the transaction
    let raw_transaction = psbt.extract_tx();
    let txid = raw_transaction.txid();
    blockchain.broadcast(&raw_transaction)?;
    println!("Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}",txid = txid);

    Ok(())
}
