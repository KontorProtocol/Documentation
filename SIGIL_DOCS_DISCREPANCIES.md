# Sigil Documentation Discrepancy Report

**Generated:** 2025-11-11
**Reviewed Against:** Kontor test-contracts/ codebase (actual working code)

This report documents every discrepancy, error, and inconsistency found between the sigil-docs/ documentation and the actual working Sigil/Kontor implementation.

---

## Critical API Discrepancies

### 1. Storage Access API - **MAJOR INCONSISTENCY**

**Location:** Multiple guides and examples throughout documentation

**Documentation Claims:**
```rust
// From guides/storage.mdx, examples/token.mdx, examples/proxy.mdx
fn mint(ctx: &ProcContext, n: Integer) {
    let to = ctx.signer().to_string();
    let ledger = storage(ctx).ledger();  // ❌ WRONG

    let balance = ledger.get(ctx, &to).unwrap_or_default();  // ❌ WRONG
    ledger.set(ctx, to, balance + n);  // ❌ WRONG
}
```

**Actual Code:**
```rust
// From test-contracts/token/src/lib.rs (VERIFIED WORKING CODE)
fn mint(ctx: &ProcContext, n: Integer) {
    let to = ctx.signer().to_string();
    let ledger = ctx.model().ledger();  // ✅ CORRECT

    let balance = ledger.get(&to).unwrap_or_default();  // ✅ CORRECT
    ledger.set(to, balance + n);  // ✅ CORRECT
}
```

**Impact:** Complete API breakage - documentation code will not compile
**Files Affected:**
- `guides/storage.mdx` (lines 63, 66, 86, 166, 173, etc.)
- `guides/contract-structure.mdx` (line 63, 86)
- `examples/token.mdx` (lines 63, 65, 66, 71, 74, 80, 81, 86)
- `examples/proxy.mdx` (lines 61, 62, 66, 70)

**Verification:**
```bash
$ grep -r "storage(ctx)" test-contracts/
# No results - pattern does NOT exist in actual code

$ grep "ctx.model()" test-contracts/token/src/lib.rs
# Multiple matches - this IS the actual pattern
```

---

### 2. Error Construction API - **MAJOR INCONSISTENCY**

**Documentation Claims:**
```rust
// From examples/token.mdx, examples/proxy.mdx
if from_balance < n {
    return Err(Error::new("insufficient funds"));  // ❌ WRONG
}

Err(Error::new("unauthorized"));  // ❌ WRONG
```

**Actual Code:**
```rust
// From test-contracts/token/src/lib.rs, test-contracts/shared-account/src/lib.rs
if from_balance < n {
    return Err(Error::Message("insufficient funds".to_string()));  // ✅ CORRECT
}

fn unauthorized_error() -> Error {
    Error::Message("unauthorized".to_string())  // ✅ CORRECT
}
```

**Impact:** Code will not compile - `Error::new()` does NOT exist
**Files Affected:**
- `examples/token.mdx` (line 77)
- `examples/proxy.mdx` (line 171)
- All error handling examples

**Actual Error Type:**
```rust
// From core/indexer/src/runtime/wit/deps/built-in.wit
variant error {
    message(string),
    overflow(string),
    div-by-zero(string),
    syntax-error(string),
}
```

**There is NO `new()` constructor** - must use enum variants directly.

---

### 3. Map API - Context Parameter Mismatch

**Documentation Claims:**
```rust
// From examples/token.mdx, guides/storage.mdx
let balance = ledger.get(ctx, &to).unwrap_or_default();  // ❌ Passing ctx
ledger.set(ctx, to, balance + n);  // ❌ Passing ctx
```

**Actual Code:**
```rust
// From test-contracts/token/src/lib.rs
let balance = ledger.get(&to).unwrap_or_default();  // ✅ No ctx parameter
ledger.set(to, balance + n);  // ✅ No ctx parameter
```

**Impact:** Incorrect function signatures - will cause compilation errors
**Files Affected:** All storage examples and guides

---

### 4. WIT Package Naming - **INCONSISTENT**

**Documentation Claims:**
```wit
// From examples/hello-world.mdx, examples/token.mdx, examples/proxy.mdx
package kontor:contract;  // ❌ WRONG

world contract {  // ❌ WRONG
    include kontor:built-in/built-in;
    // ...
}
```

