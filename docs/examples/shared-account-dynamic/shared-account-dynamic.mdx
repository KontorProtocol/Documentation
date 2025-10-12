---
sidebar_position: 4
title: 4. Shared Account (Dynamic)
---

# Shared Account Dynamic

This example uses the `interface!` macro and `ContractAddress` type to implement dynamic cross-contract calls.

## WIT Interface
Import type `contract-address`

and exports:
- `open`: Creates an account for a specificied token and tenants
- `deposit`/`withdraw`: Modifies the account balance using the specified token contract
- `balance`/`tenants`: Queries the account balance or tenant list

```wit
package kontor:contract;

world contract {
    include kontor:built-in/built-in;
    use kontor:built-in/context.{view-context, proc-context, signer};
    use kontor:built-in/error.{error};
    use kontor:built-in/numbers.{integer, decimal};
    use kontor:built-in/foreign.{contract-address};

    export init: func(ctx: borrow<proc-context>);

    export open: func(ctx: borrow<proc-context>, token: contract-address, n: integer, other-tenants: list<string>) -> result<string, error>;

    export deposit: func(ctx: borrow<proc-context>, token: contract-address, account-id: string, n: integer) -> result<_, error>;

    export withdraw: func(ctx: borrow<proc-context>, token: contract-address, account-id: string, n: integer) -> result<_, error>;

    export balance: func(ctx: borrow<view-context>, account-id: string) -> option<integer>;

    export tenants: func(ctx: borrow<view-context>, account-id: string) -> option<list<string>>;
}
```

## Rust Implementation
Compared to the `shared-account` contract, this implementation uses the `interface!` macro to generate a dynamic token interface instead of a static one pinned to a specific contract. Functions (`open`, `deposit`, `withdraw`) take a `token: ContractAddress` argument to specify the token contract. The `Account` struct includes a `token` property to ensure the correct contract is used during authorization. Other functionality remains the same:

```rust
use stdlib::*;

contract!(name = "shared-account-dynamic");

interface!(name = "token-interface", path = "../token/contract/wit");

#[derive(Clone, Storage)]
struct Account {
    pub other_tenants: Map<String, bool>,
    pub token: ContractAddress,
    pub balance: Integer,
    pub owner: String,
}

#[derive(Clone, Default, StorageRoot)]
struct SharedAccountStorage {
    pub accounts: Map<String, Account>,
}

fn authorized(ctx: &ProcContext, token: &ContractAddress, account: &AccountWrapper) -> bool {
    (account.owner(ctx) == ctx.signer().to_string()
        || account
            .other_tenants()
            .get(ctx, ctx.signer().to_string())
            .is_some_and(|b| b))
        && token == &account.token(ctx)
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

impl Guest for SharedAccountDynamic {
    fn init(ctx: &ProcContext) {
        SharedAccountStorage::default().init(ctx);
    }

    fn open(
        ctx: &ProcContext,
        token: ContractAddress,
        n: Integer,
        other_tenants: Vec<String>,
    ) -> Result<String, Error> {
        let balance = token_interface::balance(&token, &ctx.signer().to_string())
            .ok_or(insufficient_balance_error())?;
        if balance < n {
            return Err(insufficient_balance_error());
        }
        let account_id = crypto::generate_id();
        storage(ctx).accounts().set(
            ctx,
            account_id.clone(),
            Account {
                token: token.clone(),
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
        token_interface::transfer(&token, ctx.signer(), &ctx.contract_signer().to_string(), n)?;
        Ok(account_id)
    }

    fn deposit(
        ctx: &ProcContext,
        token: ContractAddress,
        account_id: String,
        n: Integer,
    ) -> Result<(), Error> {
        let balance = token_interface::balance(&token, &ctx.signer().to_string())
            .ok_or(insufficient_balance_error())?;
        if balance < n {
            return Err(insufficient_balance_error());
        }
        let account = storage(ctx)
            .accounts()
            .get(ctx, account_id)
            .ok_or(unknown_error())?;
        if !authorized(ctx, &token, &account) {
            return Err(unauthorized_error());
        }
        account.set_balance(ctx, account.balance(ctx) + n);
        token_interface::transfer(&token, ctx.signer(), &ctx.contract_signer().to_string(), n)
    }

    fn withdraw(
        ctx: &ProcContext,
        token: ContractAddress,
        account_id: String,
        n: Integer,
    ) -> Result<(), Error> {
        let account = storage(ctx)
            .accounts()
            .get(ctx, account_id)
            .ok_or(unknown_error())?;
        if !authorized(ctx, &token, &account) {
            return Err(unauthorized_error());
        }
        let balance = account.balance(ctx);
        if balance < n {
            return Err(insufficient_balance_error());
        }
        account.set_balance(ctx, balance - n);
        token_interface::transfer(&token, ctx.contract_signer(), &ctx.signer().to_string(), n)
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
This test mirrors the `shared-account` test but requires a `token` contract address for dynamic calls.

```rust
#[cfg(test)]
mod tests {
    use testlib::*;

    import!(
        name = "shared-account-dynamic",
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
                    ("shared-account-dynamic", &contract_bytes().await?),
                    ("token", &dep_contract_bytes("token").await?),
                    ("other-token", &dep_contract_bytes("token").await?),
                ])
                .build(),
        )
        .await?;

        let alice = "alice";
        let bob = "bob";
        let claire = "claire";
        let dara = "dara";

        token::mint(&runtime, alice, 100.into()).await?;

        let token_address = ContractAddress {
            name: "token".to_string(),
            height: 0,
            tx_index: 0,
        };

        let account_id = shared_account_dynamic::open(
            &runtime,
            alice,
            token_address.clone(),
            50.into(),
            vec![bob, dara],
        )
        .await??;

        let result = shared_account_dynamic::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(50.into()));

        let result = shared_account_dynamic::withdraw(
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

        shared_account_dynamic::deposit(
            &runtime,
            alice,
            token_address.clone(),
            &account_id,
            25.into(),
        )
        .await??;

        let result = shared_account_dynamic::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(75.into()));

        shared_account_dynamic::withdraw(
            &runtime,
            bob,
            token_address.clone(),
            &account_id,
            25.into(),
        )
        .await??;

        let result = shared_account_dynamic::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(50.into()));

        shared_account_dynamic::withdraw(
            &runtime,
            alice,
            token_address.clone(),
            &account_id,
            50.into(),
        )
        .await??;

        let result = shared_account_dynamic::balance(&runtime, &account_id).await?;
        assert_eq!(result, Some(0.into()));

        let result = shared_account_dynamic::withdraw(
            &runtime,
            bob,
            token_address.clone(),
            &account_id,
            1.into(),
        )
        .await?;
        assert_eq!(result, Err(Error::new("insufficient balance")));

        let result = shared_account_dynamic::withdraw(
            &runtime,
            claire,
            token_address.clone(),
            &account_id,
            1.into(),
        )
        .await?;
        assert_eq!(result, Err(Error::new("unauthorized")));

        let result = shared_account_dynamic::tenants(&runtime, &account_id).await?;
        assert_eq!(
            result,
            Some(vec![alice.to_string(), bob.to_string(), dara.to_string()])
        );

        Ok(())
    }
}
```