use futures::StreamExt;
use sui_sdk::rpc_types::SuiEventFilter;
use sui_sdk::SuiClient;

pub async fn default_ws() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new_rpc_client("https://gateway.devnet.sui.io:443", Some("ws://127.0.0.1:9001")).await?;
    let mut subscribe_all = sui.event_api().subscribe_event(SuiEventFilter::All(vec![])).await?;
    loop {
        println!("{:?}", subscribe_all.next().await);
    }
}