---
sidebar_position: 1
title: Implementation
---

# Implementation Environment Reference

## `ProcContext`

Enables state-modifying operations, such as balance transfers. Provides write access to storage and signer access for transaction authentication (via `ctx.signer()`). Used in functions like `mint` or `transfer` to update blockchain state.

## `ViewContext`

Supports read-only queries for inspecting contract state. Restricts access to read-only storage operations, with no signer or mutations allowed. Used in functions like `balance` for retrieving data without modifying the blockchain.

## `FallContext`

Manages unmatched calls via the `fallback` hook, enabling blockchain proxy patterns. Converts to `ViewContext` or `Option<ProcContext>` for storage reads if needed. Exclusive to the `fallback` function for dynamic routing of blockchain calls.

## Traits for Non-Exported Functions

Non-exported helper functions use traits to ensure context-appropriate access in a convenient way:

### `WriteContext`

For mutation logic (e.g., internal updates). Implemented only by `ProcContext`.

### `ReadContext`

For read-only logic (e.g., internal queries). Implemented by `ViewContext` and `ProcContext`, enabling shared read operations across views and procedures.

## `StorageRoot`

Marks the root struct or enum for contract storage.

## `Map<Key, Value>`

A key-value store for collections, such as account balances in blockchain contracts. Supports `get`, `set`, and `keys` methods.

## Storage Wrappers

Helpers for accessing blockchain storage:
- `storage(ctx)`: Returns the root storage wrapper.
- Field accessors (e.g., `storage(ctx).ledger()`): Provide structured access to maps or fields.
- Get/set methods: Use context to guarantee read only access for views.

## Signer Access

Retrieves the transaction signer via `ctx.signer()` in ProcContext, enabling authentication for procedures (e.g., identifying the sender in `mint` or `transfer`).

## Cross-Contract Calls

Invokes other contracts on the blockchain:
- **Static**: Direct calls via imported modules (e.g., `token::transfer` in `sigil-example-contracts/shared-account`).
- **Dynamic**: Via interfaces (e.g., `token_interface::transfer(&address, ...)` in `sigil-example-contracts/shared-account-dynamic`).

## Utilities

Additional functions for blockchain logic:

### `crypto::generate_id() -> String`

Generates unique IDs for entities (e.g., account IDs).

### `crypto::hash(String) -> (String, Vec<u8>)`

Applies sha265 hash to the string and returns a hex encoded string of the hash and the hash in raw bytes.

### `crypto::hash_with_salt(data: String, salt: String) -> (String, Vec<u8>)`

Applies sha265 hash to the string concatenated with the salte and returns a hex encoded string of the hash and the hash in raw bytes.

### `Error::new(&str)`

Creates custom errors for result types in transaction failures.

## Contract Macro

`contract!(name = "name")`: Defines the contract name and generates boilerplate Rust code based on the contract's WIT filekj.

## Import Macro

`import!(name, height, tx_index, path)`: "Statically" imports another contract’s WIT, enabling cross-contract calls in blockchain logic.

## Interface Macro

`interface!(name, path)`: Defines a dynamic interface for runtime contract calls, used for flexible cross-contract calling (e.g., in `sigil-example-contracts/shared-account-dynamic`).