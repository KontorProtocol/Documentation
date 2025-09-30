---
sidebar_position: 6
title: 6. Automated Market-Maker (AMM)
---

# AMM
This example builds on top of the Token contract and demonstrates swaps and liqudity provision on an AMM. It demonstrates transfers of balances using the `token-dyn` interface for cross-contract calls to the Token contract, and complex integer arithmetic using the 256-bit `Integer` built-in type.

## WIT Interface
Imports types:
- `signer`: Transaction sender (identity)
- `error`: Failure handling
- `integer`: Amounts

and exports functions:
- `init`: Initializes the contract
- `create`: Create a liqudity pool for a new token pair
- `fee`: Get the fee (in bases points of input amount) for trading in a pair
- `balance`: Queries a liqudity provider (LP) share balance
- `token-balance`: Queries the balance of tokens in a liqudity pool
- `deposit`: Deposit both tokens of a pair and receive LP shares
- `withdraw`: Burn LP shares and withdraw the tokens they represent
- `swap`: Swap an amount of one token of a pair for the other token
- `quote-deposit`: Receive a quote of cost of shares received when depositing
- `quote-withdraw`: Receive a quote of tokens received for shares when withdrawing
- `quote-swap`: Receive a quote of out-tokens received when swapping

```wit
package kontor:contract;

world contract {
    include kontor:built-in/built-in;
    use kontor:built-in/context.{view-context, proc-context, signer};
    use kontor:built-in/error.{error};
    use kontor:built-in/foreign.{contract-address};
    use kontor:built-in/numbers.{integer, decimal};

	record token-pair {
		a: contract-address,
		b: contract-address,
	}

	record deposit-result {
		lp-shares: integer,
		deposit-a: integer,
		deposit-b: integer,
	}

	record withdraw-result {
		amount-a: integer,
		amount-b: integer,
	}

    export init: func(ctx: borrow<proc-context>);

    export create: func(ctx: borrow<proc-context>, pair: token-pair, amount-a: integer, amount-b: integer, fee-bps: integer) -> result<integer, error>;

	export fee: func(ctx: borrow<view-context>, pair: token-pair) -> result<integer, error>;

    export balance: func(ctx: borrow<view-context>, pair: token-pair, acc: string) -> option<integer>;
    export token-balance: func(ctx: borrow<view-context>, pair: token-pair, token: contract-address) -> result<integer, error>;
    export quote-deposit: func(ctx: borrow<view-context>, pair: token-pair, amount-a: integer, amount-b: integer) -> result<deposit-result, error>;
    export deposit: func(ctx: borrow<proc-context>, pair: token-pair, amount-a: integer, amount-b: integer) -> result<deposit-result, error>;
    export quote-withdraw: func(ctx: borrow<view-context>, pair: token-pair, shares: integer) -> result<withdraw-result, error>;
    export withdraw: func(ctx: borrow<proc-context>, pair: token-pair, shares: integer) -> result<withdraw-result, error>;


    export swap: func(ctx: borrow<proc-context>, pair: token-pair, token-in: contract-address, amount-in: integer, min-out: integer) -> result<integer, error>;
    export quote-swap: func(ctx: borrow<view-context>, pair: token-pair, token-in: contract-address, amount-in: integer) -> result<integer, error>;
}
```

## Rust Implementation
- `AMMStorage` represents the contract’s persistent storage. The `StorageRoot` derive macro enables storage capabilities for the type and marks it as the "root" storage type. The root storage is accesible via a generated function: `storage`, which provides an ORM-like interface to contract functions. Every contract with storage must have exactly one type that derives `StorageRoot`.
- `Map` is a storage-enabled mapping type that can hold values of any type that also derive `Storage`. Holds mappings from token pairs to `Pool`.
- `Pool` contains the information about the state of the pool for a token pair.
- The `ctx` parameter must be passed to every operation that performs a database call. These functions take an `impl ReadContext` or an `impl WriteContext`, depending on their behavior. `ProcContext` implements both, while `ViewContext` implements only `ReadContext`. This ensures state mutations occur only as the result of a transaction while allowing the same functions and methods to be used across procedures and views.

```rust
// TODO
```

