use clients::address;
use clients::transactions;
use sui_sdk::SuiClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new_rpc_client("https://gateway.devnet.sui.io:443", None).await?;
    // address::default_get_objects_owned_by_address().await
    // transactions::default_conduct_transaction().await
    let query_obj_id = "0xea0a2a0a8abb0b9dd54bef2baf11cf9dd83df845";
    address::default_get_object_by_id(&sui, query_obj_id).await;
    address::default_get_raw_object_by_id(&sui, query_obj_id).await
}
