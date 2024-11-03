
# Core Architectural Components of Nunchuk

## Core Library (libnunchuk)
- Written in C++ for performance and portability
- Handles core Bitcoin operations
- Manages wallet state and persistence
- Interfaces with Bitcoin Core

```
Core Components:
├── Chain Interface
├── Wallet Registry
├── Key Management
└── Transaction Builder
```

## Frontend Layers
- Cross-platform GUI using Qt
- Native mobile implementations
- Shared business logic

```
Frontend Components:
├── Desktop GUI (Qt)
├── Mobile Apps
│   ├── iOS
│   └── Android
└── Common UI Components

```

## Key Management System
- Supports various hardware wallets
- Multi-signature coordination
- Key generation/derivation and backup
- Descriptor-based wallets
- Address management
- Key Configuration
    - Multisig setup
    - Threshold configuration
    - Key verification

```
Key Management:
├── Single-sig Support
├── Multi-sig Coordination
├── Hardware Wallet Integration
│   ├── Trezor
│   ├── Ledger
│   └── Coldcard
└── Air-gapped Signing
```

## Transaction Management
- Transaction building: PSBT (Partially Signed Bitcoin Transactions) support
    - Creation and coordination
    - Signature collection
    - Transaction finalization
- Multi-signature coordination
- UTXO management: Transaction history and status tracking 
- Fee estimation
- Signing coordination / Management
    - Hardware wallet integration
    - Air-gapped device support
    -  Remote signer coordination

```
Transaction Module:
├── PSBT Handler
├── Fee Estimation
├── Transaction Builder
└── Signature Aggregator
```

## Security Layer
- End-to-end encryption
- Secure key storage
- Access control

```
Security Components:
├── Encryption Module
├── Secure Storage
├── Authentication
└── Permission Management
```

## Network Layer

- Bitcoin network connectivity
- Blockchain synchronization
- Peer discovery and communication

```
Network Components:
├── Bitcoin Network Interface
├── Electrum Server Connection
└── P2P Communication
```

## Storage Layer
- Persistent wallet data (Database Management)
- Transaction records (SQLite for wallet data) and Encrypted storage
- User preferences (Configuration management)

```
Storage Components:
├── Wallet Database
├── Transaction History
└── Configuration Storage
```

## Business Logic Layer
- Core wallet functionality
- Address generation and management
- Balance calculations

```
Business Logic:
├── Wallet Operations
├── Address Management
├── Balance Tracking
└── Policy Engine
```

## Integration Layer
- Hardware wallet communication
- External service integration
- Backup and recovery systems

```
Integrations:
├── Hardware Wallet Bridge
├── External Services
└── Backup Solutions
```

## Collaboration Features
### Key Sharing
- Secure messaging
- Key exchange protocols
- Verification system

### Transaction Approval
- Multi-party approval flow
- Notification system
- Status tracking

### Inheritance Planning
- Configuration setup
- Recovery protocols
- Time-lock mechanisms

## Team management
Key sharing
Approval workflows
Inheritance protocols

# Key Architectural Principles

## Security-First Design
- Prioritizes security in all components
    - Multi-signature enforcement
    - Key isolation
    - Secure communication
    -  Access control
- Assumes zero trust between components
- Supports air-gapped operations

```
Security Features:
├── Air-gap Support
├── Multi-signature First
└── Zero Trust Architecture
```

## Modular Architecture
- Loose coupling between components
- Easy to extend and modify
- Clear separation of concerns

```
Modularity:
├── Pluggable Components
├── Clear Interfaces
└── Dependency Injection
```

## Cross-Platform Compatibility
- Consistent experience across platforms
- Platform-specific optimizations
- Shared core functionality

```
Platform Support:
├── Desktop (Linux, macOS, Windows)
├── Mobile (iOS, Android)
└── Shared Core Logic
```

## Data Flow
- Clear data flow patterns
- Consistent state management
- Error handling at each layer

```
Data Flow:
User Interface
    ↕️
Business Logic Layer
    ↕️
Core Library (libnunchuk)
    ↕️
Bitcoin Network
```

# Notable Features

## Multi-signature Focus
- Native multi-signature support
- Collaborative signing workflows
- Key holder management

