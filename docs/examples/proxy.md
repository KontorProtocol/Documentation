---
sidebar_position: 5
---

# Proxy

This example demonstrates proxying (a common pattern used in contract upgrades), utilizing the `fallback` hook and introducing the `foreign::call` built-in and aliased imports.

## WIT Interface
- `fallback`: Handles WAVE-format expressions, forwarding them to the target contract.
- `get-contract-address`: Retrieves the target contract address.
- `set-contract-address`: Updates the target contract address.

```wit
package kontor:contract;

world contract {
    include kontor:built-in/built-in;
    use kontor:built-in/context.{fall-context, proc-context, view-context, signer};
    use kontor:built-in/foreign.{contract-address};

    export init: func(ctx: borrow<proc-context>);

    export fallback: func(ctx: borrow<fall-context>, expr: string) -> string;

    export get-contract-address: func(ctx: borrow<view-context>) -> option<contract-address>;

    export set-contract-address: func(ctx: borrow<proc-context>, contract-address: contract-address);
}
```

## Rust Implementation
- `ProxyStorage` struct stores the `ContractAddress` using the `StorageRoot` macro:
- `fallback`: Forwards WAVE-format expressions to the stored contract address via the `foreign::call` built-in, using `ctx.view_context()` for read-only access. The `foreign::call` built-in is what is used by the `import!` and `interface!` macros under the hood and it is *not* recommended to use it directly in most situations.
- `fallback` accepts a `FallContext` which is distinct from `ProcContext` and `ViewContext` in that it can be turned into either one (the `proc_context` method returns `Option<ProcContext>` because the call may not have been made through a Bitcoin transaction)
- `get-contract-address`: Retrieves the stored contract address.
- `set-contract-address`: Updates the stored contract address.

```rust
use stdlib::*;

contract!(name = "proxy");

#[derive(Clone, Default, StorageRoot)]
struct ProxyStorage {
    contract_address: Option<ContractAddress>,
}

impl Guest for Proxy {
    fn init(ctx: &ProcContext) {
        ProxyStorage::default().init(ctx);
    }

    fn fallback(ctx: &FallContext, expr: String) -> String {
        let _ctx = &ctx.view_context();
        let contract_address = storage(_ctx).contract_address(_ctx).unwrap();
        foreign::call(ctx.signer(), &contract_address, &expr)
    }

    fn get_contract_address(ctx: &ViewContext) -> Option<ContractAddress> {
        storage(ctx).contract_address(ctx)
    }

    fn set_contract_address(ctx: &ProcContext, contract_address: ContractAddress) {
        storage(ctx).set_contract_address(ctx, Some(contract_address));
    }
}
```

## Testing
This test closely resembles the `shared-account-dynamic` test but specifies the `proxy` contract's address in the `import!` of `shared-account-dynamic`, aliased as `shared_account_dynamic_proxied`. It also `import!`s the `proxy` contract to configure it to target the `shared-account-dynamic` contract.

```rust
#[cfg(test)]
mod tests {
    use testlib::*;

    import!(
        name = "proxy",
        height = 0,
        tx_index = 0,
        path = "contract/wit",
    );

    import!(
        mod_name = "shared_account_dynamic_proxied",
        name = "proxy",
        height = 0,
        tx_index = 0,
        path = "../shared-account-dynamic/contract/wit",
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
                    ("token", &dep_contract_bytes("token").await?),
                    (
                        "shared-account-dynamic",
                        &dep_contract_bytes("shared-account-dynamic").await?,
                    ),
                    ("proxy", &contract_bytes().await?),
                ])
                .build(),
        )
        .await?;

        let alice = "alice";
        let bob = "bob";
        let claire = "claire";
        let dara = "dara";

        let result = proxy::get_contract_address(&runtime).await?;
        assert_eq!(result, None);

        let address = ContractAddress {
            name: "shared-account-dynamic".to_string(),
            height: 0,
            tx_index: 0,
        };
        proxy::set_contract_address(&runtime, alice, address.clone()).await?;

        let result = proxy::get_contract_address(&runtime).await?;
        assert_eq!(result, Some(address));

        token::mint(&runtime, alice, 100.into()).await?;

        let token_address = ContractAddress {
            name: "token".to_string(),
            height: 0,
            tx_index: 0,
        };

        let account_id = shared_account_dynamic_proxied::open(
            &runtime,
            alice,
            token_address.clone(),
            50.into(),
            vec![bob, dara],
        )
        .await??;

        let result = shared_account_dynamic_proxied::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(50.into()));

        let result = shared_account_dynamic_proxied::withdraw(
            &runtime,
            alice,
            ContractAddress {
                name: "other-token".to_string(),
                height: 0,
                tx_index: 0,
            },
            &account_id,
            50.into(),
        )
        .await?;
        assert_eq!(result, Err(Error::new("unauthorized")));

        shared_account_dynamic_proxied::deposit(
            &runtime,
            alice,
            token_address.clone(),
            &account_id,
            25.into(),
        )
        .await??;

        let result = shared_account_dynamic_proxied::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(75.into()));

        shared_account_dynamic_proxied::withdraw(
            &runtime,
            bob,
            token_address.clone(),
            &account_id,
            25.into(),
        )
        .await??;

        let result = shared_account_dynamic_proxied::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(50.into()));

        shared_account_dynamic_proxied::withdraw(
            &runtime,
            alice,
            token_address.clone(),
            &account_id,
            50.into(),
        )
        .await??;

        let result = shared_account_dynamic_proxied::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(0.into()));

        let result = shared_account_dynamic_proxied::withdraw(
            &runtime,
            bob,
            token_address.clone(),
            &account_id,
            1.into(),
        )
        .await?;
        assert_eq!(result, Err(Error::new("insufficient balance")));

        let result = shared_account_dynamic_proxied::withdraw(
            &runtime,
            claire,
            token_address.clone(),
            &account_id,
            1.into(),
        )
        .await?;
        assert_eq!(result, Err(Error::new("unauthorized")));

        let result = shared_account_dynamic_proxied::tenants(&runtime, &account_id).await?;
        assert_eq!(
            result,
            Some(vec![alice.to_string(), bob.to_string(), dara.to_string()])
        );

        Ok(())
    }
}
```

## Dependencies
The `test/Cargo.toml` file specifies dependencies for the test environment, including `token-test` and `shared-account-dynamic-test` for integration with the `token` and `shared-account-dynamic` contracts.

```toml
[package]
name = "proxy-test"
version = "0.1.0"
edition = "2024"

[dependencies]
testlib = { workspace = true }
stdlib = { workspace = true }
tokio = { workspace = true }

token-test = { path = "../../token/test" }
shared-account-dynamic-test = { path = "../../shared-account-dynamic/test" }
```