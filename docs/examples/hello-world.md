---
sidebar_position: 1
---

# Hello World

This example illustrates the fundamental structure of a simple contract, serving as an excellent starting point for creating your own.

## WIT Interface
The WIT file (`contract/wit/contract.wit`) defines the `contract` world:
- Includes `kontor:built-in` via the `wit/deps` symlink.
- Exports:
  - `init`: Runs on contract publication.
  - `hello-world`: Returns a string, using `view-context`.

```wit
package kontor:contract;

world contract {
    include kontor:built-in/built-in;
    use kontor:built-in/context.{view-context, proc-context};

    export init: func(ctx: borrow<proc-context>);

    export hello-world: func(ctx: borrow<view-context>) -> string;
}
```

## Rust Implementation
In `contract/src/lib.rs`, the `contract!` macro sets the contract name. The `HelloWorld` struct is automatically derived from it. The generated `Guest` trait is implemented with:
- `init`: Empty, as no contract storage is needed.
- `hello-world`: Returns a static string.

```rust
use stdlib::*;

contract!(name = "hello-world");

impl Guest for HelloWorld {
    fn init(_ctx: &ProcContext) {}

    fn hello_world(_ctx: &ViewContext) -> String {
        "Hello, World!".to_string()
    }
}
```

## Testing
The test in `test/src/lib.rs`:
- The `use testlib::*` import populates the namespace with trait implementations and types to make testing contracts as close to calling Rust functions as possible.
- `contract_bytes()` loads the bytes of the `hello-world` contract.
- `import!` generates a module named `hello_world` that exposes functions matching the contract's exports, providing a convenient interface for interacting with the contract.
  - `test = true` sets the macro to test mode. It can be used inside contract implementations to "import" and use other published contracts when this argument is omitted.
- Calls `hello_world`, and verifies the output.

```rust
#[cfg(test)]
mod tests {
    use testlib::*;

    import!(
        name = "hello-world",
        height = 0,
        tx_index = 0,
        path = "contract/wit",
        test = true,
    );

    #[tokio::test]
    async fn test_contract() -> Result<()> {
        let runtime = Runtime::new(
            RuntimeConfig::builder()
                .contracts(&[("hello-world", &contract_bytes().await?)])
                .build(),
        )
        .await
```