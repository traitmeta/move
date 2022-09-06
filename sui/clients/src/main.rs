use clients::address;
use clients::transactions;
use sui_sdk::SuiClient;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_max_level(Level::TRACE)
        .finish()
        .try_init()
        .expect("setting default subscriber failed");

    let sui = SuiClient::new_rpc_client("https://gateway.devnet.sui.io:443", None).await?;
    // address::default_get_objects_owned_by_address().await
    let from = "0x054d99f30cdbaea6dd5981d4b8673f4c6fb3b9b0";
    let to = "0xe806e842ca27361f74983711bcb8d9e6612fa06d";
    let gas = "0x597f6133abacc63f255513ce189761d9e0f2fae2";
    transactions::default_conduct_transaction(&sui, form, to, gas).await
    // address::default_get_object_by_id(&sui, query_obj_id).await;
    // address::default_get_raw_object_by_id(&sui, query_obj_id).await
}
