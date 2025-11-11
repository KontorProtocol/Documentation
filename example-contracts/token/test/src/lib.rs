#[cfg(test)]
mod tests {
    use testlib::*;

    interface!(name = "token", path = "contract/wit");

    async fn run_test_contract(runtime: &mut Runtime) -> Result<()> {
        let minter = runtime.identity().await?;
        let holder = runtime.identity().await?;
        let token = runtime.publish(&minter, "token").await?;

        token::mint(runtime, &token, &minter, 900.into()).await??;
        token::mint(runtime, &token, &minter, 100.into()).await??;

        let result = token::balance(runtime, &token, &minter).await?;
        assert_eq!(result, Some(1000.into()));

        let result = token::transfer(runtime, &token, &holder, &minter, 123.into()).await?;
        assert_eq!(
            result,
            Err(Error::Message("insufficient funds".to_string()))
        );

        token::transfer(runtime, &token, &minter, &holder, 40.into()).await??;
        token::transfer(runtime, &token, &minter, &holder, 2.into()).await??;

        let result = token::balance(runtime, &token, &holder).await?;
        assert_eq!(result, Some(42.into()));

        let result = token::balance(runtime, &token, &minter).await?;
        assert_eq!(result, Some(958.into()));

        let result = token::balance(runtime, &token, "foo").await?;
        assert_eq!(result, None);

        let balances = token::balances(runtime, &token).await?;
        assert_eq!(balances.len(), 2);
        let total = balances
            .iter()
            .fold(Integer::from(0), |acc, x| acc + x.value);
        assert_eq!(total, token::total_supply(runtime, &token).await?);

        Ok(())
    }

    #[testlib::test]
    async fn test_contract() -> Result<()> {
        run_test_contract(runtime).await
    }

    #[testlib::test(mode = "regtest")]
    async fn test_contract_regtest() -> Result<()> {
        run_test_contract(runtime).await
    }
}
