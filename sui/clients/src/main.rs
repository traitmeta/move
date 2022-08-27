use clients::address;
use clients::transactions;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    // address::default_get_objects_owned_by_address().await
    // transactions::default_conduct_transaction().await

    address::default_get_raw_object_by_id().await
} 
