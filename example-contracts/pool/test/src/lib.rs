#[cfg(test)]
mod tests {
    use testlib::*;

    interface!(name = "token", path = "../token/contract/wit");

    interface!(name = "pool");

    async fn run_test_contract(runtime: &mut Runtime) -> Result<()> {
        let admin = runtime.identity().await?;
        let minter = runtime.identity().await?;

        let token_a = runtime.publish_as(&admin, "token", "token-a").await?;
        let token_b = runtime.publish_as(&admin, "token", "token-b").await?;
        let pool = runtime.publish(&admin, "pool").await?;

        token::mint(runtime, &token_a, &minter, 1000.into()).await??;
        token::mint(runtime, &token_b, &minter, 1000.into()).await??;

        token::transfer(runtime, &token_a, &minter, &admin, 100.into()).await??;
        token::transfer(runtime, &token_b, &minter, &admin, 500.into()).await??;

        let res = pool::re_init(
            runtime,
            &pool,
            &admin,
            token_a.clone(),
            100.into(),
            token_b.clone(),
            500.into(),
            0.into(),
        )
        .await?;
        assert_eq!(res, Ok(223.into()));

        let bal_a = pool::token_balance(runtime, &pool, token_a.clone()).await?;
        assert_eq!(bal_a, Ok(100.into()));
        let bal_b = pool::token_balance(runtime, &pool, token_b.clone()).await?;
        assert_eq!(bal_b, Ok(500.into()));
        let k1 = bal_a.unwrap() * bal_b.unwrap();

        let res = pool::quote_swap(runtime, &pool, token_a.clone(), 10.into()).await?;
        assert_eq!(res, Ok(45.into()));

        let res = pool::quote_swap(runtime, &pool, token_a.clone(), 100.into()).await?;
        assert_eq!(res, Ok(250.into()));

        let res = pool::quote_swap(runtime, &pool, token_a.clone(), 1000.into()).await?;
        assert_eq!(res, Ok(454.into()));

        let res = pool::swap(
            runtime,
            &pool,
            &minter,
            token_a.clone(),
            10.into(),
            46.into(),
        )
        .await?;
        assert!(res.is_err()); // below minimum

        let res = pool::swap(
            runtime,
            &pool,
            &minter,
            token_a.clone(),
            10.into(),
            45.into(),
        )
        .await?;
        assert_eq!(res, Ok(45.into()));

        let bal_a = pool::token_balance(runtime, &pool, token_a.clone()).await?;
        let bal_b = pool::token_balance(runtime, &pool, token_b.clone()).await?;
        let k2 = bal_a.unwrap() * bal_b.unwrap();
        assert!(k2 >= k1);

        let res = pool::quote_swap(runtime, &pool, token_b.clone(), 45.into()).await?;
        assert_eq!(res, Ok(9.into()));
        let res = pool::swap(
            runtime,
            &pool,
            &minter,
            token_b.clone(),
            45.into(),
            0.into(),
        )
        .await?;
        assert_eq!(res, Ok(9.into()));

        let bal_a = pool::token_balance(runtime, &pool, token_a.clone()).await?;
        let bal_b = pool::token_balance(runtime, &pool, token_b.clone()).await?;
        let k3 = bal_a.unwrap() * bal_b.unwrap();
        assert!(k3 >= k2);

        // use token interface to transfer shares
        let res = token::balance(runtime, &pool, &admin).await?;
        assert_eq!(res, Some(223.into()));
        let res = token::balance(runtime, &pool, &minter).await?;
        assert_eq!(res, None);

        token::transfer(runtime, &pool, &admin, &minter, 23.into()).await??;

        let res = token::balance(runtime, &pool, &admin).await?;
        assert_eq!(res, Some(200.into()));
        let res = token::balance(runtime, &pool, &minter).await?;
        assert_eq!(res, Some(23.into()));

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
