Ref: https://github.com/square/subzero

The high-level software architecture of Square's Subzero Bitcoin Cold Storage solution, based on the repository analysis.

# Core Architectural Components

## HSM Integration Layer

HSM Components:
├── FIPS Certified HSM Interface
├── Key Management System
└── Security Policy Enforcement
    ├── Business Rules
    └── Transaction Validation

## Cold Storage Core

Core Components:
├── Wallet Management
│   ├── Key Generation
│   └── Key Sharing
├── Transaction Handling
│   ├── Signing Ceremony
│   └── Multi-party Validation
└── QR Code Interface
    ├── Transaction Import
    └── Signature Export

## Security Architecture

Security Layers:
├── Hardware Security
│   ├── FIPS HSM Integration
│   └── Offline Operations
├── Protocol Security
│   ├── Multi-signature Support
│   └── Business Logic Rules
└── Backup Security
    ├── Encrypted Backups
    └── Key Material Sharing

# Key Functional Modules

## Transaction Processing

Transaction Flow:
├── QR Code Input
├── Transaction Validation
├── Multi-party Signing
└── Signature Export

## Backup System
Backup Components:
├── Key Material Backup
├── Encrypted Storage
└── Wallet Restoration

# Notable Security Features

## Defense in Depth

Security Layers:
├── Hardware Security (HSM)
├── Offline Operation
├── Multi-party Signing
└── Business Logic Rules

## Key Management
Key Operations:
├── Secure Generation
├── Encrypted Storage
├── Key Sharing
└── Backup Management

# Integration Components

## Hardware Integration

HSM Interface:
├── Device Communication
├── Command Processing
└── Response Handling

## Data Transfer

Data Flow:
├── QR Code Input
│   ├── Transaction Data
│   └── Metadata
└── QR Code Output
    └── Signatures

# Development Infrastructure

## Core Implementation
Language Distribution:
├── C (54.1%)
├── Java (35.8%)
└── Supporting Languages
    ├── Shell (2.9%)
    ├── Python (2.3%)
    └── Others

## Testing Framework

Testing Components:
├── Unit Tests
├── Integration Tests
└── Security Validation

# Key Design Principles

## Security First
Security Priorities:
├── Hardware Security
├── Air-gap Maintenance
└── Multi-party Authorization

## Operational Safety

Safety Measures:
├── Offline Operations
├── Transaction Validation
└── Business Rules

# Notable Features
## Cold Storage Management
    FIPS certified HSM integration
    Offline transaction signing
    Multi-party authorization
## Security
    Hardware-based security
    Air-gapped operations
    Defense in depth approach
## Backup & Recovery
    Encrypted backups
    Key material sharing
    Wallet restoration capabilities

## This architecture enables Subzero to provide:
    Enterprise-grade security
    Operational reliability
    Secure key management
    Transaction safety
    Backup redundancy

## The modular design allows for:
    Hardware vendor flexibility
    Security policy updates
    Feature additions
    Performance optimization

## Key differentiators:
    Use of FIPS certified HSMs
    QR code-based air-gap maintenance
    Multi-party signing ceremonies
    Enforced business logic rules
    Square-owned address validation