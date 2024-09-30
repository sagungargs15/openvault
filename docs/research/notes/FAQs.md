# FAQs on Engineering an Bitcoin Custody Enterprise Wallet

## Bitcoin Wallet Projects in Rust typically don't use full-fledged web frameworks as their backend ?
 There are several reasons for the same:-

1. Specialized Requirements: Bitcoin projects often have very specific and specialized needs that don't align well with general-purpose web frameworks. They deal with cryptographic operations, blockchain interactions, and network protocols that are unique to cryptocurrencies.

2. Performance: Bitcoin operations can be computationally intensive. Full-fledged web frameworks often come with overhead that may not be necessary for Bitcoin applications, potentially impacting performance.

3. Security: Bitcoin projects handle sensitive financial data and require a high level of security. Using a minimal, purpose-built backend allows for more control over the security aspects and reduces the attack surface.

4. Low-level Control: Many Bitcoin operations require low-level control over data structures, network protocols, and system resources. Web frameworks often abstract away these details, which can be counterproductive for Bitcoin development.

5. Minimal Dependencies: Bitcoin projects often aim to minimize external dependencies to reduce potential vulnerabilities and maintain better control over the codebase.

6. Focus on Core Functionality: Most Bitcoin projects focus on core cryptocurrency functionality rather than serving web content or providing a web interface. When web interfaces are needed, they're often implemented as separate components.

7. Offline Capability: Many Bitcoin applications need to function offline or in limited-connectivity environments, which doesn't align well with web-centric frameworks.

8. Interoperability: Bitcoin projects often need to interact directly with the Bitcoin network, other blockchain systems, or specialized hardware. This is easier to manage with custom, low-level code rather than through a web framework.

9. Resource Efficiency: Bitcoin nodes and wallets may need to run on devices with limited resources. Full web frameworks can be resource-intensive and unnecessary for such use cases.

# FAQs on Bitcoin 

## What is Bitcoin Vault how is it different from Bitcoin Wallet ?
A vault is a wallet requiring at least 2 steps to spend a transaction:
- unlock/unvault
- spend
In-between these 2 steps, there is a revault option that can be triggered, effectively preventing the spend.

## What is Bitcoin Wallet ?
What we commonly call a 'wallet' is in fact a descriptor, the descriptor represent the set of spending rules for your coins, if you want to changes this rules yu need to create a new descriptor(wallet). 

## What is an HSM ? 
A hardware security module (HSM) is a dedicated crypto processor that is specifically designed for the protection of the crypto private key lifecycle i.e the private keys that are used for proving the Bitcoin Ownership or for moving teh actual Bitcoins. A hardware security module (HSM) provides secure key storage and cryptographic operations within a tamper-resistant hardware device. HSMs are designed to securely store cryptographic key material and use the key material without exposing it outside the cryptographic boundary of the hardware.

## What is a Software Defined Hardware Security Module (SD-HSM) ?
A Software Defined Hardware Security Module (HSM) is a virtualized or emulated HSM that provides the functionalities of a physical HSM but operates entirely in software. Unlike traditional physical HSMs that involve dedicated hardware devices to perform cryptographic operations securely, a software-defined HSM replicates these functions on general-purpose hardware, often in a cloud or containerized environment.
    1.	**Cryptographic Operations**: Provides secure key storage, encryption, decryption, and signing operations.
	2.	**Scalability **: Easier to scale than physical HSMs since they can be deployed in cloud environments.
	3.	**Flexibility **: Offers flexibility in deployment, can be integrated into DevOps pipelines, and can be containerized for use in microservice architectures.
	4.	**Cost-Effective **: Reduces the cost and logistical complexity associated with physical HSMs, though it often comes at a trade-off in physical security. 

## What is HSM Coldstorage
For security, OpenVault will store a reserve of Bitcoins in an offline setting. By having these funds offline, it will reduce attack surface and risk of theft.

OpenVault's solution is unique, specifically, it can leverage FIPS certified Hardware Security Modules (HSMs) to protect the private key material. It currently supports certain HSMs to kickstart the usecase often operated and trusted devices bu biggest financial payment-related players in the USA market

Funds can be sent from online systems to the cold storage at any time. Moving funds out of cold storage requires a multi-party signing ceremony. In addition, the offline HSMs are able to enforce business logic rules, for instance it can only allow sending funds to OpenVault's Bitcoin addresses whitelisted for the given institution operating it (More like Policy level Soft version of Covenants without upgrading to new BIP). Such a scheme is usually called defense in depth or an onion model. Openvault maintain's the online/offline isolation by importing transaction metadata and exporting signatures using QR codes. HSMs have the ability to share key material. This enables the ability to store in Openvault's backup architecture in a encrypted form and restore a wallet at any location. Multi-institution setup is also a big piece of this capability

This repo contains our design documents as well as specific technical information. We are sharing our source code, with the caveat that the code is currently only useful if you have the exact same hardware setup. We are willing to make the code more modular over time, as long as the broader community shows interest to implement support for additional hardware vendors.

## More FAQs
https://www.reddit.com/r/Bitcoin/comments/1bo5wb8/liana_wallet_gathering_some_feedback/
