# FAQs on Bitcoin

## Why run, operate and control your own Bitcoin Custody Infrastructure ?
You should run, operate and control your own custody infrastructure to minimize risks to your clients’ assets. This is different from many other companies in the industry who use shared custodians, which carries a few risks many inexperienced users may not be aware of. A shared custodian often supports many risky business clients and assets, which could lead to insolvency in some cases. This may not lead to a loss of funds for a client, but could freeze their assets for a long period of time until these issues are resolved. 

## What is Bitcoin Vault how is it different from Bitcoin Wallet ?
A vault is a wallet requiring at least 2 steps to spend a transaction:
- unlock/unvault
- spend
In-between these 2 steps, there is a revault option that can be triggered, effectively preventing the spend.

## What is Bitcoin Wallet ?
What we commonly call a 'wallet' is in fact a descriptor, the descriptor represent the set of spending rules for your coins, if you want to changes this rules you need to create a new descriptor(wallet). 

## What is "xprv" and "xpub" wrt to Bitcoin Wallet ?
Extended Keys (i.e Extended Private Key - xprv and Extended Public Key - xpub): These are keys used in hierarchical deterministic (HD) wallets, which allow for the derivation of multiple public and private keys from a single seed. The "x" in xprv and xpub stands for "extended." 
	- `xprv (Extended Private Key)`: This key contains all the information necessary to derive both private and public keys for addresses in a wallet. It's crucial for spending transactions because it allows you to sign transactions, thus proving ownership of the funds. An xprv can be used to regenerate all child keys, making it extremely sensitive information that should be kept private and secure. If someone gains access to an xprv, they can control all the funds associated with that wallet.
	
	- `xpub (Extended Public Key)`: In contrast, the xpub can be shared publicly because it allows only for the derivation of public keys, which are used to generate addresses for receiving funds but cannot be used to spend those funds without the corresponding private keys.

## What is BDK ?
 BDK is Bitcoin Wallet Development Kit BDK. It is tool and code repository in rust available as a crate for easy development of Modern Bitcoin Wallets. 
	- BDK uses these extended keys as part of its wallet management system, particularly for creating and managing hierarchical deterministic wallets. This approach simplifies wallet backup and restoration processes since all you need to restore a wallet is the seed (from which the xprv can be derived) or the xprv itself.
	- In BDK, wallets are often described using "descriptors," which are compact representations of how scripts (and subsequently, addresses) should be generated or derived. An ExtendedDescriptor, which might include an xprv, represents a descriptor with derivable keys, allowing for the creation of multiple addresses from a single descriptor.
	- When using BDK or its command-line interface (bdk-cli), you might encounter or use xprv when setting up a wallet, especially when dealing with transactions that require signing (spending) capabilities. For instance, generating a new wallet or restoring one from a seed typically involves handling or generating an xprv.
	- BDK's design around descriptors and extended keys enhances wallet interoperability and functionality, adhering to Bitcoin's standards for key management and transaction signing.

## What is a descriptor in Bitcoin Wallets? context ?
Descriptors in BDK are strings that encode a script template, which can include public keys or extended public keys (xpubs), along with derivation paths. For instance, when you generate a wallet with BDK, you might use a descriptor template like BIP86 (Taproot) 

## What is "external descriptor" and "internal descriptor" in terms of Wallets?
They refer to distinct sets of addresses used for different transaction types:
	- **External Descriptors**: These are typically used for receiving funds. When you share your Bitcoin address with someone to receive payments, this address is derived from an external descriptor. External descriptors are designed for addresses that are meant to be shared publicly.
	- **Internal Descriptors**: These are used for change addresses. When you send Bitcoin, the transaction might not use the exact amount you have in an existing UTXO (Unspent Transaction Output). The leftover amount, after subtracting the transaction fee, goes to a change address. These addresses are not meant to be shared publicly and are used internally by the wallet to manage funds.

## Why separate and bucket descriptors in two different categories ?
The separation into external and internal descriptors enhances privacy and security
	- Privacy: By using different addresses for change, it becomes harder for external observers to link transactions together, thus enhancing transaction privacy.
	- Security: If someone were to compromise an external address, they wouldn't immediately gain access to internal change addresses, providing an additional layer of security.	

## What is the Concept of KeychainKind in BDK SDK for Bitcoin Wallets ? 
BDK uses a concept called KeychainKind, which can be either External or Internal. This distinction helps in generating the appropriate type of address based on whether it's for receiving funds or for change.When setting up a wallet or generating addresses in BDK, developers can specify whether they want an address for external use (receiving) or internal use (change). This is done through the KeychainKind::External or KeychainKind::Internal flags when building descriptors or generating addresses.

## What is an HSM ? 
A hardware security module (HSM) is a dedicated crypto processor that is specifically designed for the protection of the crypto private key lifecycle i.e the private keys that are used for proving the Bitcoin Ownership or for moving teh actual Bitcoins. A hardware security module (HSM) provides secure key storage and cryptographic operations within a tamper-resistant hardware device. HSMs are designed to securely store cryptographic key material and use the key material without exposing it outside the cryptographic boundary of the hardware.

