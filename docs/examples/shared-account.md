---
sidebar_position: 3
title: 3. Shared Account
---

# Shared Account

This example demonstrates a simple multi-tenant shared account introducing static contract imports, complex storage structs, authorization logic, ID generation, and cross-contract calls.

## WIT Interface
- `open`: Creates an account with a deposit and tenants, returning an ID or error
- `deposit`/`withdraw`: Modifies the account balance
- `balance`: Queries the account balance
- `tenants`: Lists account participants

```wit
package kontor:contract;

world contract {
    include kontor:built-in/built-in;
    use kontor:built-in/context.{view-context, proc-context, signer};
    use kontor:built-in/error.{error};
    use kontor:built-in/numbers.{integer, decimal};

    export init: func(ctx: borrow<proc-context>);

    export open: func(ctx: borrow<proc-context>, n: integer, other-tenants: list<string>) -> result<string, error>;

    export deposit: func(ctx: borrow<proc-context>, account-id: string, n: integer) -> result<_, error>;

    export withdraw: func(ctx: borrow<proc-context>, account-id: string, n: integer) -> result<_, error>;

    export balance: func(ctx: borrow<view-context>, account-id: string) -> option<integer>;

    export tenants: func(ctx: borrow<view-context>, account-id: string) -> option<list<string>>;
}
```

## Rust Implementation
- The `import!` macro generates an interface for cross-contract calls to the `token` contract, as seen in the test environment previously.
- The `StorageRoot` macro, used for the root storage type, and the `Storage` macro, used for nested storage types in the `Account` struct, enable persistent storage.
- `other_tenants` uses `Map<String, bool>` because the storage layer does not currently support list types, and `Map` provides a limited interface with the `keys` method for iteration. Even with a `List` type using a `Map` here could make sense. Instead of `bool` an `enum` or `struct` that defines the their "role" in the account could be written.
- `authorized` Verifies procedure permissions
- `open`: Verifies token balance, generates an ID using `crypto::generate_id()`, sets the account, and transfers tokens to `ctx.contract_signer()`
- `deposit`/`withdraw`: Authorize the caller, verify balances, update storage, and call the token contract following CEI pattern
- `balance`/`tenants`: Query the storage for account details

```rust
use stdlib::*;

contract!(name = "shared-account");

import!(
    name = "token",
    height = 0,
    tx_index = 0,
    path = "../token/contract/wit"
);

#[derive(Clone, Default, Storage)]
struct Account {
    pub other_tenants: Map<String, bool>,
    pub balance: Integer,
    pub owner: String,
}

#[derive(Clone, Default, StorageRoot)]
struct SharedAccountStorage {
    pub accounts: Map<String, Account>,
}

fn authorized(ctx: &ProcContext, account: &AccountWrapper) -> bool {
    account.owner(ctx) == ctx.signer().to_string()
        || account
            .other_tenants()
            .get(ctx, ctx.signer().to_string())
            .is_some_and(|b| b)
}

fn insufficient_balance_error() -> Error {
    Error::new("insufficient balance")
}

fn unauthorized_error() -> Error {
    Error::new("unauthorized")
}

fn unknown_error() -> Error {
    Error::new("unknown account")
}

impl Guest for SharedAccount {
    fn init(ctx: &ProcContext) {
        SharedAccountStorage::default().init(ctx);
    }

    fn open(ctx: &ProcContext, n: Integer, other_tenants: Vec<String>) -> Result<String, Error> {
        let balance =
            token::balance(&ctx.signer().to_string()).ok_or(insufficient_balance_error())?;
        if balance < n {
            return Err(insufficient_balance_error());
        }
        let account_id = crypto::generate_id();
        storage(ctx).accounts().set(
            ctx,
            account_id.clone(),
            Account {
                balance: n,
                owner: ctx.signer().to_string(),
                other_tenants: Map::new(
                    &other_tenants
                        .into_iter()
                        .map(|t| (t, true))
                        .collect::<Vec<_>>(),
                ),
            },
        );
        token::transfer(ctx.signer(), &ctx.contract_signer().to_string(), n)?;
        Ok(account_id)
    }

    fn deposit(ctx: &ProcContext, account_id: String, n: Integer) -> Result<(), Error> {
        let balance =
            token::balance(&ctx.signer().to_string()).ok_or(insufficient_balance_error())?;
        if balance < n {
            return Err(insufficient_balance_error());
        }
        let account = storage(ctx)
            .accounts()
            .get(ctx, account_id)
            .ok_or(unknown_error())?;
        if !authorized(ctx, &account) {
            return Err(unauthorized_error());
        }
        account.set_balance(ctx, account.balance(ctx) + n);
        token::transfer(ctx.signer(), &ctx.contract_signer().to_string(), n)
    }

    fn withdraw(ctx: &ProcContext, account_id: String, n: Integer) -> Result<(), Error> {
        let account = storage(ctx)
            .accounts()
            .get(ctx, account_id)
            .ok_or(unknown_error())?;
        if !authorized(ctx, &account) {
            return Err(unauthorized_error());
        }
        let balance = account.balance(ctx);
        if balance < n {
            return Err(insufficient_balance_error());
        }
        account.set_balance(ctx, balance - n);
        token::transfer(ctx.contract_signer(), &ctx.signer().to_string(), n)
    }

    fn balance(ctx: &ViewContext, account_id: String) -> Option<Integer> {
        storage(ctx)
            .accounts()
            .get(ctx, account_id)
            .map(|a| a.balance(ctx))
    }

    fn tenants(ctx: &ViewContext, account_id: String) -> Option<Vec<String>> {
        storage(ctx).accounts().get(ctx, account_id).map(|a| {
            [a.owner(ctx)]
                .into_iter()
                .chain(a.other_tenants().keys(ctx))
                .collect()
        })
    }
}
```

