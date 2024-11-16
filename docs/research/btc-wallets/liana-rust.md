# Architectural components of Liana:

## Core Components (Rust-based)
- Script Engine
- Wallet Management
- Network Interface
- Storage System
- GTK-based UI

## Script Engine: Core Library (liana-core)

```
Core Components:
├── Descriptor Management
│   ├── Miniscript Support
│   └── Timelock Logic
├── Recovery Paths
│   ├── Primary Path
│   └── Recovery Path (with Timelock)
└── Wallet Operations
```

### Miniscript Implementation
- Script generation and analysis
- Policy compilation
- Spending condition validation

### Policy Management
- Recovery paths definition
- Spending conditions
- Policy verification

### Timelock Handling
- Timelock calculations
- Recovery mechanisms
- Time-based constraints

## Wallet Module
### Key Management
- Descriptor-based wallet
    - Complex spending condition management
    - Timelock implementation
    - Miniscript policy compilation

        ```
        Descriptor System:
        ├── Primary Spending Path
        ├── Recovery Path
        │   ├── Timelock Conditions
        │   └── Recovery Key Management
        └── Miniscript Policies
        ```
- Key generation/derivation
- Address management

### Transaction Handling

- UTXO management
- PSBT creation/signing
- Fee estimation
- Transaction building
        ```
        Transaction Module:
        ├── PSBT Handler
        ├── Fee Management
        ├── Transaction Builder
        └── Signing Coordinator
            ├── Hardware Wallet Integration
            └── Recovery Key Signing
        ```

## Network Layer

```
Network Components:
├── Bitcoin Core RPC
├── Blockchain Synchronization
└── Network Configuration
    ├── Mainnet
    └── Testnet
```

### Bitcoin Core Integration
- RPC communication
- Block scanning
- Transaction broadcasting

### Network Protocol Support
- Electrum protocol
- P2P networking
- Node management

## Storage System

```
Storage Components:
├── Wallet Database (sled)
├── Configuration Storage
└── Transaction History
```

### Database Management
- SQLite for persistent storage
- Transaction history
- UTXO set maintenance

### Configuration
- Wallet settings
- Network preferences
- Script templates

## Recovery System

```
Recovery Components:
├── Timelock Management
├── Recovery Path Setup
└── Inheritance Planning
    ├── Key Distribution
    └── Recovery Instructions
```

## UI Components (GTK)

```
UI Components:
├── GTK Interface
├── State Management
└── User Interaction Flows
    ├── Wallet Creation
    ├── Transaction Building
    └── Recovery Planning
```

### Wallet Dashboard
- Balance display
- Transaction history
- Address management

### Script Management
- Policy creator
- Timelock configuration
- Recovery setup

# Key Technical Features:
## Security Architecture
- Timelock-focused design
- Multiple recovery paths
- Miniscript policies
- Hardware wallet support

## Rust Benefits
- Memory safety
- Thread safety
- Performance optimization
- Strong type system

## Design Patterns
- Modular architecture
- Event-driven design
- Factory pattern

## Notable Characteristics
- Descriptor wallet support
- Miniscript integration
- Timelock specialization
- Recovery-focused design

# Implementation Details:

## Core Technologies
- Rust (primary language)
- GTK (UI framework)
- SQLite (storage)
- Bitcoin Core RPC
- Miniscript

## Security Considerations
- Timelock enforcement
- Key isolation
- Policy validation
- Recovery mechanisms

## User Experience
- Intuitive policy creation
- Recovery path visualization
- Timelock management
- Transaction composition

## Security Architecture

```
Security Components:
├── Key Management
│   ├── Primary Keys
│   └── Recovery Keys
├── Hardware Wallet Bridge
└── Secure Storage
```

## Recovery-Focused Architecture
```
Recovery Features:
├── Timelock Management
│   ├── CSV (CheckSequenceVerify)
│   └── CLTV (CheckLockTimeVerify)
├── Recovery Path Setup
└── Inheritance Planning
```

## Descriptor-Based Design

```
Descriptor Architecture:
├── Miniscript Policies
├── Timelock Integration
└── Multi-signature Support
```

## Key Technical Characteristics

### Rust Implementation
```
Rust Features:
├── Memory Safety
├── Thread Safety
└── Zero-cost Abstractions
```

### State Management
```
State Handling:
├── Wallet State
├── Transaction State
└── Recovery State
```

## Data Flow Architecture
```
Data Flow:
User Interface (GTK)
    ↕️
Liana Core
    ↕️
Bitcoin Core
    ↕️
Bitcoin Network
```

## Exceptional Features:

 - **Policy and Script Management**: Script Templates: Users can create custom policies for transactions based on factors like time locks or multisig requirements. Liana uses Bitcoin’s scripting language to enforce these conditions, allowing users to set up flexible custody and recovery policies in case of emergencies.
 - **Policy and Script Management**: A unique feature in Liana, policy and script management enables users to define advanced spending conditions, such as time-based policies, that can trigger under specific circumstances.
 - **UI/UX Experience Layer**: Guided Policy Setup: The interface helps users define their policies, create time locks, and establish multisig rules through a step-by-step approach. It also provides notifications, transaction history, and status indicators to make complex workflows more accessible.
 - **UI/UX Experience Layer**: Monitoring Polciies: The desktop application has an intuitive layout that makes it easy for users to configure and monitor their policies, view wallet balances, and conduct transactions
 - **UI/UX Experience Layer**: Multisig Coordination: For multisig setups, this component manages the process of gathering signatures from multiple cosigners. It enables users to create, review, sign, and finalize transactions according to the predefined rules in the policy management layer.
 - **Backup and Recovery Mechanism**: Policy Backup: For wallets with custom spending policies, Liana supports structured backups that include the full policy setup, making it possible to restore complex multisig or time-locked configurations without manual re-entry.
 - **Security**: Data Encryption and Privacy: Sensitive information, like private keys and transaction data, is encrypted and stored securely on the user’s device. Liana ensures that private keys are never exposed during transaction signing, maintaining user privacy and security throughout the process.
 - **Network Interface**: Connection Flexibility: Users can configure the wallet to connect to their own full node or a trusted third-party node, enabling a balance between privacy and convenience. This interface also allows syncing the wallet state with the current blockchain, fetching the balance, and validating transactions.
- **Core Wallet Engine**: Multisig and Time-Locked Contracts: A defining feature of Liana is its support for time-based spending policies, allowing users to set up configurations where funds can be accessed under different conditions, such as after a certain period. This is implemented using Bitcoin Script and OP_CHECKSEQUENCEVERIFY (CSV), which enables relative time locks.