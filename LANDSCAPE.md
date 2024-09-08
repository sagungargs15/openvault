# Bitcoin ONLY B2B Wallet/Enterprise Custody Solution Providers as Software or Service
    1. Open Source
           a. [Caravan](https://help.unchained.com/what-is-caravan), [github repo](https://unchained-capital.github.io/caravan/#/)
           b. [Liana](https://wizardsardine.com/liana/), [github repo](https://wizardsardine.com/liana/)
           c. [Galoy](https://www.galoy.io/ ), [github repo](https://github.com/GaloyMoney/blink)
           d. [My Citadel](https://mycitadel.io/), [github repo](https://github.com/mycitadel/mycitadel-desktop)
    2. Proprietary 
         a. AnchorWatch - https://anchorwatch.com/
         b. OnRamp - https://onrampbitcoin.com/
         c. Acropolis - https://www.acropolistreasury.com/
         d. River - https://river.com/business

# DevKits/Protocols/DevTools
  1. **[Revault Protocol](https://github.com/revault/revaultd)**: Advanced Bitcoin custody for organizations. It is a Bitcoin vault architecture for multi-party situations. Revault's design philosophy aligns with using what's currently available in Bitcoin's protocol, showcasing how advanced custody solutions can be built without waiting for future protocol upgrades like covenants. This approach demonstrates the flexibility and power of Bitcoin's existing features when creatively applied to solve real-world problems like secure custody.The Revault architecture primarily focuses on using Bitcoin's native features like multi-signature transactions, time-locks, and script capabilities to create a robust custody solution. It doesn't introduce new BIPs but rather innovatively combines existing functionalities:
      
      a.Multi-signature Transactions: Utilizes Bitcoin's native support for multi-sig, which doesn't require a specific BIP but is enabled by Bitcoin's scripting language.
      
      b. Time-locks: Uses Bitcoin's OP_CHECKLOCKTIMEVERIFY (CLTV) and potentially OP_CHECKSEQUENCEVERIFY (CSV) for creating time-locked transactions, which are integral to Revault's emergency and unvault mechanisms.
      
      c. Scripting: While not a BIP, Bitcoin's scripting language allows for the creation of complex conditions for spending, which Revault leverages for its transaction types (Spend, Cancel, Unvault Emergency).
    
  2. **[Bitcoin Dev Kit](https://bitcoindevkit.org/)**, **[BDK](https://github.com/bitcoindevkit/bdk)** : A modern, lightweight, descriptor-based wallet library written in Rust
  3. **[Elephant](https://github.com/bitcoindevkit/elephant)** : Elephant is a Bitcoin Wallet designed to demonstrate the power of Bitcoin policies in a sandbox environment.
  4. **[Miniscript Rust SDK](https://docs.rs/miniscript/latest/miniscript/)**, **[github](https://docs.rs/miniscript/latest/miniscript/)** :Support for Miniscript and Output Descriptors for rust-bitcoin
  5. **[Rust Bitcoin library](https://rust-bitcoin.org)**, **[github](https://github.com/rust-bitcoin/rust-bitcoin)** : The rust-bitcoin crate is a library that supports the Bitcoin network protocol and associated primitives. It is designed for Rust programs built to work with the Bitcoin network.
  6. **[Rust Bitcoin Community](https://github.com/rust-bitcoin)** : A Series of Projects to implement various Bitcoin Protocols in Rust 

# BIPs related to Custody, Inheritance, Miniscript, Privacy and Covenants
  1. **[BIP-119](https://github.com/bitcoin/bips/blob/master/bip-0119.mediawiki)** :This BIP introduces a simple covenant called a *template* which enables a limited set of highly valuable use cases without significant risk. BIP-119 templates allow for non-recursive fully-enumerated covenants with no dynamic state. CTV serves as a replacement for a pre-signed transaction oracle, which eliminates the trust and interactivity requirements. Examples of uses include vaults, non-interactive payment channel creation, congestion controlled batching, efficient to construct discreet log contracts, and payment pools, among many other 
  2. **[BIP-347](https://github.com/bitcoin/bips/blob/master/bip-0347.mediawiki)** : The introduction of OP_CAT under BIP-347 would not directly implement covenants but would provide a fundamental building block that could be used to construct covenants, thereby expanding Bitcoin's functionality significantly. Covenants in this context refer to mechanisms where the script of a Bitcoin transaction can restrict how the bitcoins can be spent in the future, essentially adding rules or conditions to the transaction's output. This proposal aims to enhance Bitcoin's scripting capabilities, allowing for more complex smart contracts, secure bridges, and on-chain trading by enabling covenants. 
  3. **[BIP-115](https://github.com/bitcoin/bips/blob/master/bip-0115.mediawiki)** (OP_CHECKMULTISIG). Multisig setups can be used for inheritance planning where multiple keys are required to spend funds, ensuring that no single point of failure exists. This could involve setting up transactions that require signatures from both the owner during their lifetime and potentially from heirs or executors after their passing.
  4. **[BIP-174](https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki)**  (Partially Signed Bitcoin Transactions - PSBT) provides a format for representing Bitcoin transactions that can be partially signed, which could be useful in scenarios where multiple parties (like executors or heirs) need to sign off on transactions after someone's death.
  5. **[BIP-352](https://github.com/bitcoin/bips/blob/master/bip-0352.mediawiki)** : (Silent Payments): Although not directly related to custody, the privacy aspects introduced by BIP-352 could theoretically be integrated or considered for enhancing the privacy of transactions within a custody solution like Revault.
  6. **[BIP-340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)** (Taproot) : These are not explicitly mentioned in the context of Revault's design but are crucial for enhancing Bitcoin's scripting capabilities. Revault's use of complex transaction scripts for emergency and regular transactions could theoretically benefit from or be designed with Taproot's capabilities in mind, although Revault's described functionality predates or doesn't require Taproot for its core operations.
  7. **[BIP-341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)** (Tapscript): Vault custody designs might not directly depend on Tapscript for their immediate functionality, BIP-341 (Taproot) enhances the overall ecosystem in which these solutions operate. It offers better privacy, efficiency, and future flexibility, which are all critical for the long-term viability and security of Bitcoin vault custody systems.

# Crypto B2B Enterprise Custody Solutions
  1. [BitGo](https://www.bitgo.com/)
  2. [Coinbase Custody](https://www.coinbase.com/en-sg/prime/custody)
  3. [Safeheron](https://safeheron.com/) 
  4. [Bakkt](https://bakkt.com/) 
  5. [Fireblocks](https://www.fireblocks.com/)
  6. [CheckSig](https://www.checksig.com/)
  7. [Porto](https://www.porto.xyz//) by [Anchorage Digital](https://www.anchorage.com/)
  8. [Atato](https://www.atato.com/) 
   
# B2C Multi-Sig Software Wallet Providers
   1. Casa - https://casa.io/
   2. Theya - https://www.theya.us/
   3. Nunchuk - https://nunchuk.io/

# B2C Hardware Wallet Signing Devices 
  1. Trezor
  2. Ledger
  3. Cold Card M4/Q
  4. Bitbox
  5. Seed Signer
  6. Foundation Passport
  7. Jade
  8. [Portal](https://github.com/TwentyTwoHW/portal-software)
  9. [Frost](https://frostsnap.com/)



# Reference Talks on Self-Custody, Inheritance, Enterprise Custody, Miniscript and Covenants
  1. [24Jan2024]Bitcoin Custody for Institutions w/ Caitlin Long & Wes Knobel (BTC166): [Video](https://www.youtube.com/watch?v=u8UpiYJqCTo)
  2. [16Jan2023] Daniela Brozzoni - Life is short, Bitcoin is forever: [Video](https://www.youtube.com/watch?v=QquvK-gMOFk)
  2. [28Aoct2023]Collaborative custody bitcoin vaults for private wealth and enterprise: [Video](https://www.youtube.com/watch?v=0as_B7wfAp0)
  3. [22Aug2022] Bitcoin Energy & Custody w/ Parker Lewis & Will Cole: [Video](https://www.youtube.com/watch?v=Xw9kQFJcXds&t=0s)
  4. [21Jul2022]The Bitcoin Custody Protocol - Fedimint w/ Obi Nwosu: [Video](https://www.youtube.com/watch?v=X1OlFs2IMgo)  
  5. [17Mar2022] Bitcoin Retirement Planning & Self-Custody w/ Parker Lewis & Jeff Vandrew: [Video](https://www.youtube.com/watch?v=vA1bOKbEIzI)
  6. [16Jun2021] Bitcoin Security and Self Custody w/ Nick Neuman: [Video](https://www.youtube.com/watch?v=vA1bOKbEIzI)