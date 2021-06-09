use anyhow::Result;
use ethers_core::rand::thread_rng;
use ethers_signers::{LocalWallet, Signer};


pub async fn signs() -> Result<()> {
    //let wallet = LocalWallet::new(&mut rand::thread_rng()); remove rand since it's already declared 
    let wallet = LocalWallet::new(&mut thread_rng());
    
    let message = "Some data";

    // sign a message
    let signature = wallet.sign_message(message).await?;
    println!("Produced signature {}", signature);

    // verify the signature
    signature.verify(message, wallet.address()).unwrap();

    println!("Verified signature produced by {:?}!", wallet.address());

    Ok(())
}
