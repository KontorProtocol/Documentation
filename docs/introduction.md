---
sidebar_position: 1
title: Introduction
description: Introduction to Sigil
hide_table_of_contents: false
---

# Introduction

Sigil is currently in *early-preview* and available to select developers for feedback and review. Developers can write smart contracts with Sigil, but not yet execute them with the Kontor Indexer or deploy them to a Bitcoin network.

Current roadmap:
* Gas system
* List types
* Events system
* Advanced storage querying 
* Integration with Kontor Indexer 
* Typescript SDK
* Deployment to Bitcoin `testnet4` / `signet`


## Core Architecture

### WASM Integration

Sigil is built on WebAssembly (WASM), which provides several key benefits for blockchain smart contracts:

- **Deterministic Execution**: Contracts can be executed and interrupted based on gas limits and memory constraints, ensuring predictable behavior.
- **Efficient Bytecode**: WASM offers a portable and efficient bytecode format for the Sigil runtime.
- **Polyglot Potential**: Future SDKs could allow other languages to compile to SigilWasm, broadening accessibility.
- **Library Reuse**: WASM enables the use of pure (independent of runtime built-ins) libraries outside the blockchain ecosystem.

Unlike other WASM-based blockchain languages, Sigil is deeply integrated with the modern WebAssembly Component Model, providing a structured and interoperable approach to contract development.

Sigil's minimal runtime primitives and userspace abstractions enable secure, gas-efficient contracts with a familiar development experience. Storage is isolated, cross-contract calls are convenient to perform while re-entrances are strictly prohibited, `fallback` hook enables support for contract versioning via proxying.

### WASM Components Model

Traditional WASM modules compiled from various languages produce self-contained but unstructured files, making their contents opaque. The WASM Component Model introduces a language-agnostic structure, enhancing reusability and transparency. In Sigil:

- **WIT Signatures**: Every SigilWasm contract includes a WebAssembly Interface Type (WIT) "header" file that explicitly exports all public functions in a standardized, high-level type language. This makes contracts more accessible to developers and tooling.
- **Runtime Validation**: The WIT file is compiled into the WASM code, allowing the SigilWasm runtime to extract, validate the contract's interface at runtime or for tooling purposes.
- **Future Component Imports**: The Component Model lays the foundation for importing other components for code reuse, though cross-contract calls currently use a foreign call primitive in an isolated execution context.
- **WAVE Format**: SigilWasm uses the WASM Value Expression (WAVE) format for passing arguments and returning results, standardizing value types across languages compiled to WASM.
- **Exported Function Conventions**: Exported functions in WIT files follow standard WIT types, with two additional conventions:
  - **Hook-based Execution**: Functions with specific names serve as hooks and are called under certain conditions and not explicitely by a user.
  - **Execution Context**: The first argument of every function is an execution context, defining its behavior and access rights.

## Key Features

### Hooks

Sigil defines a minimal set of hooks to handle specific contract behaviors:

- **`init` Hook**: Runs when a contract is published and is primarily for setting initial storage values or perform data migrations for contract upgrades; although, it currently supports arbitrary contract code execution.
- **`fallback` Hook**: Accepts WAVE arguments and returns WAVE results. It's called when no other function matches the call, primarily used to implement proxy contracts for versioning in userspace, similar to how proxying is implemented in Solidity.

### Execution Contexts

Public functions in Sigil are "decorated" with one of three execution contexts, controlling their behavior and access:

- **`proc` (Procedure) Context**:
  - Can read and write to storage.
  - Callable when submitting blockchain transactions.
- **`view` Context**:
  - Read-only access to storage.
  - Callable via the API exposed by Kontor nodes.
- **`fall` (Fallback) Context**:
  - Used exclusively for the `fallback` hook.
  - Can be converted into other context types

Both `proc` and `view` functions can be called within a single contract or across contracts via foreign calls, with static type safety in the SDK and runtime checks ensuring compliance with read-only or read-write rules. Unlike some blockchain languages, Sigil does not implicitly expose storage contents. Developers must explicitly create `view` functions to expose data, enabling fine-grained control for future migrations or upgrades. These functions can include arbitrary code, such as userspace authentication or authorization.

