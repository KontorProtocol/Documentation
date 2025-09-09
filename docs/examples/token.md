---
sidebar_position: 2
---

# Token

This example demonstrates persistent storage, user balances, transactions, and error handling for a fungible token on the blockchain.

## WIT Interface
The WIT file (`contract/wit/contract.wit`) includes imports for:
- `signer`: Transaction sender.
- `error`: Failure handling.
- `integer`: Amounts.

and exports:
- `init`: Initializes the contract.
- `mint`: Adds tokens to the signer’s balance.
- `transfer`: Moves tokens, potentially returning an error.
- `balance`: Queries a balance.

```wit
package kontor:contract;

world contract {
    include kontor:built-in/built-in;
    use kontor:built-in/context.{view-context, proc-context, signer};
    use kontor:built-in/error.{error};
    use kontor:built-in/numbers.{integer, decimal};

    export init: func(ctx: borrow<proc-context>);

    export mint: func(ctx: borrow<proc-context>, n: integer);
    export transfer: func(ctx: borrow<proc-context>, to: string, n: integer) -> result<_, error>;
    export balance: func(ctx: borrow<view-context>, acc: string) -> option<integer>;
}
```

## Rust Implementation
In `contract/src/lib.rs`:
- `TokenStorage` represents the contract’s persistent storage. The `StorageRoot` derive macro enables storage capabilities for the type and marks it as the "root" storage type, generating a `storage` function that provides ORM-like persistent storage access to contract functions. Every contract with storage must have exactly one type that derives `StorageRoot`.
- `Map` is a storage-enabled map type that can hold values of any type that also derives `Storage`. In this case, `Integer` is provided by `stdlib` and is storage-enabled.
- The `ctx` parameter is passed to every operation that performs a database call. These functions take an `impl ReadContext` or an `impl WriteContext`, depending on their behavior. `ProcContext` implements both, while `ViewContext` implements only `ReadContext`. This ensures state mutations occur only as the result of a transaction while allowing the same functions and methods to be used across procedures and views.

The implementation uses the following:

```rust
use stdlib::*;

contract!(name = "token");

#[derive(Clone, Default, StorageRoot)]
struct TokenStorage {
    pub ledger: Map<String, Integer>,
}

impl Guest for Token {
    fn init(ctx: &ProcContext) {
        TokenStorage::default().init(ctx);
    }

    fn mint(ctx: &ProcContext, n: Integer) {
        let to = ctx.signer().to_string();
        let ledger = storage(ctx).ledger();

        let balance = ledger.get(ctx, &to).unwrap_or_default();
        ledger.set(ctx, to, balance + n);
    }

    fn transfer(ctx: &ProcContext, to: String, n: Integer) -> Result<(), Error> {
        let from = ctx.signer().to_string();
        let ledger = storage(ctx).ledger();

        let from_balance = ledger.get(ctx, &from).unwrap_or_default();
        let to_balance = ledger.get(ctx, &to).unwrap_or_default();

        if from_balance < n {
            return Err(Error::new("insufficient funds"));
        }

        ledger.set(ctx, from, from_balance - n);
        ledger.set(ctx, to, to_balance + n);
        Ok(())
    }

    fn balance(ctx: &ViewContext, acc: String) -> Option<Integer> {
        storage(ctx).ledger().get(ctx, acc)
    }
}
```

## Testing
The test in `test/src/lib.rs`, similar to the `hello-world` test, uses the `import!` macro to generate a convenient interface for calling the contract. It also includes assertions for error handling. For all calls, the first `?` operator checks whether the runtime threw an error during execution. For contract functions that explicitly return a `Result`, the result can be checked with an additional `?` operator or left unwrapped to assert on the outcome.

When working with numbers in Sigil, either the `Integer` or `Decimal` types should be used. `From` instnaces have been implemented for many of the primitive types which is why there are many `<num>.into()`s in the test. One can also write: `Integer::from(100)`, or `Decimal::from("1.5")`, or even `let x: Decimal = 1.5.into()`, etc.

```rust
#[cfg(test)]
mod tests {
    use testlib::*;

    import!(
        name = "token",
        height = 0,
        tx_index = 0,
        path = "contract/wit",
        test = true,
    );

    #[tokio::test]
    async fn test_contract() -> Result<()> {
        let runtime = Runtime::new(
            RuntimeConfig::builder()
                .contracts(&[("token", &contract_bytes().await?)])
                .build(),
        )
        .await?;

        let minter = "test_minter";
        let holder = "test_holder";
        token::mint(&runtime, minter, 900.into()).await?;
        token::mint(&runtime, minter, 100.into()).await?;

        let result = token::balance(&runtime, minter).await?;
        assert_eq!(result, Some(1000.into()));

        let result = token::transfer(&runtime, holder, minter, 123.into()).await?;
        assert_eq!(
            result,
            Err(Error::Message("insufficient funds".to_string()))
        );

        token::transfer(&runtime, minter, holder, 40.into()).await??;
        token::transfer(&runtime, minter, holder, 2.into()).await??;

        let result = token::balance(&runtime, holder).await?;
        assert_eq!(result, Some(42.into()));

        let result = token::balance(&runtime, minter).await?;
        assert_eq!(result, Some(958.into()));

        let result = token::balance(&runtime, "foo").await?;
        assert_eq!(result, None);

        Ok(())
    }
}
```