# What Is a Partially Signed Bitcoin Transaction (PSBT)?

# PSBTs Practical

## PSBT + Coinjoin between 3 parties
ASCII art representation of a PSBT (Partially Signed Bitcoin Transaction) transaction for a CoinJoin between three parties

```
          Party A        Party B        Party C
             |              |              |
             |              |              |
             |              |              |
             v              v              v
   +----------------+  +----------------+  +----------------+
   |Create PSBT     |  |                |  |                |
   | (Unspent UTXOs |  |                |  |                |
   |  & Outputs)    |  |                |  |                |
   +----------------+  +----------------+  +----------------+
             |              |              |
             |              |              |
   ------------>-------------->-------------->-------------- 
                 Share PSBT for Coordination
   <------------<--------------<--------------<------------
             |              |              |
             v              v              v
   +----------------+  +----------------+  +----------------+
   |Sign PSBT       |  |Sign PSBT       |  |Sign PSBT       |
   | (Add Signature |  | (Add Signature |  | (Add Signature |
   |  for A's inputs)|  |  for B's inputs)|  |  for C's inputs)|
   +----------------+  +----------------+  +----------------+
             |              |              |
             |              |              |
   ------------>-------------->-------------->-------------- 
                 Share Partially Signed PSBT
   <------------<--------------<--------------<------------
             |              |              |
             v              v              v
   +----------------+  +----------------+  +----------------+
   |Combine Signatures|  |Combine Signatures| |Combine Signatures|
   | into one PSBT    |  | into one PSBT    | | into one PSBT    |
   +----------------+  +----------------+  +----------------+
             |              |              |
             |              |              |
             v              v              v
   +----------------+  +----------------+  +----------------+
   |Finalize PSBT    |  |Finalize PSBT    |  |Finalize PSBT    |
   | (Verify all     |  | (Verify all     |  | (Verify all     |
   |  signatures)    |  |  signatures)    |  |  signatures)    |
   +----------------+  +----------------+  +----------------+
             |              |              |
             v              v              v
   +----------------+  +----------------+  +----------------+
   |Broadcast        |  |Broadcast        |  |Broadcast        |
   | Fully Signed    |  | Fully Signed    |  | Fully Signed    |
   | Transaction     |  | Transaction     |  | Transaction     |
   +----------------+  +----------------+  +----------------+
             |              |              |
             |              |              |
             v              v              v
         Transaction Confirmed on Bitcoin Blockchain
```

**This ASCII art illustrates the following steps:**

1. **Creation of PSBT**: Party A creates the initial PSBT with the inputs (UTXOs) and outputs for the CoinJoin transaction. For outputs each party creates their own address and shares it with the co-ordinator here in this case happens to be Party A. A also creates his own address. Please note in case of coinjoin it can also be a 3rd party decentralised software that runs the coordination independent and no involvement of any of the 3 participants. 
2. **Sharing for Coordination**: The PSBT is shared among all parties for coordination.
3. **Signing by Each Party**: Each party signs the transaction for their own inputs.
Combining Signatures: The PSBT with partial signatures is shared among parties again to combine all signatures into one PSBT.
4. **Finalization**: The transaction is finalized by verifying all signatures.
5. **Broadcast**: The fully signed transaction is broadcast to the Bitcoin network.
Confirmation: The transaction gets confirmed on the blockchain.

This process ensures that all parties involved in the CoinJoin can contribute their signatures at different times and places, maintaining privacy and security through the use of PSBTs.

## Let's dive deeper into Stage 4: Combining Signatures of a CoinJoin transaction using PSBTs. 
This stage is crucial because it involves merging all the partial signatures from different parties into a single PSBT, which will then be ready for finalization and broadcast. Here's an ASCII art representation and explanation of this stage:

```
          Party A        Party B        Party C
             |              |              |
             |              |              |
     +-------------------+-------------------+-------------------+
     | PSBT with A's     | PSBT with B's     | PSBT with C's     |
     | partial signatures| partial signatures| partial signatures|
     +-------------------+-------------------+-------------------+
             |              |              |
             |              |              |
   ------------>-------------->-------------->-------------- 
             |              |              |
             v              v              v
     +-------------------+-------------------+-------------------+
     |  Combine PSBTs    |  Combine PSBTs    |  Combine PSBTs    |
     |  (Party A's Role) |  (Party B's Role) |  (Party C's Role) |
     +-------------------+-------------------+-------------------+
             |              |              |
             |              |              |
   <------------<--------------<--------------<------------
             |              |              |
             v              v              v
     +-------------------+-------------------+-------------------+
     |  Combined PSBT    |  Combined PSBT    |  Combined PSBT    |
     |  (All Signatures) |  (All Signatures) |  (All Signatures) |
     +-------------------+-------------------+-------------------+
```


### Detailed Explanation:

1. **Initial State**: 
Each party (A, B, C) has their own version of the PSBT, which includes the signatures they've added for their inputs. 
    a. Party A has a PSBT with signatures for A's inputs.
    b. Party B has a PSBT with signatures for B's inputs.
    c. Party C has a PSBT with signatures for C's inputs.