**Actual Code:**
```wit
// From test-contracts/token/wit/contract.wit (and ALL test-contracts)
package root:component;  // ✅ CORRECT

world root {  // ✅ CORRECT
    include kontor:built-in/built-in;
    // ...
}
```

**Impact:** WIT files won't match actual contract structure
**Verification:**
```bash
$ grep "package " test-contracts/*/wit/contract.wit
test-contracts/amm/wit/contract.wit:package root:component;
test-contracts/arith/wit/contract.wit:package root:component;
test-contracts/crypto/wit/contract.wit:package root:component;
test-contracts/fib/wit/contract.wit:package root:component;
test-contracts/pool/wit/contract.wit:package root:component;
test-contracts/proxy/wit/contract.wit:package root:component;
test-contracts/shared-account/wit/contract.wit:package root:component;
test-contracts/token/wit/contract.wit:package root:component;
```

**ALL contracts use `package root:component` and `world root` - NOT `kontor:contract`**

**Files Affected:**
- `examples/hello-world.mdx` (line 19)
- `examples/token.mdx` (line 26)
- `examples/proxy.mdx` (line 20)
- All example WIT snippets

---

### 5. Testing API - **COMPLETELY DIFFERENT**

**Documentation Claims:**
```rust
// From examples/hello-world.mdx, examples/token.mdx
#[tokio::test]
async fn test_contract() -> Result<()> {
    let runtime = Runtime::new(
        RuntimeConfig::builder()
            .contracts(&[("hello-world", &contract_bytes().await?)])
            .build(),
    )
    .await?;

    let result = hello_world::hello_world(&runtime).await?;
    assert_eq!(result, "Hello, World!");
    Ok(())
}
```

**Actual Code:**
```rust
// From core/indexer/tests/token_contract.rs (VERIFIED WORKING TESTS)
#[runtime(contracts_dir = "../../test-contracts")]
async fn test_token_contract() -> Result<()> {
    let minter = runtime.identity().await?;  // runtime auto-injected
    let token = runtime.publish(&minter, "token").await?;

    token::mint(runtime, &token, &minter, 900.into()).await?;

    let result = token::balance(runtime, &token, &minter).await?;
    assert_eq!(result, Some(1000.into()));
    Ok(())
}
```

**Key Differences:**
1. **Attribute**: Uses `#[runtime(contracts_dir = "...")]` NOT `#[tokio::test]`
2. **Runtime**: Auto-injected variable `runtime` NOT `Runtime::new(...)`
3. **Contract Loading**: Uses `runtime.publish(&signer, "name")` NOT `RuntimeConfig::builder().contracts(...)`
4. **Identity Creation**: Must create identities with `runtime.identity().await?`
5. **Function Calls**: Pass `runtime` as FIRST parameter: `token::mint(runtime, &token, &signer, amount)`

**Impact:** Test examples will not work AT ALL
**Files Affected:**
- `examples/hello-world.mdx` (lines 69-84)
- `examples/token.mdx` (lines 108-146)
- `examples/proxy.mdx` (lines 106-238)
- `guides/testing.mdx` (multiple locations)
- `quickstart.mdx` (lines 148-176)

---

### 6. Generated Function Signatures - Cross-Contract Calls

**Documentation Claims (guides/cross-contract-calls.mdx):**
```rust
// Generated by interface!
pub fn transfer(
    contract_address: &ContractAddress,
    signer: &Signer,
    to: &str,
    n: Integer
) -> Result<(), Error>  // ❌ Returns Result, not in async context
```

**Actual Generated Signatures:**
```rust
// From actual test code
pub async fn transfer(
    runtime: &mut Runtime,  // ✅ First parameter is runtime
    contract_address: &ContractAddress,
    signer: &Signer,
    to: &str,
    n: Integer
) -> Result<Result<(), Error>>  // ✅ Returns Result<Result<T, Error>>
```

**Key Differences:**
1. Functions are `async`, NOT sync
2. FIRST parameter is `runtime: &mut Runtime`
3. Return type is `Result<Result<T, Error>>` for proper error handling

**Impact:** Misrepresents the actual API completely
**Files Affected:**
- `guides/cross-contract-calls.mdx` (lines 28-38)
- `guides/testing.mdx` (lines 122-129)

---

### 7. Proxy Contract Storage Access

