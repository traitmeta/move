use std::str::FromStr;
use sui_sdk::types::base_types::{SuiAddress, ObjectID};
use sui_sdk::SuiClient;

pub async fn default_get_objects_owned_by_address() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new_rpc_client("https://gateway.devnet.sui.io:443",None).await?;
    let address = SuiAddress::from_str("0x1a6254d89ee1698ed62c03481d28eee88c685b94")?;
    let objects = sui.read_api().get_objects_owned_by_address(address).await?;
    println!("{:?}", objects);
    Ok(())
}



pub async fn default_get_object_by_id() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new_rpc_client("https://gateway.devnet.sui.io:443",None).await?;
    let address = ObjectID::from_str("0xea0a2a0a8abb0b9dd54bef2baf11cf9dd83df845")?;
    let objects = sui.read_api().get_parsed_object(address).await?;
    println!("{:?}", objects);
    Ok(())
}

pub async fn default_get_raw_object_by_id() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new_rpc_client("https://gateway.devnet.sui.io:443",None).await?;
    let address = ObjectID::from_str("0xea0a2a0a8abb0b9dd54bef2baf11cf9dd83df845")?;
    let objects = sui.read_api().get_object(address).await?;
    println!("{:?}", objects);
    Ok(())
}