## Testing
Similar to the tests for other contracts, this one uses the `import!` macro to generate an interface for calling the contract. It also includes assertions for error handling. For all calls, the first `?` operator checks whether the runtime threw an error during execution. For contract functions that explicitly return a `Result`, the result can be "unwrapped" with an additional `?` operator or left omitted to make an assertion on an error.

In addition to importing the `amm` contract, three tokens are instantiated by the `token` contract, and we import the `token-dyn` interface for dynamic cross-contract calls.

When working with numbers in Sigil, either the `Integer` or `Decimal` types should be used. `From` instances have been implemented for many of the primitive types which is why there are many `<num>.into()`s in the test. One can also write: `Integer::from(100)`, or `Decimal::from("1.5")`, or even `let x: Decimal = 1.5.into()`, etc.

```rust
#[tokio::test]
async fn test_amm_swaps() -> Result<()> {
    let runtime = Runtime::new(RuntimeConfig::default()).await?;

    let token_a = ContractAddress {
        name: "token_a".to_string(),
        height: 0,
        tx_index: 0,
    };

    let token_b = ContractAddress {
        name: "token_b".to_string(),
        height: 0,
        tx_index: 0,
    };

    let admin = "test_admin";
    let minter = "test_minter";
    token_a::mint(&runtime, minter, 1000.into()).await?;
    token_b::mint(&runtime, minter, 1000.into()).await?;

    token_a::transfer(&runtime, minter, admin, 100.into()).await??;
    token_b::transfer(&runtime, minter, admin, 500.into()).await??;

    let pair = amm::TokenPair {
        a: token_a.clone(),
        b: token_b.clone(),
    };
    let res = amm::create(
        &runtime,
        admin,
        pair.clone(),
        100.into(),
        500.into(),
        0.into(),
    )
    .await?;
    assert_eq!(res, Ok(223.into()));

    let bal_a = amm::token_balance(&runtime, pair.clone(), token_a.clone()).await?;
    assert_eq!(bal_a, Ok(100.into()));
    let bal_b = amm::token_balance(&runtime, pair.clone(), token_b.clone()).await?;
    assert_eq!(bal_b, Ok(500.into()));
    let k1 = bal_a.unwrap() * bal_b.unwrap();

    let res = amm::quote_swap(&runtime, pair.clone(), token_a.clone(), 10.into()).await?;
    assert_eq!(res, Ok(45.into()));

    let res = amm::quote_swap(&runtime, pair.clone(), token_a.clone(), 100.into()).await?;
    assert_eq!(res, Ok(250.into()));

    let res = amm::quote_swap(&runtime, pair.clone(), token_a.clone(), 1000.into()).await?;
    assert_eq!(res, Ok(454.into()));

    let res = amm::swap(
        &runtime,
        minter,
        pair.clone(),
        token_a.clone(),
        10.into(),
        46.into(),
    )
    .await?;
    assert!(res.is_err()); // below minimum

    let res = amm::swap(
        &runtime,
        minter,
        pair.clone(),
        token_a.clone(),
        10.into(),
        45.into(),
    )
    .await?;
    assert_eq!(res, Ok(45.into()));

    let bal_a = amm::token_balance(&runtime, pair.clone(), token_a.clone()).await?;
    let bal_b = amm::token_balance(&runtime, pair.clone(), token_b.clone()).await?;
    let k2 = bal_a.unwrap() * bal_b.unwrap();
    assert!(k2 >= k1);

    let res = amm::quote_swap(&runtime, pair.clone(), token_b.clone(), 45.into()).await?;
    assert_eq!(res, Ok(9.into()));
    let res = amm::swap(
        &runtime,
        minter,
        pair.clone(),
        token_b.clone(),
        45.into(),
        0.into(),
    )
    .await?;
    assert_eq!(res, Ok(9.into()));

    let bal_a = amm::token_balance(&runtime, pair.clone(), token_a.clone()).await?;
    let bal_b = amm::token_balance(&runtime, pair.clone(), token_b.clone()).await?;
    let k3 = bal_a.unwrap() * bal_b.unwrap();
    assert!(k3 >= k2);

    Ok(())
}
```
