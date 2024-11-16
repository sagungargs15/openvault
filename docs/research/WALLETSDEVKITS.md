# DevKits/Protocols/DevTools
  1. **[Revault Protocol](https://github.com/revault/revaultd)**: Advanced Bitcoin custody for organizations. It is a Bitcoin vault architecture for multi-party situations. Revault's design philosophy aligns with using what's currently available in Bitcoin's protocol, showcasing how advanced custody solutions can be built without waiting for future protocol upgrades like covenants. This approach demonstrates the flexibility and power of Bitcoin's existing features when creatively applied to solve real-world problems like secure custody.The Revault architecture primarily focuses on using Bitcoin's native features like multi-signature transactions, time-locks, and script capabilities to create a robust custody solution. It doesn't introduce new BIPs but rather innovatively combines existing functionalities:
      
      a. Multi-signature Transactions: Utilizes Bitcoin's native support for multi-sig, which doesn't require a specific BIP but is enabled by Bitcoin's scripting language.
      
      b. Time-locks: Uses Bitcoin's OP_CHECKLOCKTIMEVERIFY (CLTV) and potentially OP_CHECKSEQUENCEVERIFY (CSV) for creating time-locked transactions, which are integral to Revault's emergency and unvault mechanisms.
      
      c. Scripting: While not a BIP, Bitcoin's scripting language allows for the creation of complex conditions for spending, which Revault leverages for its transaction types (Spend, Cancel, Unvault Emergency).
    
  2. **[Bitcoin Dev Kit](https://bitcoindevkit.org/)**, **[BDK](https://github.com/bitcoindevkit/bdk)** : A modern, lightweight, descriptor-based wallet library written in Rust. BDK includes functionalities for wallet creation, transaction building, and blockchain interaction. It's built to be easy to integrate into applications, focusing on usability and security. 
  3. **[Elephant](https://github.com/bitcoindevkit/elephant)** : Elephant is a Bitcoin Wallet designed to demonstrate the power of Bitcoin policies in a sandbox environment.
  4. **[Miniscript Rust SDK](https://docs.rs/miniscript/latest/miniscript/)**, **[github](https://docs.rs/miniscript/latest/miniscript/)** :Support for [Miniscript](https://bitcoin.sipa.be/miniscript/) and Output Descriptors for rust-bitcoin. This library supports the creation and manipulation of Bitcoin scripts using the Miniscript language, which is particularly useful with Taproot's introduction. Miniscript allows for the creation of modular script components. For instance, you can define a "threshold" component where m of n keys must sign, and then reuse this in different parts of your hierarchical structure.  Miniscript provides tools to verify that the script does what it's supposed to do, reducing the risk of bugs or vulnerabilities in complex scripts. By using Miniscript, you leverage its built-in checks to ensure that your scripts are secure against common scripting errors or exploits. Miniscript enhances Bitcoin's scripting capabilities by allowing for the creation of complex, yet manageable, custody structures like key hierarchies
        a. Multi-signature with Key Hierarchy: Miniscript can define scripts where different keys or combinations of keys from different hierarchical levels are required to spend funds. This could be used for governance structures where different levels of approval are needed based on the amount or type of transaction. Example: A setup where small transactions can be signed by a single key from a lower level in the hierarchy, but larger transactions require signatures from multiple keys across different levels.
  5. **[Miniscript Templates](https://github.com/Blockstream/miniscript-templates)**: About Templates for Miniscript-based spending policies. For use by Bitcoin wallet developers and users.
  6. Other Bitcoin or Rust Projects for reference
    a. **[Rust Bitcoin library](https://rust-bitcoin.org)**, **[github](https://github.com/rust-bitcoin/rust-bitcoin)** :The rust-bitcoin crate is a library that supports the Bitcoin network protocol and associated primitives. It is designed for Rust programs built to work with the Bitcoin network. Rust-Bitcoin is a low-level library that provides bindings to Bitcoin's core primitives like script, network, and blockchain data structures. It's more of a foundational library upon which other, more user-friendly tools like BDK are built. It's essential for developers who need fine-grained control over Bitcoin operations.
    b. **[Rust for Bitcoiners](https://github.com/bitcoin-dev-project/rust-for-bitcoiners)**: The theme is "Rust for Bitcoiners", so the examples are created in such a way that it is relevant to Bitcoin technology. It consists of a collective of teachings
    c. **[Rust Bitcoin Community](https://github.com/rust-bitcoin)** :A Series of Projects to implement various Bitcoin Protocols in Rust
    d. **[Fedimint](https://github.com/fedimint/fedimint)**: Federated E-Cash Mint
    e. **[Bitcoin Rust Book by Braiins](https://github.com/bitcoin-dev-project/bitcoiner-intro-to-rust)**: Learn Rust and review Bitcoin fundamentals by building a command-line program.
    f. **[Unofficial Rust code and resources](https://github.com/rust-unofficial/awesome-rust)**: A curated list of Rust code and resources.
    g. **[Rust backend web framework](https://medium.com/deno-the-complete-reference/the-fastest-rust-backend-web-framework-in-2024-19e2967e8d8e)**: 
        (i) [Rocket](https://github.com/rwf2/Rocket)
        (ii) [Actix](https://github.com/actix/actix-web)
        (iii) [Axum](https://github.com/tokio-rs/axum) 
        (iv) [Warp](https://github.com/seanmonstar/warp)
        (v) [Gotham](https://github.com/gotham-rs/gotham)
    h. **[SQL ORM for Rust Backend]()**: An async & dynamic ORM for Rust
        (i) [Sea ORM](https://github.com/SeaQL/sea-orm): SeaORM is a relational ORM to help you build web services in Rust with the familiarity of dynamic languages.
        (ii) [Diesel](https://github.com/diesel-rs/diesel): Diesel gets rid of the boilerplate for database interaction and eliminates runtime errors without sacrificing performance. It takes full advantage of Rust's type system to create a low overhead query builder that "feels like Rust.

**Summary**: When choosing an SDK or library for Bitcoin development in Rust, consider:
1. BDK for high-level wallet and transaction management.
2. Rust-Bitcoin for lower-level operations or if you need to work directly with Bitcoin's core protocols.
3. Elements or Rust Lightning if you're exploring sidechains or the Lightning Network.