### Primitives

Sigil's runtime provides a minimal set of primitives to keep the virtual machine simple and secure, encouraging userspace abstractions:

1. **Signer Access**: Available only in `proc` contexts, allows access to the signer of the current transaction.
2. **Cross-Contract Calls**: A function for calling other contracts, contributing to the gas expenditure of the transaction.
3. **Storage Functions**:
   - `get` and `set` operations for primitive types (e.g., strings, u64), parameterized by a key or path.
   - Checking if a key exists in storage.
   - Regex-based path matching.

The Rust SDK abstracts these primitives into higher-level, ORM-like interfaces, reducing the need for direct interaction.

### Library Code and SDK

Sigil emphasizes userspace development through Rust traits and macros:

- **Traits for Runtime Primitives**: The runtime implements traits for all primitives, allowing pure Rust libraries (including Sigil's standard library) to operate independently of the runtime. Developers can implement in-memory versions of these traits for testing without relying on the SigilWasm runtime.
- **Ordinary Rust Programming**: Rust macros make Sigil programming feel simple and familiar.
- **Private Helper Functions**: Non-exported functions (without WIT signatures) compile to regular WASM instructions, behaving like standard Rust functions.

### Storage and Versioning

Sigil's storage system is designed for simplicity, isolation, and predictable gas usage:

- **Primitive Operations**: Storage uses path-keyed `get` and `set` operations for basic types (e.g., integers, booleans, strings).
- **ORM-like Macros**: The SDK provides macros for declaring persistent data types, including records, enums, non-recursive sum types, and built-in `Map` types. Nested fields can be accessed (e.g., `foo.bar.baz`) without retrieving entire structures.
- **Isolation**: Each contract's storage is isolated, with no equivalent to Ethereum's `delegatecall`. This simplifies the mental model, treating foreign contract calls like web API calls.
- **Gas Tracking**: The design avoids complex operations (e.g., SQL-like joins) to make gas consumption intuitive.
- **Versioning**: Sigil supports userspace versioning through the `fallback` hook and proxy contracts. Developers can implement advanced upgrade mechanisms, such as DAO-like voting for safe upgrades, avoiding reliance on key-based upgrades that risk rug pulls. External calls to proxy contracts appear as standard contract calls, enhancing transparency.

### Security Considerations

Sigil simplifies reasoning about security compared to languages like Solidity by eliminating features like `delegatecall` and disallowing cross-contract calling cycles. However, developers should still follow the **Checks-Effects-Interactions (CEI)** pattern:

- **Check**: Verify conditions (e.g., sufficient funds).
- **Effect**: Update storage (e.g., deduct funds).
- **Interaction**: Perform external operations (e.g., cross-contract calls).

For example: 

```rust
check_sufficient_funds(); // Check
transfer_funds_in_storage(); // Effect
do_external_operations(); // Interaction
```

While not strictly necessary, leveraging this pattern makes code easier to reason about and adds an additional defensive layer.

## Current Implementation Status

The early preview of Sigil includes:

- Full integration with the WASM Component Model, including WIT signatures and WAVE formats.
- Support for `init` and `fallback` hooks.
- `proc`, `view`, and `fall` execution contexts with runtime enforcement.
- Minimal primitives for signer access, cross-contract calls, and storage operations.
- A Rust SDK with ORM-like macros for storage and traits for runtime primitives.
- Userspace versioning via proxy contracts.

## Known Limitations and Future Plans

**Planned Improvements**:

- Add support for list types in storage.
- Enhance querying capabilities, including key prefix matching for `Map` types.
- Gas tracking
- Typescript SDK for interacting with contracts clientside
- Full platform integration testing of contracts

## Get Started

Follow the [Getting Started](/getting-started) guide to set up your environment and explore `sigil-example-contracts`. Future test tooling will include live Kontor instances and client-side SDK integration with Bitcoin testnet for full blockchain validation.
