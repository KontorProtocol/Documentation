---
sidebar_position: 4
title: Kontor vs. Move
---
# Kontor vs. Move

The biggest difference between the Sigil and Move smart contract systems is that Sigil is an embedded DSL in Rust designed to make writing contracts feel like writing regular Rust code. Sigil's emphasis is on simplicity, readability, and overall developer experience. Move, by contrast, is a novel DSl with Rust-like syntax that implements an idiosyncratic resource-oriented programming model designed to support greater theoretical performance and safety guarantees but requires learning a significant number of new concepts and implementing complex workflows to fit the Move model. Because of precisely these limitations, Move has split into at least two different main versions: Aptos Move and Sui Move, each of which has its own (significant) variations in programming model, and incompatible tooling. As such, the Move ecosystem is fragmented, and the tooling is even less mature than it would be otherwise.

A significant fraction of Move's design quirks were introduced to support formal verification. Formal verification, while a very interesting technology, is not the silver bullet for code correctness that it is often made out to be. Verification of any meaningful amount of code is labor-intensive and highly technical, which explains its limited adoption up to the present day. Only a few Aptos Move dApps have used formal verification so far, and Sui Move had no support until this year. (https://blog.sui.io/asymptotic-move-prover-formal-verification/). Even when formal verification is employed, it is usually practical to verify only a small core set of functions and simple properties they should exhibit.

Along similar lines, while Move’s resource model makes it impossible to copy or implicitly drop assets—which does remove a class of bugs—it does not verify business intent in general. Mis‑scoped authorities, mispriced fees, incorrect caps, or time/sequence errors still compile and must be guarded explicitly. You also pay ceremony—abilities, `acquires`, and resource plumbing that ripple through refactors. In Move, explicitly declaring `acquires` shapes is a benefit for auditability and scheduling, but it adds significant friction to development.

Overall, **Sigil** is much easier to write: it lets you author contracts as idiomatic Rust modules with a typed, multi‑method ABI defined in WIT. You work with familiar constructs (structs, methods, maps) and call signatures like `mint(ctx, n: integer)` or `balance(ctx, acc: string) -> option<integer>`. Testing feels like ordinary Rust: spin up an in‑memory runtime, call methods directly, assert results. Most mistakes surface as compiler errors or method‑signature mismatches rather than as byte‑level bugs at runtime.

**Move** offers stronger _theoretical_ safety for assets via its **linear resource model**. Values tagged as resources cannot be copied or implicitly dropped; ownership is explicit, moves are explicit, and access to on‑chain resources must be declared (with `acquires`). Combined with its bytecode verifier, Move can enforce some invariants such as conservation of supply or “no orphaned assets” by construction.

// - Rust quirks in Move
//  - In fact the whole (`key`, `store`, `drop`, `copy`) model is weird Rust semantics
//  - Move code with **resource types** and abilities; state lives in per‑address storage. You manipulate resources via `move_to`, `borrow_global(_mut)`, and you annotate functions with `acquires` for any resources they touch.
// - Performance
// - `move test` runs annotated test functions inside the Move VM with sandboxed storage. Great fidelity to on‑chain behavior, but the workflow is its own toolchain and semantics.
//  - cf. `cargo test` with its in‑memory runtime; call methods directly and assert on typed results. Very fast feedback; no chain or special harness is required for unit tests.