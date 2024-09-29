# Stitching MVP

## Approach 1: Subzero
	•	**Bitcoin Chain Interations**: [Rust-Bitcoin](https://github.com/rust-bitcoin/rust-bitcoin): For Bitcoin blockchain interactions.
	•   **Cryptographic operations**: [RustCrypto](https://github.com/RustCrypto): For cryptographic operations.
	•   **HSM Integration**: A hardware security module (HSM) is a dedicated crypto processor that is specifically     designed for the protection of the crypto private key lifecycle i.e the private keys that are used for proving the Bitcoin Ownership or for moving teh actual Bitcoins. 
            1. [Yubihsm/SoftHSM](https://github.com/iqlusioninc/yubihsm.rs): For hardware or software-defined HSM integration.
            2. [hsm-crypto](https://github.com/kanidm/hsm-crypto) - A library for cryptographic operations backed by a HSM or TPM Resources. This library allows the use of HSM's, TPM's or SoftHSM's in cryptographic applications. The goal is to simplify interactions with these devices so that applications can utilise these
            3.[Cryptoki](https://github.com/parallaxsecond/rust-cryptoki) -  Rust wrapper for the PKCS #11 API
            4. [AWS Cloud HSM](https://github.com/awslabs/aws-sdk-rust/tree/855a713dee202551e9752ed1272e48d1b6041a44/sdk/cloudhsmv2) - More details read [here](https://aws.amazon.com/cloudhsm/faqs/)
    •   **Auditing & Logging**: Rust’s logging crates such as log or tracing
    •   **A Bitcoin cold storage enterprise wallet by Square**: [Subzero](https://github.com/square/subzero) 
        The Subzero repository, Square’s cold storage solution for Bitcoin enterprise custody, is composed of several key components
            1.	**Coordinator Service**: This manages unspent transaction outputs (UTXOs), initiates signing ceremonies, and merges signatures. It coordinates between the core and gateway wallets.
            2.	**Core**: Written in C, this component runs inside the Hardware Security Module (HSM). It handles encryption/decryption using specific mechanisms and communicates with external systems via a TCP socket, using protobuf-encoded commands.
            3.	**User Interface (UI)**: This Java-based UI communicates with the Core over TCP, encoding commands and rendering graphics via a Linux framebuffer. The UI also manages utility functions like signature merging and transaction building, especially for Segwit and multisig transactions.
            4.	**QR Codes**: Subzero uses custom QR codes containing base45-encoded protobufs for triggering operations and reading results.
            5.	**Live USB Creator**: Scripts for generating a DVD containing signed artifacts of the Core and UI code. The entire setup can be booted offline from a DVD.
            6.	**Beancounter**: A Go program used to audit the balances of both gateway and cold wallets.

## Approach 2: Caravan

## Approach 3: Liana