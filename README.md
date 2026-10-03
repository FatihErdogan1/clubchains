# ClubChains

A Rust command-line prototype for managing university clubs — membership, voting with delegation, events, certificates and club finances — with a minimal **Stellar Soroban** smart contract for minting club tokens on testnet.
Built for the **Rise In × Patika.dev × Stellar Rust Hackathon 2025**.

> Türkçe açıklama: [README.tr.md](README.tr.md)

## Features

**Membership & governance**
- Register members (each starts with 1 vote power) and freeze/block members
- Cast votes on topics — frozen members and members without vote power are rejected
- Delegate voting rights to another member
- Impact-score report (members, total vote power, frozen and delegated counts, total votes), member list and vote log

**Events & certificates**
- Create and list events
- Issue and list participation certificates (`CERT-<timestamp>` IDs)

**Finances**
- Record income and expenses, view a financial summary with net balance

**Soroban integration**
- "Mint token" / "Burn token" menu options call `stellar contract invoke` on **Stellar testnet** against a deployed contract
- `src/contracts/` contains a Soroban contract (soroban-sdk 20) with `mint` and `balance` functions, storing balances in contract instance storage

**Other**
- Interactive menu (Turkish UI) with a demo-data loader
- State persisted between runs in a local `chain.json` file (serde/serde_json)

## Tech Stack

- Rust (edition 2024), `serde`, `serde_json`, `chrono`
- Stellar Soroban SDK 20 (contract), Stellar CLI (testnet invocations)

## Project Structure

```
src/
├── main.rs        # entry point
├── menu.rs        # interactive CLI menu
├── models.rs      # Member, Vote, Event, Certificate, Income, Expense, Chain
├── storage.rs     # load/save state as chain.json
├── demo.rs        # sample data loader
├── logic/         # one module per action (join, vote, delegate, freeze, mint, burn, event, income, expense, certificate, reports)
└── contracts/     # Soroban token contract (separate crate)
```

## Running

Requires a Rust toolchain that supports edition 2024 (Rust 1.85+).

```bash
cargo run
```

The mint/burn options additionally need the [Stellar CLI](https://developers.stellar.org/docs/tools/cli) on your `PATH` with a testnet identity. The contract ID and identity names are hard-coded in `src/logic/mint.rs` and `src/logic/burn.rs` (e.g. `stellar keys generate fatih276 --network testnet`).

## Status

Hackathon prototype. Club records live in the local JSON file; only token mint/burn calls go on-chain. The contract crate in `src/contracts/` is not part of the main Cargo build, and it currently implements `mint` and `balance` only (no `burn`).

---

**Author:** Fatih Erdoğan