2. **Sharing PSBTs**: 
Each party shares their partially signed PSBT with the others. This step ensures that all parties have a copy of the PSBTs containing all partial signatures.

3. **Combining PSBTs**:

    a. Now, each party or a dedicated coordinator (could be one of the parties or an external entity) takes all these partially signed PSBTs and combines them. In the ASCII art, this is represented by the "Combine PSBTs" block for each party. However, in practice, this might be done by one designated party or a software tool.

    b. The combination process involves taking the signatures from each party's PSBT and merging them into one PSBT. This is typically done using a function like combinepsbt in Bitcoin Core or similar functionality in other software.

4. **Result**: 
After combining, all parties would have a PSBT that now contains signatures for all inputs from all parties. This combined PSBT is what's shown in the final block in the ASCII art.

5. **Flow of Information**: 
The arrows demonstrate how the PSBTs are shared back and forth (first sharing partial signatures, then receiving the combined PSBT).

This stage ensures that every input in the transaction has the necessary signatures from all parties involved, preparing the PSBT for the finalization step where it will be verified and converted into a fully signed Bitcoin transaction ready for broadcast.

## Rust code example for combining three PSBTs, in the context of a CoinJoin transaction using BDK v1.0

```
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::bitcoin::util::psbt::combine;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example PSBTs as strings. In real scenarios, these would come from different parties in a CoinJoin.
    let psbt1_str = "cHNidP8BAHECAAAAASR8KkU3h02o1iJ6z8V2t3mJHTaZ98b1w93W2L8gJ37tAAAAAA..."; // Truncated for brevity
    let psbt2_str = "cHNidP8BAHECAAAAASR8KkU3h02o1iJ6z8V2t3mJHTaZ98b1w93W2L8gJ37tAAAAAA..."; // Another truncated PSBT
    let psbt3_str = "cHNidP8BAHECAAAAASR8KkU3h02o1iJ6z8V2t3mJHTaZ98b1w93W2L8gJ37tAAAAAA..."; // Yet another truncated PSBT

    // Parse the PSBT strings into PartiallySignedTransaction structures
    let psbt1 = PartiallySignedTransaction::from_str(psbt1_str)?;
    let psbt2 = PartiallySignedTransaction::from_str(psbt2_str)?;
    let psbt3 = PartiallySignedTransaction::from_str(psbt3_str)?;

    // Combine the three PSBTs
    let combined_psbt = combine(&[psbt1, psbt2, psbt3])?;

    // Now 'combined_psbt' contains all signatures from all three PSBTs
    println!("Combined PSBT: {}", combined_psbt.to_string());

    Ok(())
}
```

### Notes:
1. **Dependencies**: Ensure your Cargo.toml includes the necessary dependencies:
toml
```
[dependencies]
bdk = "1.0"

```

2. **Combining PSBTs**: The combine function now takes an array slice of three PartiallySignedTransaction instances. This function merges the information from each PSBT, ensuring that all signatures for each input are included in the resulting PSBT.

3. **Error Handling**: The use of ? for error propagation means that if any step fails, the function will return early with an error. You might want to handle errors more explicitly in a production environment.

4. **PSBT Strings**: In a CoinJoin scenario, these strings would be exchanged between participants, each of whom might have signed for their inputs. The example strings are placeholders and would be much longer in real usage.

5. **CoinJoin Considerations**: In a real CoinJoin, you would need to ensure that each participant's PSBT adds inputs and outputs correctly so that when combined, the transaction looks like one where multiple parties are contributing to and benefiting from a single transaction, thus enhancing privacy by obscuring who is paying whom.

This example demonstrates how you can combine multiple PSBTs, which is a key step in a CoinJoin where different participants have signed their respective inputs. Remember that after combining, you would typically proceed to finalize and broadcast the transaction.

## PSBTS Before and After Signing JSONs

### Before Signing (Initial PSBT)

```
{
  "tx": {
    "version": 2,
    "locktime": 0,
    "inputs": [
      {
        "txid": "abc123...",
        "vout": 0,
        "sequence": 4294967295
      },
      {
        "txid": "def456...",
        "vout": 1,
        "sequence": 4294967295
      },
      {
        "txid": "ghi789...",
        "vout": 2,
        "sequence": 4294967295
      }
    ],
    "outputs": [
      {
        "address": "bc1qrecipient1...",
        "amount": 500000
      },
      {
        "address": "bc1qrecipient2...",
        "amount": 300000
      },
      {
        "address": "bc1qrecipient3...",
        "amount": 200000
      }
    ]
  },
  "inputs": [
    {
      "utxo": {
        "txid": "abc123...",
        "vout": 0,
        "amount": 600000
      },
      "partial_signatures": {}
    },
    {
      "utxo": {
        "txid": "def456...",
        "vout": 1,
        "amount": 400000
      },
      "partial_signatures": {}
    },
    {
      "utxo": {
        "txid": "ghi789...",
        "vout": 2,
        "amount": 300000
      },
      "partial_signatures": {}
    }
  ],
  "outputs": [
    {
      "address": "bc1qrecipient1...",
      "amount": 500000
    },
    {
      "address": "bc1qrecipient2...",
      "amount": 300000
    },
    {
      "address": "bc1qrecipient3...",
      "amount": 200000
    }
  ]
}
```

