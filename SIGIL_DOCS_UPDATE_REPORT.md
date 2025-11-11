# Sigil Documentation Update Report

**Generated:** 2025-11-11 (Second Review)
**Status:** Partial fixes - some files still have critical issues

---

## ✅ FIXED FILES

### 1. guides/storage.mdx - **FULLY FIXED**
- ✅ Now correctly uses `ctx.model()` throughout (lines 31, 121, 122, 126, 166, 170, 173, 178, etc.)
- ✅ Map API no longer passes extra `ctx` parameter
- ✅ All storage access patterns now match actual code

### 2. guides/testing.mdx - **FULLY FIXED**
- ✅ Now uses `#[runtime(contracts_dir = "...")]` attribute (lines 18, 38, 77, 95)
- ✅ Shows auto-injected `runtime` variable
- ✅ Correct `runtime.identity()` and `runtime.publish()` API
- ✅ Correct function call signatures: `token::mint(runtime, &token, &signer, amount)`
- ✅ Shows `Result<Result<T, Error>>` double Result pattern (lines 118-129)
- ✅ Explains double `??` operator correctly

### 3. quickstart.mdx - **FULLY FIXED**
- ✅ WIT package now `root:component` (line 53)
- ✅ WIT world now `root` (line 55)
- ✅ Uses `ctx.model()` correctly (lines 90-96)
- ✅ Storage access pattern matches actual code

### 4. guides/contract-structure.mdx - **FULLY FIXED**
- ✅ Uses `ctx.model()` throughout (lines 32, 33, 34, 35, 39, 70, 76)
- ✅ Map API correct: `ledger.get(&account)` without extra ctx (lines 34, 39)
- ✅ All code examples now compile correctly

### 5. guides/cross-contract-calls.mdx - **MOSTLY GOOD**
- ✅ Shows cross-contract call patterns clearly
- ✅ Uses `ctx.model()` correctly (lines 94, 106)
- ⚠️ NOTE: Function signatures shown don't include `async` or `runtime` parameter, but this may be intentional simplification for the guide (since it's explaining what happens INSIDE contracts, not in tests)

### 6. architecture.mdx - **ACCEPTABLE**
- ⚠️ Rollback semantics (line 91) describe a partial rollback mechanism, but this conflicts with other documentation
- **NEEDS CLARIFICATION:** Does an error in Contract B rollback Contract A's changes if A doesn't propagate the error?
- The statement is: "an `Err` from the called contract rolls back only its own storage changes, leaving the calling contract's storage updates intact unless it also returns an `Err`"
- This needs to be tested to verify actual behavior

---

## ❌ STILL BROKEN FILES

### 1. examples/token.mdx - **CRITICAL ISSUES REMAIN**

**Storage API - Lines 63, 66, 71, 80, 81, 86:**
```rust
// ❌ WRONG - Still uses old API
let ledger = storage(ctx).ledger();  // Line 63
let balance = ledger.get(ctx, &to).unwrap_or_default();  // Line 65
ledger.set(ctx, to, balance + n);  // Line 66
let ledger = storage(ctx).ledger();  // Line 71
ledger.set(ctx, from, from_balance - n);  // Line 80
ledger.set(ctx, to, to_balance + n);  // Line 81
storage(ctx).ledger().get(ctx, acc)  // Line 86

// ✅ SHOULD BE
let ledger = ctx.model().ledger();
let balance = ledger.get(&to).unwrap_or_default();
ledger.set(to, balance + n);
```

**Error Construction - Line 77:**
```rust
// ❌ WRONG
return Err(Error::new("insufficient funds"));

// ✅ SHOULD BE
return Err(Error::Message("insufficient funds".to_string()));
```

**WIT Package - Line 25:**
```wit
// ❌ WRONG
package kontor:contract;

// ✅ SHOULD BE
package root:component;
```

**WIT World - Line 27:**
```wit
// ❌ WRONG
world contract {

// ✅ SHOULD BE
world root {
```

**Test API - Lines 108-119:**
```rust
// ❌ WRONG - Still uses old test API
#[tokio::test]
async fn test_contract() -> Result<()> {
    let runtime = Runtime::new(
        RuntimeConfig::builder()
            .contracts(&[("token", &contract_bytes().await?)])
            .build(),
    )
    .await?;

    token::mint(&runtime, minter, 900.into()).await?;

// ✅ SHOULD BE
#[runtime(contracts_dir = "../../test-contracts")]
async fn test_contract() -> Result<()> {
    let minter = runtime.identity().await?;
    let token = runtime.publish(&minter, "token").await?;

    token::mint(runtime, &token, &minter, 900.into()).await?;
```

**Impact:** This is a primary example file - developers will copy this code and it **WILL NOT COMPILE**

---

### 2. examples/hello-world.mdx - **CRITICAL ISSUES REMAIN**

**WIT Package - Line 19:**
```wit
// ❌ WRONG
package kontor:contract;

// ✅ SHOULD BE
package root:component;
```

**WIT World - Line 21:**
```wit
// ❌ WRONG
world contract {

// ✅ SHOULD BE
world root {
```