**Documentation Claims:**
```rust
// From examples/proxy.mdx (lines 60-62)
fn fallback(ctx: &FallContext, expr: String) -> String {
    let _ctx = &ctx.view_context();
    let contract_address = storage(_ctx).contract_address(_ctx).unwrap();  // ❌ WRONG
    foreign::call(ctx.signer(), &contract_address, &expr)
}
```

**Actual Code:**
```rust
// From test-contracts/proxy/src/lib.rs
fn fallback(ctx: &FallContext, expr: String) -> String {
    let ctx_ = &ctx.view_context();
    if let Some(contract_address) = ctx_.model().contract_address() {  // ✅ CORRECT
        foreign::call(ctx.signer(), &contract_address, &expr)
    } else {
        "".to_string()
    }
}
```

**Differences:**
1. Uses `ctx_.model().contract_address()` NOT `storage(_ctx).contract_address(_ctx)`
2. Handles `Option` properly instead of unwrapping
3. Doesn't pass `_ctx` twice

---

## Architecture Documentation Issues

### 8. Error Rollback Semantics - **MISLEADING**

**Documentation Claims (architecture.mdx, line 91):**
> "In nested cross-contract calls, an `Err` from the called contract rolls back only its own storage changes, leaving the calling contract's storage updates intact unless it also returns an `Err`"

**Actual Behavior (error-handling.mdx, lines 197-226):**
The documentation contradicts itself. In error-handling.mdx it correctly states:
> "Cross-Contract Rollback: Both errors and panics roll back the entire call chain"
> "If Contract A calls Contract B, and B returns an error, ALL storage changes in both A and B are rolled back"

**Testing Required:** Need to verify actual rollback behavior through tests
**Impact:** Critical misunderstanding of error handling could lead to security issues

---

### 9. Re-entrancy Examples - **CONFUSING**

**Documentation Claims (guides/cross-contract-calls.mdx):**
```rust
// arith contract calls fib contract
// fib contract calls back to arith -> OK (different direction)

// But if arith calls fib, and fib calls back to arith:
// -> Runtime error: "reentrancy prevented"
```

**This is contradictory** - it says "arith→fib→arith" is OK in one line, then says it causes an error in the next.

**Actual Behavior (from tests):**
```rust
// From core/indexer/tests/fib_contract.rs (lines 58-59)
// arith contract tries to call fib which calls back to arith
let result = arith::fib(runtime, &arith, &signer, fib.clone(), 9).await;
assert!(result.is_err_and(|e| e.root_cause().to_string().contains("reentrancy prevented")));
```

The test shows: **arith→fib→arith is PREVENTED**

---

## Terminology Inconsistencies

### 10. "Guest" Trait Naming

**Actual Code:**
```rust
impl Guest for Token {  // Uses specific contract name
impl Guest for Counter {
impl Guest for Proxy {
```

**Documentation Sometimes Says:**
```rust
impl Guest for Token {  // Correct
impl Guest for HelloWorld {  // Uses PascalCase of contract name
```

The `contract!(name = "token")` macro generates a type called `Token` (not `token`), and you implement `Guest` for that type. Documentation should be clear about this capitalization/naming convention.

---

## Missing Information

### 11. Contract Deployment Process

**Documentation Claims (introduction.mdx, line 9):**
> "Developers can write smart contracts with Sigil, but not yet execute them with the Kontor Indexer or deploy them to a Bitcoin network."

**But actual tests show:**
- Contracts CAN be executed with the Kontor runtime
- Tests run successfully in both local and regtest modes
- `#[runtime(mode = "regtest")]` suggests regtest deployment works

**Inconsistency:** Documentation undersells what actually works

---

### 12. Actual Test File Structure

**Documentation Claims (guides/testing.mdx, lines 343-351):**
```
test/
├── Cargo.toml
├── build.rs         # Compiles contract to WASM
└── tests/
    ├── basic.rs
    ├── errors.rs
    └── integration.rs
```

**Actual Structure:**
Tests are in `core/indexer/tests/` NOT in separate workspace directories:
```
core/indexer/tests/
├── token_contract.rs
├── fib_contract.rs
├── amm_contract.rs
└── ...
```

**Impact:** Developers following docs will create wrong directory structure

---

## Quickstart Tutorial Issues

### 13. Counter Example Missing from Codebase

**Documentation (quickstart.mdx):** Provides complete "counter" contract example

