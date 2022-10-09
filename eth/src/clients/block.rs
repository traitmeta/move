use web3::types::{BlockNumber, BlockId};

pub async fn getblock() -> web3::Result<()> {
    let transport = web3::transports::Http::new("https://mainnet.infura.io/v3/xxx")?;
    let web3 = web3::Web3::new(transport);

    let height = web3.eth().block_number().await?;
    println!("{:?}", height);

    let block_info = web3.eth().block(BlockId::Number(BlockNumber::Number(height))).await?;
    if let Some(info) = block_info{
        println!("{:?}", info.transactions);
    }

    Ok(())
}
