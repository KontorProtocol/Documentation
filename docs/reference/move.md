---
sidebar_position: 4
title: Kontor vs. Move
---
# Kontor vs. Move

The biggest difference between the Sigil and Move smart contract systems is that Sigil is an embedded DSL in Rust designed to make writing contracts feel like writing regular Rust code. Sigil's emphasis is on simplicity, readability, and overall developer experience. Move, by contrast, is a novel DSl with Rust-like syntax that implements an idiosyncratic resource-oriented programming model designed to support greater theoretical performance and safety guarantees but requires learning a significant number of new concepts and implementing complex workflows to fit the Move model. Because of precisely these limitations, Move has split into at least two different main versions: Aptos Move and Sui Move, each of which has its own (significant) variations in programming model, and incompatible tooling. As such, the Move ecosystem is fragmented, and the tooling is even less mature than it would be otherwise.

**Sigil** is much easier to write: it lets you author contracts as idiomatic Rust modules with a typed, multi‑method ABI defined in WIT. You work with familiar constructs (structs, methods, maps) and call signatures like `mint(ctx, n: integer)` or `balance(ctx, acc: string) -> option<integer>`. Testing feels like ordinary Rust: spin up an in‑memory runtime, call methods directly, assert results. Most mistakes surface as compiler errors or method‑signature mismatches rather than as byte‑level bugs at runtime.

**Move** offers stronger _theoretical_ safety for assets via its **linear resource model**. Values tagged as resources cannot be copied or implicitly dropped; ownership is explicit, moves are explicit, and access to on‑chain resources must be declared (with `acquires`). Combined with its bytecode verifier, Move can enforce some invariants such as conservation of supply or “no orphaned assets” by construction.



## Safety and Formal Verification

A significant fraction of Move's design quirks were introduced to support formal verification. Formal verification, while a very interesting technology, is not the silver bullet for code correctness that it is often made out to be. Verification of any meaningful amount of code is labor-intensive and highly technical, which explains its limited adoption up to the present day. Only a few Aptos Move dApps have used formal verification so far, and Sui Move had no support until this year. (https://blog.sui.io/asymptotic-move-prover-formal-verification/). Even when formal verification is employed, it is usually practical to verify only a small core set of functions and simple properties they should exhibit.

Along similar lines, while Move’s resource model makes it impossible to copy or implicitly drop assets—which does remove a class of bugs—it does not verify business intent in general. Mis‑scoped authorities, mispriced fees, incorrect caps, or time/sequence errors still compile and must be guarded explicitly. You also pay ceremony—abilities, `acquires`, and resource plumbing that ripple through refactors. In Move, explicitly declaring `acquires` shapes is a benefit for auditability and scheduling, but it adds significant friction to development.


- **Dialect and ecosystem realities**: Aptos and Sui diverge on storage/object models, APIs, and idioms. Portability is non‑trivial, libraries are younger, and you’ll spend time on chain‑specific packaging and tooling. These are solvable problems, but they reduce the day‑to‑day advantage of the underlying theory.

Sigil, by contrast, optimizes for **developer speed and clarity**: idiomatic Rust, a **typed, multi‑method ABI** (WIT), and structured storage. Most mistakes surface as compiler errors or signature mismatches rather than byte‑level bugs at runtime. You still write invariants, but you do so in straightforward Rust with tight feedback loops.

# Smart Contracts

## Programming Model and ABI

With Move, you publish a module that exposes `public`/`entry` functions. The API is the set of functions you choose to expose, and their access patterns are governed by abilities (`copy`, `drop`, `store`, `key`), the `signer` model, and explicit `acquires` declarations for any on‑chain resources a function reads/writes. Resource values are linear: they cannot be copied or implicitly dropped; you move them or you borrow them.

Sigil’s core idea—first‑class, typed contract methods—makes contracts read like ordinary Rust services: multiple named entrypoints with explicit parameter and return types, and interface boundaries the compiler can check. You write methods such as `init`, `mint`, `transfer`, or `balance` with clear signatures, and the ABI is generated from WIT. This yields self‑documenting interfaces, predictable error surfaces, and straightforward scaffolding (client SDKs, mocks, auto‑bindings) without bespoke conventions.

## Storage

Move places state in global storage keyed by address and type (Aptos), or as first‑class objects with ownership and IDs (Sui). You materialize or borrow resources via primitives like `move_to`, `borrow_global(_mut)`, and object accessors; functions must declare which resource types they `acquire` from storage.

Sigil favors **structured, typed storage**: fields and maps that mirror your data model, not ad‑hoc key strings. You load a storage root, access typed collections (e.g., a map from account to balance), update values, and return typed results. This reduces accidental key drift and makes refactors predictable.

## Safety

Move offers stronger, theoretical safety for assets via the resource model. Linearity and abilities eliminate whole classes of bugs (duplication, implicit drops), and the VM’s bytecode verifier enforces these properties. If you invest in specifications, the Move Prover can establish deep invariants like conservation of supply or absence of orphaned assets—but those proofs are costly to write and maintain, so they are typically reserved for small, critical components.

