# Architectural components of Specter Desktop:

### Core Components
Python-based architecture
Flask web framework
Bitcoin Core RPC integration
Device and wallet management

### Device Manager
Hardware Wallet Interface (HWI)
Device detection and communication
Support for multiple hardware wallets
Key extraction and management

### PSBT Handling
Transaction signing coordination
Hardware wallet communication protocols
Key verification

### Wallet Manager
#### Multiple Wallet Types
 - Single signature wallets
 - Multisignature wallets (n-of-m)
 - Watch-only functionality

#### Transaction Management
 - UTXO tracking
 - Address generation
 - Transaction composition
 - PSBT coordination

### Bitcoin Core Interface

#### RPC Communication
 - Direct Bitcoin Core integration
 - Transaction broadcasting
 - Block data retrieval

#### Node Management
 - Connection handling
 - Blockchain synchronization
 - Network status monitoring

### Web Interface (Flask)

#### Backend Services
 - REST API implementation
 - User authentication
 - Session management

#### Frontend
 - Responsive web interface
 - JavaScript-based interaction
 - Real-time updates

### Storage Layer
#### File-based Storage
 - JSON configuration files
 - Device information
 - Wallet data
 - User preferences

## Key Technical Features:

### Security Architecture
Air-gapped signing support
Multisignature coordination
Watch-only wallet capability
Deterministic builds

### Integration Capabilities
Bitcoin Core dependency
HWI for hardware wallet support
External tool integration

### Design Patterns
MVC architecture in Flask
Event-driven device communication
Repository pattern for storage

### Notable Characteristics

Emphasis on privacy
Focus on cold storage
Multisig-first approach
Self-hosted solution

## Implementation Details:

### Core Technologies
Python (primary language)
Flask (web framework)
Bitcoin Core RPC
SQLite (where needed)
JavaScript (frontend)

### Communication Flow
REST APIs for client-server interaction
WebSocket for real-time updates
RPC for Bitcoin Core communication

### Security Considerations
Local-only web interface
Encryption of sensitive data
Secure key handling
Authentication mechanisms

```mermaid

graph TD
    %% Main Components
    Client[Client Layer] --> API[API Gateway]
    API --> AuthS[Auth Service]
    API --> WalletS[Wallet Service]
    API --> TxS[Transaction Service]
    API --> NetworkS[Network Service]
    
    %% Authentication Service Details
    AuthS --> |JWT| Auth_DB[(Auth DB)]
    AuthS --> |2FA| Security[Security Module]
    
    %% Wallet Service Details
    WalletS --> KeyManager[Key Management]
    WalletS --> WalletDB[(Wallet DB)]
    KeyManager --> HSM[Hardware Security Module]
    KeyManager --> |Seeds/Keys| SecureStore[(Encrypted Storage)]
    
    %% Transaction Service Details
    TxS --> UTXO[UTXO Manager]
    TxS --> TxBuilder[Transaction Builder]
    TxS --> FeeCalc[Fee Calculator]
    TxS --> TxDB[(Transaction DB)]
    
    %% Network Service Details
    NetworkS --> NodeConn[Node Connection]
    NetworkS --> P2P[P2P Network]
    NetworkS --> Mempool[Mempool Monitor]
    
    %% Auxiliary Services
    WalletS --> BackupS[Backup Service]
    BackupS --> CloudStore[(Encrypted Backup)]
    
    %% Security Layers
    Security --> Encryption[Encryption Module]
    Security --> Signing[Signing Module]
    
    %% External Integrations
    P2P --> Bitcoin((Bitcoin Network))
    NodeConn --> Bitcoin
    
    %% Subgraphs for logical grouping
    subgraph Security Layer
        HSM
        SecureStore
        Encryption
        Signing
    end
    
    subgraph Data Layer
        Auth_DB
        WalletDB
        TxDB
        CloudStore
    end
    
    %% Styling
    classDef primary fill:#f9f,stroke:#333,stroke-width:2px
    classDef secondary fill:#bbf,stroke:#333,stroke-width:2px
    classDef storage fill:#dfd,stroke:#333,stroke-width:2px
    
    class Client,API primary
    class AuthS,WalletS,TxS,NetworkS secondary
    class Auth_DB,WalletDB,TxDB,CloudStore,SecureStore storage

```