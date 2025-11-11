#[cfg(test)]
mod tests {
    use testlib::*;

    interface!(name = "amm");

    interface!(name = "token", path = "../token/contract/wit");

    async fn run_test_contract(runtime: &mut Runtime) -> Result<()> {
        let admin = runtime.identity().await?;
        let minter = runtime.identity().await?;

        let amm = runtime.publish(&admin, "amm").await?;
        let token_a = runtime.publish_as(&admin, "token", "token-a").await?;
        let token_b = runtime.publish_as(&admin, "token", "token-b").await?;

        token::mint(runtime, &token_a, &minter, 1000.into()).await??;
        token::mint(runtime, &token_b, &minter, 1000.into()).await??;

        token::transfer(runtime, &token_a, &minter, &admin, 100.into()).await??;
        token::transfer(runtime, &token_b, &minter, &admin, 500.into()).await??;

        let pair = amm::TokenPair {
            a: token_a.clone(),
            b: token_b.clone(),
        };
        let res = amm::create(
            runtime,
            &amm,
            &admin,
            pair.clone(),
            100.into(),
            500.into(),
            0.into(),
        )
        .await?;
        assert_eq!(res, Ok(223.into()));

        let bal_a = amm::token_balance(runtime, &amm, pair.clone(), token_a.clone()).await?;
        assert_eq!(bal_a, Ok(100.into()));
        let bal_b = amm::token_balance(runtime, &amm, pair.clone(), token_b.clone()).await?;
        assert_eq!(bal_b, Ok(500.into()));
        let k1 = bal_a.unwrap() * bal_b.unwrap();

        let res = amm::quote_swap(runtime, &amm, pair.clone(), token_a.clone(), 10.into()).await?;
        assert_eq!(res, Ok(45.into()));

        let res = amm::quote_swap(runtime, &amm, pair.clone(), token_a.clone(), 100.into()).await?;
        assert_eq!(res, Ok(250.into()));

        let res =
            amm::quote_swap(runtime, &amm, pair.clone(), token_a.clone(), 1000.into()).await?;
        assert_eq!(res, Ok(454.into()));

        let res = amm::swap(
            runtime,
            &amm,
            &minter,
            pair.clone(),
            token_a.clone(),
            10.into(),
            46.into(),
        )
        .await?;
        assert!(res.is_err()); // below minimum

        let res = amm::swap(
            runtime,
            &amm,
            &minter,
            pair.clone(),
            token_a.clone(),
            10.into(),
            45.into(),
        )
        .await?;
        assert_eq!(res, Ok(45.into()));

        let bal_a = amm::token_balance(runtime, &amm, pair.clone(), token_a.clone()).await?;
        let bal_b = amm::token_balance(runtime, &amm, pair.clone(), token_b.clone()).await?;
        let k2 = bal_a.unwrap() * bal_b.unwrap();
        assert!(k2 >= k1);

        let res = amm::quote_swap(runtime, &amm, pair.clone(), token_b.clone(), 45.into()).await?;
        assert_eq!(res, Ok(9.into()));
        let res = amm::swap(
            runtime,
            &amm,
            &minter,
            pair.clone(),
            token_b.clone(),
            45.into(),
            0.into(),
        )
        .await?;
        assert_eq!(res, Ok(9.into()));

        let bal_a = amm::token_balance(runtime, &amm, pair.clone(), token_a.clone()).await?;
        let bal_b = amm::token_balance(runtime, &amm, pair.clone(), token_b.clone()).await?;
        let k3 = bal_a.unwrap() * bal_b.unwrap();
        assert!(k3 >= k2);

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