Sigil inherits Rust/WASM **memory and type safety** and adds a **typed ABI** that makes malformed calls unrepresentable in most client code. Business invariants (supply conservation, authorization policies, etc.) are explicit in your methods and enforced by tests; you are not working against a linear type system, which keeps the ergonomics high.

## Tooling, Testing, and DX

A Sigil‑style contract is just **Rust with a clean contract boundary**. The natural loop is `cargo test` against fast in‑memory harnesses; you call methods directly and assert on typed results. IDEs and Rust tooling carry most of the load, so iteration speed is high.

With Move, you use `move build`/`move test` (and chain‑specific CLIs) to run tests inside the VM with sandboxed storage. Fidelity to on‑chain semantics is excellent, and resource mistakes surface early, but the workflow uses its own toolchain, semantics, and packaging model.

## Performance and Resource Contention

Move chains (Aptos, Sui) are designed for high throughput with schedulers that parallelize transactions when they touch disjoint resources/objects. Contention appears when many transactions access the same resources; those serialize, but unrelated flows can proceed in parallel.

Sigil’s account‑like storage model avoids UTXO‑style single‑use conflicts and keeps concurrency predictable at the key/collection level. Integration with Bitcoin (PSBTs, atomic swaps) is explicit and ergonomic; you design workflows that anchor to Bitcoin where it matters, without forcing every state mutation to be a Bitcoin transaction.

---

## How it feels to write code

**Sigil (Rust + WIT on Bitcoin/Kontor)**

- **Interface**: a WIT world that exports **named, typed entrypoints** (e.g., `init`, `mint`, `transfer`, `balance`) that take a `proc-context` or `view-context` and ordinary types (`integer`, `string`, `result`, `option`).
- **Implementation**: idiomatic Rust with a small macro surface (`contract!`, `StorageRoot`), plus a storage API (e.g., a typed `Map`). You retrieve from storage, update fields or maps, and return typed values.
- **Testing**: `cargo test` with an in‑memory runtime; call methods directly and assert on typed results. Very fast feedback; no chain or special harness is required for unit tests.
- **Learning curve**: if you know Rust, you’re productive quickly; you mainly learn Sigil’s contexts and storage helpers.

**Move (resource‑oriented VM on Aptos/Sui)**

- **Interface**: a module with `public`/`entry` functions; no separate interface file—the public API is the functions you expose.
- **Implementation**: resource types with abilities; state lives in global storage (Aptos) or as objects (Sui). You use `move_to`, `borrow_global(_mut)`, object APIs, and annotate with `acquires` for resources you touch.
- **Testing**: `move test` runs annotated tests inside the VM with sandboxed storage. Great fidelity to on‑chain behavior, with its own toolchain semantics.
- **Learning curve**: you internalize abilities (`key`, `store`, `drop`, `copy`), linearity (no implicit copy/drop), the signer model, and `acquires` discipline.

---

## Day‑to‑day trade‑offs you’ll feel

- **Velocity vs. ceremony**: Sigil’s ergonomics (Rust, typed methods, straightforward storage) make implementation and refactoring fast. Move’s guarantees come with more ceremony (abilities, `acquires`, resource plumbing), and changes ripple through types and call sites.
- **Testing loop**: Sigil’s in‑process tests are extremely tight loops—great for iterating on business rules. Move’s tests catch resource‑model mistakes earlier but with a slightly heavier harness than typical Rust tests.
- **Failure surfaces**: With Sigil, the compiler and ABI catch many issues; economic invariants rely on explicit checks and tests. With Move, many asset‑handling bugs are **unrepresentable**; you spend time communicating intent to the type system so your program type‑checks.













Below is a concise, side‑by‑side on what that means in practice.

---

## How it feels to write code

**Move (resource‑oriented VM on Aptos/Sui)**

* **Interface**: a **module** with `public`/`entry` functions. No separate interface file—API is whatever functions you expose.
* **Implementation**: Move code with **resource types** and abilities; state lives in per‑address storage. You manipulate resources via `move_to`, `borrow_global(_mut)`, and you annotate functions with `acquires` for any resources they touch.
* **Testing**: `move test` runs annotated test functions inside the Move VM with sandboxed storage. Great fidelity to on‑chain behavior, but the workflow is its own toolchain and semantics.

---

## Safety model: what you get “for free”

**Sigil**

* **Memory & type safety** from Rust/WASM; a **typed ABI** (via WIT) prevents malformed calls at the boundary.
* **Reasoning comfort**: business logic is straightforward, with fewer language‑level constraints to work around.
* **What you still own**: economic invariants (e.g., supply conservation, no negative balances) are the *contract’s* responsibility; the language doesn’t encode linear assets for you.

**Move**

* **Linearity**: resources cannot be duplicated or silently dropped—enforced by the type system and the VM.
* **Explicit effects**: `acquires` declarations and borrow rules make global state access auditable and schedule‑able; you can often *prove* supply conservation and absence of orphaned assets.
* **What you pay**: extra ceremony at every step (abilities, `acquires`, resource plumbing), and a narrower set of patterns for data flow.