## Testing
- The `import!` macro generates interfaces for both the `shared-account` and `token` contracts. Although the test primarily interacts with the `token` contract through `shared-account` calls, it must first `mint` tokens.
- `dep_contract_bytes` loads the WebAssembly bytes for the "imported" `token` contract.

```rust
#[cfg(test)]
mod tests {
    use testlib::*;

    import!(
        name = "shared-account",
        height = 0,
        tx_index = 0,
        path = "contract/wit",
    );

    import!(
        name = "token",
        height = 0,
        tx_index = 0,
        path = "../token/contract/wit",
    );

    #[tokio::test]
    async fn test_contract() -> Result<()> {
        let runtime = Runtime::new(
            RuntimeConfig::builder()
                .contracts(&[
                    ("shared-account", &contract_bytes().await?),
                    ("token", &dep_contract_bytes("token").await?),
                ])
                .build(),
        )
        .await?;

        let alice = "alice";
        let bob = "bob";
        let claire = "claire";
        let dara = "dara";

        token::mint(&runtime, alice, 100.into()).await?;

        let account_id =
            shared_account::open(&runtime, alice, 50.into(), vec![bob, dara]).await??;

        let result = shared_account::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(50.into()));

        shared_account::deposit(&runtime, alice, &account_id, 25.into()).await??;

        let result = shared_account::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(75.into()));

        shared_account::withdraw(&runtime, bob, &account_id, 25.into()).await??;

        let result = shared_account::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(50.into()));

        shared_account::withdraw(&runtime, alice, &account_id, 50.into()).await??;

        let result = shared_account::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(0.into()));

        let result = shared_account::withdraw(&runtime, bob, &account_id, 1.into()).await?;
        assert_eq!(result, Err(Error::new("insufficient balance")));

        let result = shared_account::withdraw(&runtime, claire, &account_id, 1.into()).await?;
        assert_eq!(result, Err(Error::new("unauthorized")));

        let result = shared_account::tenants(&runtime, &account_id).await?;
        assert_eq!(
            result,
            Some(vec![alice.to_string(), bob.to_string(), dara.to_string()])
        );

        Ok(())
    }
}
```

## Dependencies
Adding the `token-test` dependency to the `token/test/Cargo.toml` file causes `cargo test` or `cargo build` commands in `shared-account` to trigger builds in `token` so the compiled wasm file is available for use in this context.

```toml
[package]
name = "shared-account-test"
version = "0.1.0"
edition = "2024"

[dependencies]
testlib = { workspace = true }
stdlib = { workspace = true }
tokio = { workspace = true }

token-test = { path = "../../token/test" }
```