## What is a Software Defined Hardware Security Module (SD-HSM) ?
A Software Defined Hardware Security Module (HSM) is a virtualized or emulated HSM that provides the functionalities of a physical HSM but operates entirely in software. Unlike traditional physical HSMs that involve dedicated hardware devices to perform cryptographic operations securely, a software-defined HSM replicates these functions on general-purpose hardware, often in a cloud or containerized environment.
    1.	**Cryptographic Operations**: Provides secure key storage, encryption, decryption, and signing operations.
	2.	**Scalability**: Easier to scale than physical HSMs since they can be deployed in cloud environments.
	3.	**Flexibility**: Offers flexibility in deployment, can be integrated into DevOps pipelines, and can be containerized for use in microservice architectures.
	4.	**Cost-Effective**: Reduces the cost and logistical complexity associated with physical HSMs, though it often comes at a trade-off in physical security. 

## What is HSM Coldstorage
For security, OpenVault will store a reserve of Bitcoins in an offline setting. By having these funds offline, it will reduce attack surface and risk of theft.

OpenVault's solution is unique, specifically, it can leverage FIPS certified Hardware Security Modules (HSMs) to protect the private key material. It currently supports certain HSMs to kickstart the usecase often operated and trusted devices bu biggest financial payment-related players in the USA market

Funds can be sent from online systems to the cold storage at any time. Moving funds out of cold storage requires a multi-party signing ceremony. In addition, the offline HSMs are able to enforce business logic rules, for instance it can only allow sending funds to OpenVault's Bitcoin addresses whitelisted for the given institution operating it (More like Policy level Soft version of Covenants without upgrading to new BIP). Such a scheme is usually called defense in depth or an onion model. Openvault maintain's the online/offline isolation by importing transaction metadata and exporting signatures using QR codes. HSMs have the ability to share key material. This enables the ability to store in Openvault's backup architecture in a encrypted form and restore a wallet at any location. Multi-institution setup is also a big piece of this capability

This repo contains our design documents as well as specific technical information. We are sharing our source code, with the caveat that the code is currently only useful if you have the exact same hardware setup. We are willing to make the code more modular over time, as long as the broader community shows interest to implement support for additional hardware vendors.

## What is Bitcoin Custody Solution ?
Bitcoin custody solutions offer a range of advantages, particularly for enterprises and institutions looking to secure large amounts of Bitcoin. These solutions provide mechanisms for securely holding, managing, and controlling private keys, ensuring that assets are protected from theft, loss, and unauthorized access. Bitcoin custody solutions offer crucial security, compliance, and operational advantages, especially for enterprises managing large sums of Bitcoin.

## What is Cold Storage Custody vs Warm(Hot) Storage Custody Solution ?
Two schemes of storage custody are Hot and Cold. 
	•**Cold Storage**: Custody solutions often leverage cold storage (offline storage) to keep assets out of reach from hackers. Cold storage solutions ensure that private keys are never exposed to the internet.
	•**Warm Storage**: Some solutions also offer “warm” storage, which strikes a balance between security and accessibility, allowing quicker access to funds while still providing a high level of security.

## What is Social Recovery Backup ? 
For personal or corporate custodians, some custody solutions allow for “social recovery,” where a trusted group of individuals can help recover keys if they are lost.

## What Risk Mitigations/Enhanced Security/Compliance & Auditing can Custody Providers help with ?
	Internal and External Threats: Custody solutions mitigate both internal risks (e.g., insider theft) and external risks (e.g., hacking). By using multisig wallets, no single person has complete control over the assets, reducing the risk of fraudulent transactions.

	Policy Enforcement: Enterprises can define customizable spending policies, such as requiring multiple approvals from different departments or signatories for high-value transactions.

	Auditing Tools: Enterprises need transparency and detailed records for auditing purposes. Custody solutions often integrate tools that log all activity related to wallet use, providing audit trails for compliance, risk management, and governance.

	Multisig Support: With multisig, multiple parties (or devices) are required to approve a transaction before it is executed, preventing unauthorized transfers even if one key is compromised.

	Geographic Distribution: In enterprise settings, keys or parts of keys can be stored in multiple secure locations, providing additional security and protection against localized threats such as physical theft.

	Private Key Management: Custody solutions allow for sophisticated private key management, which includes splitting keys into parts (via multisig schemes) or using hardware security modules (HSMs) for cold storage. This ensures that a single point of failure doesn’t compromise funds.

## 	What is Collaborative Bitcoin Custody ?

	OpenVault shines when multiple users participate in a vault with their own private key, effectively co-managing it. Participants get push notifications of spending proposals, approvals, and finalized transactions - keeping them informed and enabling them to promptly take action when required. 

## What is Spending Policies in an Enterprise Setting ?
 
	Businesses can capture their business rules into vaults and leverage our mobile application. OpenVault also offers institutional co-management.

## What are Key Agents ? 
With OpenVault, bitcoin advisors and key agents can provide their customers with comprehensive custody and inheritance solutions.

## What do I need a Key Agent ? 
Key agency is a service that can be monetized, either charging by signature or as a percentage of the assets being collaboratively held. OpenVault makes it easy for key agents to specify and execute their offerings.

## What is the concept of Proof of Reserves ?
OpenVault supports proof-of-reserve attestations on vaults. These proofs can be generated to cryptographically prove that the members of the vault have key access and ownership over the vault’s set of UTXOs. Periodic and regular proof-of-reserve attestations are an important component of custodial hygiene, especially in multi-party or multi-institutional custody vaults.

## What happens with every new vault that is created ?
When a new vault is created, an encryption group is created for the vault participants to share information that only they can view. This includes the vault configuration (spending policy), name, description, spending proposals, and utxo labels.

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

## More FAQs
https://www.reddit.com/r/Bitcoin/comments/1bo5wb8/liana_wallet_gathering_some_feedback/
