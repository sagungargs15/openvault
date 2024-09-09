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
      
      a. Multi-signature Transactions: Utilizes Bitcoin's native support for multi-sig, which doesn't require a specific BIP but is enabled by Bitcoin's scripting language.
      
      b. Time-locks: Uses Bitcoin's OP_CHECKLOCKTIMEVERIFY (CLTV) and potentially OP_CHECKSEQUENCEVERIFY (CSV) for creating time-locked transactions, which are integral to Revault's emergency and unvault mechanisms.
      
      c. Scripting: While not a BIP, Bitcoin's scripting language allows for the creation of complex conditions for spending, which Revault leverages for its transaction types (Spend, Cancel, Unvault Emergency).
    
  2. **[Bitcoin Dev Kit](https://bitcoindevkit.org/)**, **[BDK](https://github.com/bitcoindevkit/bdk)** : A modern, lightweight, descriptor-based wallet library written in Rust
  3. **[Elephant](https://github.com/bitcoindevkit/elephant)** : Elephant is a Bitcoin Wallet designed to demonstrate the power of Bitcoin policies in a sandbox environment.
  4. **[Miniscript Rust SDK](https://docs.rs/miniscript/latest/miniscript/)**, **[github](https://docs.rs/miniscript/latest/miniscript/)** :Support for [Miniscript](https://bitcoin.sipa.be/miniscript/) and Output Descriptors for rust-bitcoin
  5. **[Rust Bitcoin library](https://rust-bitcoin.org)**, **[github](https://github.com/rust-bitcoin/rust-bitcoin)** : The rust-bitcoin crate is a library that supports the Bitcoin network protocol and associated primitives. It is designed for Rust programs built to work with the Bitcoin network.
  6. **[Rust Bitcoin Community](https://github.com/rust-bitcoin)** : A Series of Projects to implement various Bitcoin Protocols in Rust 

# BIPs related to Custody, Inheritance, Miniscript, Privacy and Covenants
  1. **[BIP-119](https://github.com/bitcoin/bips/blob/master/bip-0119.mediawiki)** :This BIP introduces a simple covenant called a *template* which enables a limited set of highly valuable use cases without significant risk. BIP-119 templates allow for non-recursive fully-enumerated covenants with no dynamic state. CTV serves as a replacement for a pre-signed transaction oracle, which eliminates the trust and interactivity requirements. Examples of uses include vaults, non-interactive payment channel creation, congestion controlled batching, efficient to construct discreet log contracts, and payment pools, among many other 
  2. **[BIP-347](https://github.com/bitcoin/bips/blob/master/bip-0347.mediawiki)** : The introduction of OP_CAT under BIP-347 would not directly implement covenants but would provide a fundamental building block that could be used to construct covenants, thereby expanding Bitcoin's functionality significantly. Covenants in this context refer to mechanisms where the script of a Bitcoin transaction can restrict how the bitcoins can be spent in the future, essentially adding rules or conditions to the transaction's output. This proposal aims to enhance Bitcoin's scripting capabilities, allowing for more complex smart contracts, secure bridges, and on-chain trading by enabling covenants. 
  3. **[BIP-115](https://github.com/bitcoin/bips/blob/master/bip-0115.mediawiki)** (OP_CHECKMULTISIG). Multisig setups can be used for inheritance planning where multiple keys are required to spend funds, ensuring that no single point of failure exists. This could involve setting up transactions that require signatures from both the owner during their lifetime and potentially from heirs or executors after their passing.
  4. **[BIP-174](https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki)** :(Partially Signed Bitcoin Transactions - PSBT) provides a format for representing Bitcoin transactions that can be partially signed, which could be useful in scenarios where multiple parties (like executors or heirs) need to sign off on transactions after someone's death.
  5. **[BIP-352](https://github.com/bitcoin/bips/blob/master/bip-0352.mediawiki)** :(Silent Payments): Although not directly related to custody, the privacy aspects introduced by BIP-352 could theoretically be integrated or considered for enhancing the privacy of transactions within a custody solution like Revault.
  6. **[BIP-65](https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki)** :Introduced OP_CHECKLOCKTIMEVERIFY (CLTV), which allows for absolute timelocks, locking transactions until a specific block height or timestamp is reached.
  7. **[BIP-68](https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki)** :Introduced the concept of relative timelocks, allowing transactions to be locked until a certain number of blocks have passed since the previous transaction's output was spent. This was implemented through the nSequence field, which, when used in conjunction with the OP_CHECKSEQUENCEVERIFY (CSV) opcode introduced by BIP112, enables complex smart contract functionalities like atomic swaps and more secure escrow services.
  8. **[BIP-112](https://github.com/bitcoin/bips/blob/master/bip-0112.mediawiki)** :(CHECKSEQUENCEVERIFY) complements BIP68 by adding the OP_CHECKSEQUENCEVERIFY opcode to the Bitcoin scripting language. This opcode checks if the relative lock-time has been satisfied, making it possible to enforce timelocks at the script level.
  9. **[BIP-388](https://github.com/bitcoin/bips/blob/master/bip-0388.mediawiki)**: Wallet Policies for Descriptor Wallets. It defines a standardized way to describe wallet policies for descriptor wallets. These are wallets where the outputs (funds) are described using a simple "language" or script, which includes all necessary metadata about key usage. This allows for precise control over how keys are used, which is crucial for security and functionality.Wallet Policy Structure: The policy includes information like key origins, derivation paths, and script types. It's designed to be compact enough for human inspection during setup, focusing on essential features to reduce memory usage on devices with limited capacity. Security Implications: By having users verify the wallet policy on the hardware wallet's screen, BIP 388 ensures that no transaction can proceed without user consent, provided the user verifies the displayed information correctly. This addresses one of the key security concerns in hardware wallet usage.The primary goal of BIP 388 is to enhance security and user verification in hardware wallets by introducing a structured way to describe how funds can be spent. This policy must be verifiable on the hardware wallet's screen, ensuring that users can confirm the spending conditions before approving transactions.
  10. **[BIP-340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)**: This BIP introduces Schnorr signatures to Bitcoin, which is foundational for implementing threshold signatures like Frost. Schnorr signatures allow for signature aggregation, which is crucial for threshold schemes where multiple parties can collaboratively sign a transaction. Discussions around how threshold signatures could be applied or are being applied in Bitcoin, especially post-Taproot activation. BIP-340 provides the cryptographic foundation for such advanced signature schemes. Schnorr signatures are a cryptographic signature scheme that offers several advantages over the previously used Elliptic Curve Digital Signature Algorithm (ECDSA). Schnorr signatures allow for multi-signature transactions to be aggregated into a single signature, which not only reduces the size of transactions but also enhances privacy by making multi-signature transactions look like regular single-signature transactions. This aggregation also improves transaction privacy and efficiency. 
  11. **[BIP-341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)** :Taproot enhances the overall ecosystem in which these solutions operate. It offers better privacy, efficiency, and future flexibility, which are all critical for the long-term viability and security of Bitcoin vault custody systems. This proposal defines the new output type for Bitcoin transactions known as Pay-to-Taproot (P2TR). Taproot leverages the Schnorr signatures introduced in BIP 340. Taproot improves the efficiency of complex transactions by allowing them to be hidden within what's known as a Merkle tree. This means that only the part of the script that's actually used needs to be revealed, enhancing privacy and reducing transaction size for complex scripts. It essentially makes Bitcoin's scripting capabilities more flexible and efficient, allowing for more complex smart contracts while maintaining privacy. These are not explicitly mentioned in the context of Revault's design but are crucial for enhancing Bitcoin's scripting capabilities. Revault's use of complex transaction scripts for emergency and regular transactions could theoretically benefit from or be designed with Taproot's capabilities in mind, although Revault's described functionality predates or doesn't require Taproot for its core operations.
  12.**[BIP-342](https://github.com/bitcoin/bips/blob/master/bip-0342.mediawiki)** Tapscript: Vault custody designs might not directly depend on Tapscript for their immediate functionality. This BIP modifies Bitcoin's scripting language to support the new Taproot output type. It introduces what's known as Tapscript, which includes new opcodes and changes to how scripts are verified.Tapscript allows for more efficient script execution, particularly beneficial for Schnorr signatures. It includes changes like the introduction of OP_SUCCESS opcodes, which can be used to future-proof the script language by allowing for the addition of new operations without immediately activating them. This flexibility is crucial for ongoing development and improvements in Bitcoin's scripting capabilities.
  13. **[BIP-327](https://github.com/bitcoin/bips/blob/master/bip-0327.mediawiki)**: "MuSig2 for BIP340-compatible Multi-Signatures". It is a Bitcoin Improvement Proposal aimed at enhancing the efficiency and privacy of multi-signature transactions on the Bitcoin network. BIP 327 introduces MuSig2, which is a multi-signature scheme allowing multiple signers to create a single aggregate public key and cooperatively produce ordinary Schnorr signatures valid under this aggregate key. This is particularly beneficial for scenarios requiring shared control over funds, like in multi-party transactions or enhanced security setups. **Key Aggregation**: Each participant in the multi-signature setup contributes their public key. These keys are then aggregated into a single public key using a specific algorithm. This aggregation process ensures that the resulting key does not reveal how many parties are involved, enhancing privacy. **Signature Creation**: The signing process involves each participant generating a nonce (a number used only once) and a partial signature. These are then combined into a final signature that validates against the aggregate public key. This process requires interaction between all signers but reduces the on-chain footprint to that of a single signature. **Security and Verification**: The scheme is designed to be secure against various attacks, including those where one or more participants might attempt to sabotage the signing process. The verification process checks if the signature matches the aggregated public key for a given message, ensuring the integrity of the transaction. Privacy: Since the transaction looks like it was signed by a single key, it's difficult to determine if it's a multi-signature setup, enhancing privacy. Can be used in various scenarios where shared control or higher security is needed, like in escrow services, corporate treasuries, or any scenario where multiple parties need to agree on fund movements. FROST is essentially an extension or variation of MuSig2 designed for threshold signatures, where a subset of participants (t out of n) can sign transactions. FROST aims to provide the same efficiency and privacy benefits as MuSig2 but for threshold signatures, which are crucial for distributed key generation and signing scenarios.Although BIP 327 itself doesn't cover FROST explicitly, its implementation and the underlying principles are directly applicable and necessary for FROST's functionality in Bitcoin. FROST (Flexible Round-Optimized Schnorr Threshold) scheme focusses on **Key Aggregation**
  14. **[DRAFT BIP]** The ChillDKG BIP (Bitcoin Improvement Proposal) is currently in draft form and focuses on implementing a Distributed Key Generation (DKG) protocol specifically designed for use with the FROST (Flexible Round-Optimized Schnorr Threshold) signature scheme. ChillDKG aims to provide a standalone, easy-to-implement DKG protocol that can be used for generating keys for threshold signatures, particularly for FROST. This is crucial for scenarios where multiple parties need to collaboratively generate a shared public key without any single party having the power to sign transactions alone. While the ChillDKG BIP focuses on key generation, it's intended to work in conjunction with another BIP for FROST signing, indicating a broader initiative to enhance Bitcoin's scripting capabilities with threshold signatures. By implementing threshold signatures, ChillDKG contributes to scenarios where higher security or privacy is required, like in multi-signature setups for corporate treasuries or complex smart contracts.Protocol: Based on the SimplPedPop protocol, ChillDKG incorporates modern cryptographic primitives to ensure secure, authenticated, and uncensored communication among participants.Key Generation: Participants engage in a process where they commit to shares of a secret key, followed by a verification phase to ensure all shares are correct, culminating in the generation of a threshold public key.

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
  8. [Portal](https://github.com/TwentyTwoHW/portal-software), [TwentyTwo](https://twenty-two.xyz/)
  9. [Frostsnap](https://frostsnap.com/)

# Reference Talks on Self-Custody, Inheritance, Enterprise Custody, Miniscript and Covenants
    1. INSTITUTIONAL CUSTODY: [24Jan2024]Bitcoin Custody for Institutions w/ Caitlin Long & Wes Knobel (BTC166): [Video](https://www.youtube.com/watch?v=u8UpiYJqCTo)
    2. INHERITANCE: [16Jan2023] Daniela Brozzoni - Life is short, Bitcoin is forever: [Video](https://www.youtube.com/watch?v=QquvK-gMOFk)
    2. BITCOIN VAULTS: [28Aoct2023]Collaborative custody bitcoin vaults for private wealth and enterprise: [Video](https://www.youtube.com/watch?v=0as_B7wfAp0)
    3. SELF-CUSTODY: [22Aug2022] Bitcoin Energy & Custody w/ Parker Lewis & Will Cole: [Video](https://www.youtube.com/watch?v=Xw9kQFJcXds&t=0s)
    4. SELF-CUSTODY: [21Jul2022]The Bitcoin Custody Protocol - Fedimint w/ Obi Nwosu: [Video](https://www.youtube.com/watch?v=X1OlFs2IMgo)  
    5. INHERITANCE: [17Mar2022] Bitcoin Retirement Planning & Self-Custody w/ Parker Lewis & Jeff Vandrew: [Video](https://www.youtube.com/watch?v=vA1bOKbEIzI)
    6. SELF-CUSTODY:[16Jun2021] Bitcoin Security and Self Custody w/ Nick Neuman: [Video](https://www.youtube.com/watch?v=vA1bOKbEIzI)
    7. MINISCRIPT: [10Mar2020] Andrew Poelstra - Bitcoin Script to Miniscript: [Video](https://www.youtube.com/watch?v=_v1lECxNDiM) 
    8. MINISCRIPT: [12Jan2023] CD86: Miniscript with Rob, Vivek, and NVK: [Video](https://www.youtube.com/watch?v=PjCL3tr5VAw) Tags: miniscript, multisig wallets, inheritance planning, corporate treasuries, hardware wallets, tradeoffs 
    9. MINISCRIPT: [24May2023] Miniscript Panel on Opensource stage of Miami Bitcoin 2023 [Video](https://www.youtube.com/watch?v=RiLP79eJvqw&t=12722s)
    10. MINISCRIPT: [23Jan2023]SLP452 Bitcoin Miniscript and what it enables – with Antoine Poinsot & Salvatore Ingala: [Podcast](https://stephanlivera.com/episode/452/)
    11. MINISCRIPT: [6Sep2024]Bitcoin in a Taproot World w/ Nick Farrow, Rob Hamilton & Kody Low: [Video](https://www.youtube.com/watch?v=6FFIY4gMy28)

# Reference Blogs/Articles
    1. MINISCRIPT: [Towards a trustless Bitcoin wallet with miniscript](https://www.ledger.com/blog/towards-a-trustless-bitcoin-wallet-with-miniscript) 
    2. MINISCRIPT: [miniscript is Coming](https://blog.ledger.com/miniscript-is-coming/)

  # Top Technical Leaders Open for Consultations
    1. [Vivek kasarabada](https://x.com/seardsalmon) - Miniscript
    2. https://x.com/darosior - Miniscript
    3. https://x.com/salvatoshi  - Miniscript
    4. Nick Farrow - Frostsnap 
    5. Rob Hamilton - AnchorWatch
    6. Kody Low - Fedi 

