<div align="center">
  <img src="https://raw.githubusercontent.com/RALPH22222/csmp-web-admin/main/public/logo.png" width="80" alt="BayanIpon Logo" />
  <h1>BayanIpon - Soroban Smart Contracts</h1>
  <p><strong>The Decentralized Trust Layer</strong></p>
</div>

---

## 📖 Overview

This repository contains the **Soroban Smart Contracts** written in Rust for **BayanIpon**. These contracts form the core trustless layer of our protocol, ensuring that the traditional *Paluwagan* (group savings) can operate without intermediaries, manual cash boxes, or predatory risks.

## ✨ Core Features

- **Paluwagan Escrow Pools:** 
  A trustless escrow mechanism that securely locks participant contributions (e.g., in USDC). The contract enforce the cycle rules and automatically releases payouts to the designated recipient when a cycle concludes.
- **On-Chain Credit Scoring (Soulbound Reputation):** 
  Every contribution, late payment, and default is tracked on-chain. Good behavior dynamically mints and updates a soulbound non-fungible asset representing the user's creditworthiness.
- **Micro-Lending Protocol:** 
  Users who achieve a high on-chain credit score unlock access to short-term liquidity pools, replacing the predatory 240% APR "5-6" loans with fair, algorithmically defined interest rates.
- **Role-Based Access Control:** 
  Utilizes strict multi-signature and admin authorization mechanisms to ensure community organizers can initiate pools but can never unilaterally withdraw or steal funds.

## 🛠️ Tech Stack

- **Language:** Rust
- **Blockchain:** Stellar Network
- **Smart Contract Platform:** Soroban

## 🚀 Getting Started

### Prerequisites
- Rust (`rustup`, `cargo`)
- `stellar-cli` for deploying to Testnet/Futurenet
- Target `wasm32-unknown-unknown` installed

### Build the Contracts

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests

```bash
cargo test
```

### Deploy to Testnet

Use the Stellar CLI to deploy the compiled `.wasm` file:
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/bayan_ipon_contract.wasm \
  --source <ADMIN_SECRET_KEY> \
  --network testnet
```

---
*Built for the Stellar Blockchain Hackathon - Local Finance & Real World Access Track.*