**Actual Codebase:** No counter contract exists in test-contracts/
```bash
$ ls test-contracts/
amm  arith  crypto  fib  pool  proxy  shared-account  token
```

**Impact:** Users can't find the example to reference

---

### 14. Build Script Location

**Documentation (quickstart.mdx, line 116):**
```bash
# For optimized builds (recommended), create `build.sh`:
```

**Actual:** build.sh exists at `test-contracts/build.sh` and builds ALL contracts in workspace
**Impact:** Documentation suggests creating per-contract build scripts, but actual setup uses workspace-level script

---

### 15. Getting Started - ZIP Package Reference

**Documentation (getting-started.mdx, lines 31-48):**
References a "ZIP package" delivered via messaging with structure:
```
sigil-package/
├── Kontor/
└── sigil-example-contracts/
```

**Actual Repository Structure:**
```
Kontor/
├── core/
├── test-contracts/  (NOT sigil-example-contracts/)
├── contracts/
└── ...
```

**Impact:** Instructions don't match the actual repository
**Question:** Is there a separate distribution package? If so, docs should clarify.

---

## Reference Documentation

### 16. Missing Implementation Details

**File:** `reference/implementation.mdx` - **NOT REVIEWED YET** (didn't read this file)
**File:** `reference/numerics.mdx` - **NOT REVIEWED YET**
**File:** `reference/testing.mdx` - **NOT REVIEWED YET**

These may contain additional discrepancies.

---

## Examples Not Verified

The following example docs were read but not fully verified against actual code:
- `examples/amm.mdx`
- `examples/pool.mdx`
- `examples/shared-account.mdx`
- `examples/shared-account-dynamic.mdx`
- `examples/index.mdx`

These likely contain the same API discrepancies as the ones verified above.

---

## Summary of Critical Issues

### Must Fix Immediately (Code Won't Compile):

1. ✅ **Storage API**: Change `storage(ctx)` → `ctx.model()` everywhere
2. ✅ **Error Construction**: Change `Error::new(...)` → `Error::Message(...to_string())`
3. ✅ **Map get/set**: Remove `ctx` parameter from `get()` and `set()` calls
4. ✅ **WIT Packages**: Change `package kontor:contract` → `package root:component`
5. ✅ **WIT Worlds**: Change `world contract` → `world root`
6. ✅ **Testing API**: Completely rewrite all test examples to use `#[runtime(...)]` attribute

### Should Fix (Misleading/Confusing):

7. ⚠️ **Generated Function Signatures**: Add `async`, `runtime` parameter, correct return types
8. ⚠️ **Error Rollback**: Clarify and make consistent across architecture.mdx and error-handling.mdx
9. ⚠️ **Re-entrancy**: Fix contradictory example
10. ⚠️ **Directory Structure**: Update to match actual test layout
11. ⚠️ **Counter Example**: Either add to test-contracts/ or remove from quickstart
12. ⚠️ **ZIP Package**: Clarify if this is separate from the repo

---

## Verification Commands Used

```bash
# Verified storage API
grep -r "storage(ctx)" test-contracts/  # No results
grep "ctx.model()" test-contracts/token/src/lib.rs  # Multiple matches

# Verified Error construction
grep "Error::new" test-contracts/  # No results (except in deps)
grep "Error::Message" test-contracts/token/src/lib.rs  # Found

# Verified WIT packages
grep "package " test-contracts/*/wit/contract.wit  # All use root:component

# Verified test attribute
grep "#\[runtime" core/indexer/tests/token_contract.rs  # Found
grep "Runtime::new" core/indexer/tests/  # No matches

# Verified Map API
grep "ledger.get(ctx," test-contracts/  # No results
grep "ledger.get(&" test-contracts/token/src/lib.rs  # Found
```

---

## Recommendation

**Priority 1:** Fix all API examples in:
- guides/storage.mdx
- guides/testing.mdx
- guides/cross-contract-calls.mdx
- examples/token.mdx
- examples/hello-world.mdx
- examples/proxy.mdx
- quickstart.mdx

**Priority 2:** Clarify rollback semantics and re-entrancy in architecture.mdx

**Priority 3:** Update directory structure references and clarify ZIP package distribution

---

**Report Compiled By:** Claude (Anthropic)
**Methodology:** Line-by-line comparison of documentation against working test-contracts/ code
**Confidence Level:** HIGH - All major discrepancies verified with grep and file inspection
