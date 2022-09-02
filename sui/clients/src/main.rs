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
    transactions::default_conduct_transaction(&sui).await
    // let query_obj_id = "0x258eceba1168d876a55267b79cb36165f6f90c55";
    // address::default_get_object_by_id(&sui, query_obj_id).await;
    // address::default_get_raw_object_by_id(&sui, query_obj_id).await
}