**Test API - Lines 69-84:**
```rust
// ❌ WRONG - Still uses old test API
#[tokio::test]
async fn test_contract() -> Result<()> {
    let runtime = Runtime::new(
        RuntimeConfig::builder()
            .contracts(&[("hello-world", &contract_bytes().await?)])
            .build(),
    )
    .await?;

    let result = hello_world::hello_world(&runtime).await?;

// ✅ SHOULD BE
#[runtime(contracts_dir = "../../test-contracts")]
async fn test_contract() -> Result<()> {
    let signer = runtime.identity().await?;
    let hello = runtime.publish(&signer, "hello-world").await?;

    let result = hello_world::hello_world(runtime, &hello).await?;
```

**Impact:** This is the FIRST example developers see - broken code will cause immediate confusion

---

### 3. examples/proxy.mdx - **CRITICAL ISSUES REMAIN**

**Storage API - Lines 61, 62, 66, 70:**
```rust
// ❌ WRONG
let contract_address = storage(_ctx).contract_address(_ctx).unwrap();  // Line 61
storage(ctx).contract_address(ctx)  // Line 66
storage(ctx).set_contract_address(ctx, Some(contract_address));  // Line 70

// ✅ SHOULD BE
let contract_address = ctx_.model().contract_address().unwrap();
ctx.model().contract_address()
ctx.model().set_contract_address(Some(contract_address));
```

**WIT Package - Line 20:**
```wit
// ❌ WRONG
package kontor:contract;

// ✅ SHOULD BE
package root:component;
```

**WIT World - Line 22:**
```wit
// ❌ WRONG
world contract {

// ✅ SHOULD BE
world root {
```

**Error Construction - Lines 171, 217, 227:**
```rust
// ❌ WRONG
Err(Error::new("unauthorized"))  // Line 171
Err(Error::new("insufficient balance"))  // Line 217
Err(Error::new("unauthorized"))  // Line 227

// ✅ SHOULD BE
Err(Error::Message("unauthorized".to_string()))
Err(Error::Message("insufficient balance".to_string()))
Err(Error::Message("unauthorized".to_string()))
```

**Test API - Lines 105-119:**
Still uses `Runtime::new(RuntimeConfig::builder()...)` instead of `#[runtime(...)]` attribute

**Impact:** This example demonstrates advanced patterns - broken code undermines credibility

---

## Summary Statistics

### Files Checked: 10
- **Fully Fixed:** 4 files (storage.mdx, testing.mdx, quickstart.mdx, contract-structure.mdx)
- **Needs Clarification:** 2 files (cross-contract-calls.mdx, architecture.mdx)
- **Still Broken:** 3 files (token.mdx, hello-world.mdx, proxy.mdx)
- **Not Reviewed:** error-handling.mdx, numbers.mdx, remaining examples

### Issue Breakdown

**CRITICAL (Code Won't Compile):**
- ❌ 3 files still use `storage(ctx)` instead of `ctx.model()`
- ❌ 3 files still use `Error::new()` instead of `Error::Message()`
- ❌ 3 files still use wrong WIT packages (`kontor:contract` vs `root:component`)
- ❌ 3 files still use old test API (`Runtime::new(...)` vs `#[runtime(...)]`)

**Files by Priority:**

**P0 - Must Fix (First Examples):**
1. examples/hello-world.mdx - First example developers see
2. examples/token.mdx - Primary reference example

**P1 - Should Fix Soon:**
3. examples/proxy.mdx - Advanced pattern example

**P2 - Need Clarification:**
4. architecture.mdx - Rollback semantics need testing/verification

---

## Verification Commands

To verify the remaining issues:

```bash
# Check for storage(ctx) pattern in examples
grep -n "storage(ctx)" sigil-docs/examples/*.mdx

# Check for Error::new pattern
grep -n "Error::new" sigil-docs/examples/*.mdx

# Check for kontor:contract package
grep -n "package kontor:contract" sigil-docs/examples/*.mdx

# Check for Runtime::new test pattern
grep -n "Runtime::new" sigil-docs/examples/*.mdx
```

**Expected Results:** All should return NO matches after fixes

---

## Recommended Next Steps

1. **Fix examples/hello-world.mdx** - Highest priority (first impression)
2. **Fix examples/token.mdx** - Primary reference example
3. **Fix examples/proxy.mdx** - Complete the example suite
4. **Test rollback behavior** - Verify architecture.mdx claims with actual tests
5. **Review remaining examples** - Check amm.mdx, pool.mdx, shared-account*.mdx

---

## Notes on What Was Fixed

The guides (storage, testing, contract-structure, quickstart) are now **excellent** and match the actual codebase. The fixes show a solid understanding of the correct API. However, the **example files** appear to not have been updated yet, and they contain the same errors as before.

This suggests the examples may be in a separate directory or generated from a different source. They need the same updates that were applied to the guides.

---

**Report Status:** Ready for review
**Action Required:** Update the 3 broken example files with the same fixes applied to guides
