---
id: examples
title: Examples
sidebar_label: Examples
sidebar_position: 2
---

This section covers the five example contracts in the `sigil-example-contracts` folder (`hello-world`, `token`, `shared-account`, `shared-account-dynamic`, `proxy`), each building on the previous to demonstrate new concepts in Sigil on the Kontor runtime. The examples progress from a minimal contract to a token system, shared account, dynamic shared account, and proxy pattern, showing how to develop blockchain smart contracts with seamless Rust-like testing and quick feedback in a simulated environment.

Each contract directory in `sigil-example-contracts` contains:
- `contract/`: Rust code (`src/lib.rs`), WIT interface (`wit/contract.wit`), and `wit/deps` (symlink to host-provided built-ins, e.g., `kontor:built-in`).
- `test/`: Tests (`src/lib.rs`), `build.rs` (compiles, optimizes, compresses contract to `target/`), and `Cargo.toml` for dependencies.
- Root `Cargo.toml` and `Cargo.lock` for the workspace.

To develop and test smart contracts for the blockchain, navigate to a contract’s directory (e.g., `sigil-example-contracts/hello-world`) and execute `cargo test`. Tests use the Kontor runtime for simulation, loading contract bytes from `target/` via `contract_bytes()` or `dep_contract_bytes()` for dependencies specified in `test/Cargo.toml`. This provides fast iteration like standard Rust development, but future layers will enable testing against a running Kontor instance and client-side SDK integration with a node running against Bitcoin testnet for full blockchain validation. Each example assumes familiarity with prior ones, focusing on new concepts.