### After Signing (PSBT with Partial Signatures)
```
{
  "tx": {
    "version": 2,
    "locktime": 0,
    "inputs": [
      {
        "txid": "abc123...",
        "vout": 0,
        "sequence": 4294967295
      },
      {
        "txid": "def456...",
        "vout": 1,
        "sequence": 4294967295
      },
      {
        "txid": "ghi789...",
        "vout": 2,
        "sequence": 4294967295
      }
    ],
    "outputs": [
      {
        "address": "bc1qrecipient1...",
        "amount": 500000
      },
      {
        "address": "bc1qrecipient2...",
        "amount": 300000
      },
      {
        "address": "bc1qrecipient3...",
        "amount": 200000
      }
    ]
  },
  "inputs": [
    {
      "utxo": {
        "txid": "abc123...",
        "vout": 0,
        "amount": 600000
      },
      "partial_signatures": {
        "03a34b...": "3045022100ab12cd...3045022100ef34gh..."
      }
    },
    {
      "utxo": {
        "txid": "def456...",
        "vout": 1,
        "amount": 400000
      },
      "partial_signatures": {
        "027b56...": "3045022100cd45ef...3045022100gh67ij..."
      }
    },
    {
      "utxo": {
        "txid": "ghi789...",
        "vout": 2,
        "amount": 300000
      },
      "partial_signatures": {
        "02d89c...": "3045022100ij78kl...3045022100mn89op..."
      }
    }
  ],
  "outputs": [
    {
      "address": "ac1qrecipient1...",
      "amount": 500000
    },
    {
      "address": "bc1qrecipient2...",
      "amount": 300000
    },
    {
      "address": "cc1qrecipient3...",
      "amount": 200000
    }
  ]
}
```


# PSBTs Theory

## Definition:
A PSBT* is a Bitcoin transaction standard for transactions that are not fully signed. It enables multiple parties to sign the same transaction collaboratively.

## Introduction:
PSBT was introduced in BIP 174 to improve interoperability between wallets, facilitating complex multi-signature workflows.

## Format and Metadata:
	•	Defines a standard format to represent Bitcoin transactions.
	•	Carries metadata to make signing and verifying transactions easier.

## Process:
	•	PSBTs can be signed by multiple parties in parallel.
	•	Once all signatures are collected, they are combined to create a fully signed transaction.

## What Are PSBTs Used For?
	1.	Trustless Atomic Swaps: Enables asset exchanges across blockchains without third-party reliance.
	2.	Multi-Signature Wallets:
        •	Facilitates secure signing by multiple parties.
        •	Simplifies complex workflows for multi-sig wallets.
	3.	Offline Payments:
        •	Supports secure signing on cold wallets, keeping private keys offline.
        •	Combines offline signing with broadcast using watch-only wallets and Bitcoin nodes.
	4.	Coordination for Multi-Party Transactions:
        •	Used in protocols like CoinJoin, CoinSwap, and PayJoin to coordinate between multiple signers.

## Advantages of PSBTs
	1.	Interoperability:
        •	Enhances wallet and software compatibility.
        •	Widely adopted by major wallet providers and node software.
	2.	Offline Signing:
        •	Verifies transaction details on cold devices.
        •	Simplifies secure transaction creation using watch-only wallets.
	3.	Multi-Signature Support:
        •	Makes multi-sig transactions portable and secure.
        •	Encourages privacy, security, and loss prevention in the Bitcoin ecosystem.
	4.	Multi-Party Coordination:
	    •	Provides a standard for constructing and finalizing transactions with inputs from multiple signers.

## How Do PSBTs Work?
	1.	Initiation:
	    •	Each participant selects UTXOs and provides output addresses for the transaction.
	2.	Transaction Construction:
        •	A coordinator creates a draft transaction using the provided UTXOs and outputs.
        •	Converts the transaction to a PSBT and distributes it to participants.
	3.	Signing:
        •	Each participant adds their signature to the PSBT independently.
        •	Signed PSBTs are returned to the coordinator.
	4.	Finalization:
        •	The coordinator combines the signed PSBTs into a fully signed transaction.
        •	The transaction is broadcast to the Bitcoin network.
	5.	Trustless Security:
	    •	The process is designed to ensure no party, including the coordinator, can steal funds.


## History, Background and Future
Partially Signed Bitcoin Transactions (PSBT) were introduced in Bitcoin Core version 0.17.0 in 2018 and formally defined in Bitcoin Improvement Proposal (BIP) 174. PSBTs improve how different wallet software and devices work together, making transactions more secure and efficient.

However, the PSBT standard does have several drawbacks, which is why work is currently being done to develop a PSBT v2 standard. In particular, constructing a transaction by adding inputs iteratively is inefficient, and PSBT files can grow relatively large, especially for hardware wallets, which typically have minimal memory.

* the development of PSBT v2 (Partially Signed Bitcoin Transaction Version 2) is being documented under BIP 370. 