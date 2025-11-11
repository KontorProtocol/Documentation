#[cfg(test)]
mod tests {
    use testlib::*;

    interface!(name = "hello-world");

    #[testlib::test]
    async fn test_contract() -> Result<()> {
        let alice = runtime.identity().await?;
        let hello_world = runtime.publish(&alice, "hello-world").await?;
        let result = hello_world::hello_world(runtime, &hello_world).await?;
        assert_eq!(result, "Hello, World!");
        Ok(())
    }
}
