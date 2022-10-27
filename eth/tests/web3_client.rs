use core::clients::block;


// cargo test --package core --test web3_client -- test_getblock --exact --nocapture 
#[tokio::test]
async fn test_getblock() {
    block::getblock().await;
}
