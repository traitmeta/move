use std::str::FromStr;
use sui_sdk::{
    crypto::KeystoreType,
    types::{
        base_types::{ObjectID, SuiAddress},
        crypto::Signature,
        messages::{Transaction, TransactionData},
    },
    SuiClient,
};

pub async fn default_conduct_transaction(
    sui: &SuiClient,
    from: &str,
    to: &str,
    gas: &str,
) -> Result<(), anyhow::Error> {
    // Create a sui transfer transaction
    let transfer_tx = build_transaction_sui(sui, from, to, gas).await?;

    // Sign the transaction
    let signature = sign_transaction(from, &transfer_tx);

    // Execute the transaction
    let transaction_response = sui
        .quorum_driver()
        .execute_transaction(Transaction::new(transfer_tx, signature))
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}

// why build transaction need client?
pub async fn build_transaction_sui(
    sui: &SuiClient,
    from: &str,
    to: &str,
    gas: &str,
) -> Result<TransactionData, anyhow::Error> {
    let from = SuiAddress::from_str(from)?;
    let gas_object_id = ObjectID::from_str(gas)?;
    let recipient = SuiAddress::from_str(to)?;

    // Create a sui transfer transaction
    let transfer_tx = sui
        .transaction_builder()
        .transfer_sui(from, gas_object_id, 1000, recipient, Some(1000))
        .await?;

    Ok(transfer_tx)
}

// why build transaction need client?
pub async fn sign_transaction(
    from: &str,
    data: &TransactionData,
) -> Result<Signature, anyhow::Error> {
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };
    let from = SuiAddress::from_str(from)?;

    // Get signer from keystore
    let keystore = KeystoreType::File(keystore_path).init()?;
    let signer = keystore.signer(from);

    // Sign the transaction
    let signature = Signature::new(data, &signer);

    Ok(signature)
}