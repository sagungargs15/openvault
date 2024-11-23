https://github.com/ProtonWallet/andromeda

The high-level software architecture of Proton's Andromeda Bitcoin Wallet, focusing on its privacy-centric and self-custody approach.

# About Proton's Andromeda
The andromeda libraries aims to provide logical blocks to build a privacy-focused, cross-platform, self-custody Bitcoin and Lightning wallet, integrated in Proton's ecosystem

## Core Architectural Components

### Bitcoin Core Layer (bitcoin crate)
Bitcoin Components:
├── Chain Syncing
├── Transaction Management
│   ├── Transaction Building
│   ├── Signing
│   └── Broadcasting
└── Address Management
    ├── Generation
    └── Balance/UTXO Tracking
### API Integration Layer (api crate)
API Components:
├── Proton Wallet Backend
│   ├── HTTP Client
│   └── API Interfaces
└── Service Integration
### WASM Interface Layer (wasm crate)
WASM Components:
├── Cross-platform Bindings
├── Browser Integration
└── WASM-specific Interfaces

## Key Technical Features

### Privacy Framework
Privacy Components:
├── Transaction Privacy
├── Network Privacy
└── Data Protection

### Cross-platform Support
Platform Support:
├── Native Applications
├── Web Integration (WASM)
└── Shared Core Logic

## Security Architecture

### Self-custody Implementation
Custody Features:
├── Key Management
├── Secure Storage
└── Transaction Control

### Security Layer
Security Components:
├── Encryption
├── Key Protection
└── Access Controls

## Integration Components

### BDK Integration
Bitcoin Development Kit:
├── Wallet Operations
├── Chain Synchronization
└── Transaction Building

### Proton Ecosystem
Ecosystem Integration:
├── Backend Services
├── API Communication
└── State Management

## Development Infrastructure

### Build System
Build Components:
├── Native Build
├── WASM Compilation
│   ├── wasm-pack
│   └── WebAssembly Output
└── Cross-platform Tools

### Project Structure
Codebase Organization:
├── Core Libraries (/crates)
│   ├── api
│   ├── bitcoin
│   └── wasm
└── Examples
    └── CLI Implementation

## Key Design Principles

### Privacy Focus
Privacy Measures:
├── Transaction Privacy
├── Network Privacy
└── User Data Protection

### Cross-platform Architecture
Platform Strategy:
├── Native Performance
├── Web Compatibility
└── Consistent Experience

## Notable Features

### Bitcoin Operations
    Chain synchronization
    Transaction management
    Address generation
    UTXO tracking
### Privacy Features
    Privacy-focused design
    Secure transaction handling
    Protected user data
### Integration Capabilities
    Proton ecosystem integration
    BDK integration
    API services
### This architecture enables Andromeda to provide:
    Strong privacy protection
    Self-custody solutions
    Cross-platform support
    Ecosystem integration
    Secure Bitcoin operations

### The modular design allows for:
    Easy maintenance
    Feature additions
    Security updates
    Platform expansion
    Integration flexibility

### Key differentiators:
    Privacy-first approach
    Rust implementation
    WASM support
    Proton ecosystem integration
    Self-custody focus
### The architecture emphasizes:
    Privacy protection
    Cross-platform compatibility
    Security best practices
    Modular design
    Ecosystem integration
