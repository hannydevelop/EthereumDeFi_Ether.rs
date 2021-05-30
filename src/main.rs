use anyhow::Result;
use ethers::{prelude::*};
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<()> {

    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let accounts = provider.get_accounts().await?;
    let from = accounts[0];
    let to = accounts[1];

    let tx = TransactionRequest::new().to(to).value(1000).from(from);

    let balance_before = provider.get_balance(from, None).await?;
    let tx = provider.send_transaction(tx).await?;

  
    println!("TX Hash: {:?}", tx);

    let nonce1 = provider
        .get_transaction_count(from, Some(BlockNumber::Latest.into()))
        .await?;

    let nonce2 = provider
        .get_transaction_count(from, Some(BlockNumber::Number(0.into()).into()))
        .await?;

        assert!(nonce2 < nonce1);


    let balance_after = provider.get_balance(from, None).await?;
    assert!(balance_after < balance_before);

    println!("Balance before {}", balance_before);
    println!("Balance after {}", balance_after);

    Ok(())
}