# Architectural components of Sparrow Desktop Wallet:

## Core Layer
    - Written in Java
    - Manages communication between all modules
    - Handles application lifecycle
    - Implements core business logic

## Wallet Module
#### Key Management System
    - HD wallet implementation (BIP32/39/44)
    - Key derivation and storage
    - Address generation and management

#### Transaction Builder
    - UTXO management and selection
    - Fee estimation/calculation
    - PSBT (Partially Signed Bitcoin Transaction) creation / PSBT Handler
    - Script engine for transaction validation

## Network Module
#### Multiple Backend Support
    - Bitcoin Core RPC interface
    - Electrum protocol implementation
    - Tor integration for privacy

#### Node Management
    - Connection handling
    - Block synchronization
    - Transaction broadcasting

## Storage Module
#### Database Management
    - SQLite for transaction history
    - UTXO set maintenance
    - Address indexing

#### Configuration
    - User preferences
    - Network settings
    - Wallet configurations

## Security Module
#### Encryption Engine
    - Wallet encryption
    - Secure key storage

#### Hardware Wallet Integration
    - Support for multiple hardware wallets
    - Communication protocols
    - Signature validation

## UI Layer (JavaFX)
#### Wallet Dashboard
    - Balance display
    - Transaction history
    - Address management

#### Transaction Interface
    - Send/Receive functionality
    - PSBT creation and signing
    - Fee management

#### Settings and Configuration
    - Network settings
    - Backup/restore
    - Preferences management

# Key Technical Characteristics:

## Modularity
   - Clear separation of concerns
   - Loosely coupled components
   - Pluggable architecture for different backends

## Security Features
   - Deterministic builds
   - Air-gap support
   - Watch-only wallet functionality
   - Hardware wallet integration

## Secure Integration
   - Multiple hardware wallet support
   - Encryption at rest
   - Secure transaction signing

## Performance Considerations
   - Async operations for UI responsiveness
   - Efficient UTXO management
   - Optimized block scanning

## Data Flow
   - Event-driven architecture
   - Reactive programming patterns
   - Thread safety considerations

```mermaid

graph TD
    %% Main Application Entry
    Client[JavaFX UI Client] --> Core[Sparrow Core Controller]
    
    %% Core Components
    Core --> WalletM[Wallet Module]
    Core --> NetworkM[Network Module]
    Core --> SecurityM[Security Module]
    Core --> StorageM[Storage Module]
    
    %% Wallet Module Details
    WalletM --> KeyMan[Key Management]
    WalletM --> TxBuilder[Transaction Builder]
    WalletM --> PSBTMan[PSBT Handler]
    
    KeyMan --> HDWallet[HD Wallet Manager]
    KeyMan --> KeyDeriv[Key Derivation]
    KeyMan --> AddrGen[Address Generator]
    
    TxBuilder --> UTXOMan[UTXO Manager]
    TxBuilder --> FeeCalc[Fee Calculator]
    TxBuilder --> TxCompose[Transaction Composer]
    
    %% Network Module Details
    NetworkM --> BTCCore[Bitcoin Core RPC]
    NetworkM --> Electrum[Electrum Protocol]
    NetworkM --> TorInt[Tor Integration]
    NetworkM --> NodeMan[Node Manager]
    
    BTCCore --> Network((Bitcoin Network))
    Electrum --> Network
    TorInt --> Network
    
    %% Security Module Details
    SecurityM --> Encrypt[Encryption Engine]
    SecurityM --> SigValid[Signature Validator]
    SecurityM --> HWBridge[Hardware Wallet Bridge]
    
    HWBridge --> Trezor[Trezor Interface]
    HWBridge --> Ledger[Ledger Interface]
    HWBridge --> Coldcard[Coldcard Interface]
    
    %% Storage Module Details
    StorageM --> SQLite[(SQLite DB)]
    StorageM --> FileSystem[(File System)]
    StorageM --> Config[Config Manager]
    
    %% UI Components
    Client --> Dashboard[Wallet Dashboard]
    Client --> TxUI[Transaction UI]
    Client --> UTXOUI[UTXO Manager UI]
    Client --> Settings[Settings Panel]
    
    %% Data Flow Connections
    Dashboard --> WalletM
    TxUI --> TxBuilder
    UTXOUI --> UTXOMan
    Settings --> Config
    
    %% Subgraphs for logical grouping
    subgraph User Interface Layer
        Client
        Dashboard
        TxUI
        UTXOUI
        Settings
    end
    
    subgraph Core Services
        WalletM
        NetworkM
        SecurityM
        StorageM
    end
    
    subgraph Hardware Integration
        Trezor
        Ledger
        Coldcard
    end
    
    %% Styling
    classDef primary fill:#f9f,stroke:#333,stroke-width:2px
    classDef secondary fill:#bbf,stroke:#333,stroke-width:2px
    classDef storage fill:#dfd,stroke:#333,stroke-width:2px
    classDef ui fill:#ffd,stroke:#333,stroke-width:2px
    
    class Client,Core primary
    class WalletM,NetworkM,SecurityM,StorageM secondary
    class SQLite,FileSystem storage
    class Dashboard,TxUI,UTXOUI,Settings ui

    ```