# CipherPay zkAudit Architecture

## High-Level Architecture Diagram

```mermaid
graph TD
    subgraph User/Wallet
        A[User/Wallet]
        B[CipherPay SDK]
    end
    subgraph Audit Service
        C[CipherPay zkAudit (Rust + RISC Zero)]
    end
    subgraph ZKP Circuits
        D[CipherPay Circuits (Circom)]
    end
    subgraph Auditor
        E[Auditor]
    end
    A -->|Initiate Private Tx| B
    B -->|Generate/ViewKey, Request Proof| C
    C -->|Implements ZKP Logic (RISC Zero guest)| C
    C -->|(Optional: Reference/Ported Logic)| D
    C -->|API: Generate/Verify Proof| B
    B -->|Selective Disclosure| E
    E -->|Verify Proof| C
```

## Component Roles

- **CipherPay Circuits (Circom):**
  - Defines ZKP constraints (e.g., audit_proof.circom, merkle.circom)
  - Used for on-chain or traditional SNARK/PLONK proofs

- **CipherPay zkAudit (Rust + RISC Zero):**
  - Implements equivalent ZKP logic in Rust for RISC Zero zkVM
  - Provides APIs for proof generation, verification, and selective disclosure
  - Handles compliance, audit, and programmable privacy flows

- **CipherPay SDK:**
  - Manages user keys, view keys, and wallet operations
  - Interfaces with zkAudit for proof requests and selective disclosure
  - Used by users, wallets, and dApps

- **Auditor:**
  - Receives selectively disclosed transaction details and ZKPs
  - Verifies proofs using zkAudit APIs

## Typical Flow

1. **User** initiates a private transaction via the SDK.
2. **SDK** requests a ZKP from zkAudit (e.g., proof-of-payment, audit proof).
3. **zkAudit** runs the ZKP logic (in RISC Zero), matching the constraints of the Circom circuits.
4. **SDK** provides a view key and selectively discloses transaction details + proof to the **Auditor**.
5. **Auditor** verifies the proof using zkAudit, confirming the transaction's validity without seeing unrelated private data.

## Notes
- The architecture supports both on-chain (Circom/SNARK) and off-chain (RISC Zero/zkVM) proofs.
- The ZKP logic in zkAudit should be kept in sync with the Circom circuits for consistency and security.
- Modular design allows for future extensibility (new proof types, compliance rules, etc). 