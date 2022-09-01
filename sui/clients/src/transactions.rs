use std::str::FromStr;
use sui_sdk::{
    crypto::KeystoreType,
    types::{
        base_types::{ObjectID, SuiAddress},
        crypto::Signature,
        messages::Transaction,
    },
    SuiClient,
};

pub async fn default_conduct_transaction(sui: &SuiClient) -> Result<(), anyhow::Error> {
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let my_address = SuiAddress::from_str("0x0a9186f3be567385b45eb8c9a6bc99d68074b738")?;
    let gas_object_id = ObjectID::from_str("0xea0a2a0a8abb0b9dd54bef2baf11cf9dd83df845")?;
    let recipient = SuiAddress::from_str("0x20ef8251e36d6247b372975c374914f2e2b16120")?;

    // Create a sui transfer transaction
    let transfer_tx = sui
        .transaction_builder()
        .transfer_sui(my_address, gas_object_id, 1000, recipient, Some(1000))
        .await?;

    // Get signer from keystore
    let keystore = KeystoreType::File(keystore_path).init()?;
    let signer = keystore.signer(my_address);

    // Sign the transaction
    let signature = Signature::new(&transfer_tx, &signer);

    // Execute the transaction
    let transaction_response = sui
        .quorum_driver()
        .execute_transaction(Transaction::new(transfer_tx, signature))
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
