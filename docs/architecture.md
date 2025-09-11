---
sidebar_position: 2
title: Architecture
description: Sigil Architecture
hide_table_of_contents: false
---


## Sigil Architecture

### WebAssembly Runtime

Like many other smart contract frameworks, Sigil is built on WebAssembly (WASM), which provides several key benefits for blockchain smart contracts:

- **Deterministic Execution**: Contracts can be executed and interrupted based on gas limits and memory constraints, ensuring predictable behavior.
- **Efficient Bytecode**: WASM offers a portable and efficient bytecode format for the Sigil runtime.

Unlike other WASM-based blockchain languages, Sigil is deeply integrated with the modern **WebAssembly Component Model**, providing a structured and interoperable approach to contract development. Sigil's runtime primitives and userspace abstractions enable secure, gas-efficient contracts with a familiar development experience. Storage is isolated, cross-contract calls are convenient to perform (while re-entrances are strictly prohibited), and a `fallback` hook enables support for contract versioning via proxying.

### WebAssembly Component Model

Traditional WASM modules compiled from other languages produce self-contained but unstructured files, making their contents opaque to other code modules. The WASM Component Model introduces a language-agnostic structure, enhancing reusability and transparency. In Sigil:

- **WIT Signatures**: Every Sigil contract includes a WebAssembly Interface Type (WIT) "header" file that explicitly exports all public functions in a standardized, high-level type language. This makes contracts more accessible to developers and tooling.
- **Runtime Validation**: The WIT file is compiled into the WASM code, allowing the Sigil runtime to extract and validate the contract's interface at runtime and for tooling integration.
- **Future Component Imports**: The Component Model lays the foundation for importing other components for code reuse, though cross-contract calls currently use a foreign call primitive in an isolated execution context.
- **WAVE Format**: Sigil uses the WebAssembly Value Expression (WAVE) format for passing arguments and returning results, standardizing value types across languages compiled to WASM.
- **Exported Function Conventions**: Exported functions in WIT files follow standard WIT types, with two additional conventions:
  - **Hook-Based Execution**: Functions with specific names serve as hooks and are called under certain conditions and not explicitly by a user.
  - **Execution Context**: The first argument of every function is an execution context, defining its behavior and access rights.

## Key Features

### Hooks

Sigil defines a minimal but full-featured set of hooks to handle specific contract behaviors:

- **`init` Hook**: Runs when a contract is published and is primarily for setting initial storage values or perform data migrations for contract upgrades. It currently supports arbitrary contract code execution.
- **`fallback` Hook**: Accepts WAVE arguments and returns WAVE results. This hook is called when no other function matches the call, and it is primarily used to implement proxy contracts for versioning in userspace, similar to how proxying is implemented in Solidity.

### Execution Contexts

Public functions in Sigil are "decorated" with one of three execution contexts, controlling their behavior and access:

- **`proc` (Procedure) Context**:
  - Can read and write to storage
  - Callable when submitting blockchain transactions
- **`view` Context**:
  - Read-only access to storage
  - Callable via the API exposed by Kontor nodes
- **`fall` (Fallback) Context**:
  - Used exclusively for the `fallback` hook
  - Can be converted into other context types

Both `proc` and `view` functions can be called within a single contract or across contracts via foreign calls, with static type safety in the SDK and runtime checks ensuring compliance with read-only or read-write rules. Unlike some blockchain languages, Sigil does not implicitly expose storage contents. Developers must explicitly create `view` functions to expose data, enabling fine-grained control for future migrations or upgrades. These functions can include arbitrary code, such as userspace authentication or authorization.

### Primitives

Sigil's runtime primitives keep the virtual machine simple and secure, encouraging userspace abstractions:

1. **Signer Access**: Available only in `proc` contexts, allows access to the signer of the current transaction
2. **Cross-Contract Calls**: A function for calling other contracts, contributing to the gas expenditure of the transaction
3. **Storage Functions**:
   - `get` and `set` operations for primitive types (e.g., strings, u64), parameterized by a key or path
   - Checking if a key exists in storage
   - Regex-based path matching

The Sigil Rust SDK abstracts these primitives into higher-level, ORM-like interfaces, reducing the need for direct interaction.

### Library Code and SDK

Sigil emphasizes userspace development through Rust traits and macros:

- **Traits for Runtime Primitives**: The runtime implements traits for all primitives, allowing pure Rust libraries (including Sigil's standard library) to operate independently of the runtime. Developers can implement in-memory versions of these traits for testing without relying on the Sigil runtime.
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

Sigil simplifies reasoning about security compared to languages like Solidity by eliminating features like `delegatecall` and completely disallowing cross-contract calling cycles. However, developers should still follow the **Checks-Effects-Interactions (CEI)** pattern:

- **Check**: Verify conditions (e.g., sufficient funds)
- **Effect**: Update storage (e.g., deduct funds)
- **Interaction**: Perform external operations (e.g., cross-contract calls)

For example: 

```rust
check_sufficient_funds(); // Check
transfer_funds_in_storage(); // Effect
do_external_operations(); // Interaction
```

Leveraging this pattern makes code easier to reason about and adds an additional defensive layer.