## Hardware Wallet Integration
- Broad hardware wallet support
- Secure signing procedures
- Device management

## Backup and Recovery
- Robust backup systems
- Recovery procedures
- Key backup verification

## Privacy Features
- Coin control
- Address management
- Network privacy options

## Features:

- **UI/UX Experience Layer**: Nunchuk’s UI/UX layer includes both mobile and desktop interfaces that emphasize usability while managing complex multisig interactions.
- **UI/UX Experience Layer**: The interface allows users to set up multisig wallets, view balances, propose transactions, and confirm signatures. It includes features like push notifications for transaction proposals, reminders for pending signatures, and visual prompts for multisig processes, making multisig accessible to non-technical users.
- **UI/UX Experience Layer**: The UI is also designed to be informative and provide educational resources, guiding users through setting up secure configurations, understanding multisig, and executing transactions.
- **UI/UX Experience Layer**: The wallet implements secure communication protocols for signing transactions and coordinating with other cosigners. End-to-end encryption is used for data storage and communication to ensure sensitive data remains confidential and tamper-proof.
- **Security and Authentication**: This component includes authentication features such as biometric login and passcode protection for accessing the wallet, providing additional layers of security for device access.
- **Security and Authentication**: Nunchuk’s architecture also includes monitoring for suspicious activities, with safeguards like time delays and notifications to prevent unauthorized transactions.
- **Communication and Notification System**: This system allows for communication between cosigners in a multisig setup, helping participants to coordinate transaction proposals, approvals, and rejections. This is crucial in collaborative custody setups where different cosigners may be on different devices or in different locations.
- **Communication and Notification System**: The notification system provides real-time alerts for transaction proposals, signature requests, and transaction completion, keeping users informed and minimizing delays.
- **Backup and Recovery Mechanism**: Nunchuk provides robust backup and recovery options, including mnemonic phrases for key recovery and support for encrypted backups of wallet configurations.
- **Backup and Recovery Mechanism**: The wallet may also offer compatibility with popular multisig standards (like BIP45 for multisig wallet backup), allowing users to restore multisig setups without needing to reconfigure each participant’s device.
- **Third-Party Integrations**: Nunchuk integrates with hardware wallets (e.g., Ledger, Trezor) and possibly software-defined HSMs to enable secure signing, especially in multisig workflows where different types of devices are used for key storage.
- **Third-Party Integrations**: It also provides support for watch-only wallets, enabling users to monitor funds without having full access to the signing keys. This is useful for cosigners who need visibility into wallet balances and transactions but aren’t involved in every signing operation.
- **Network and Blockchain Interface**: It includes logic for broadcasting transactions, fetching unspent transaction outputs (UTXOs), and monitoring confirmations, providing real-time information about transaction statuses.
- **Network and Blockchain Interface**: Nunchuk interacts with the Bitcoin blockchain via a network interface layer. This component manages communication with Bitcoin nodes and allows users to connect to a variety of sources, including their own full node or external block explorers for transaction verification.
- **Collaborative Custody and Multisig**: This component is designed to facilitate collaborative custody, where multiple parties manage a shared wallet using a multisig setup. It enables users to set up, manage, and coordinate signing between multiple participants.
- **Collaborative Custody and Multisig**: The wallet uses PSBT (Partially Signed Bitcoin Transactions) to coordinate signing without revealing private keys. This setup enables shared control among multiple signatories, supporting 2-of-3 and higher multisig configurations commonly used for family, business, or institutional security.
- **Collaborative Custody and Multisig**: The component includes interfaces for proposing, signing, and finalizing transactions, with notifications and communication features to streamline multisig coordination.
- **Secure Key Management**:  Secure key management is a critical component that handles the generation, storage, and usage of private keys. In Nunchuk, key management follows principles of self-custody, where private keys are stored locally on users’ devices. 
- **Secure Key Management**: It supports cold storage, meaning users can create multisig setups where one or more keys are kept offline. Integration with hardware wallets and air-gapped devices is also part of this component, ensuring flexibility and security for different setups.
- **Secure Key Management**: The wallet incorporates mnemonic-based recovery for secure key backup and restoration, ensuring that users can recover their funds if they lose access to their devices.

