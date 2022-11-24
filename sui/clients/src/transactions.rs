use futures::TryFutureExt;
use serde_json::{json, Number, Value as JsonValue};
use std::str::FromStr;
use sui_sdk::{
    crypto::KeystoreType,
    json::SuiJsonValue,
    rpc_types::SuiTypeTag,
    types::{
        base_types::{ObjectID, SuiAddress},
        crypto::Signature,
        gas_coin::GasCoin,
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
    let signature = sign_transaction(from, &transfer_tx).unwrap();

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
pub fn sign_transaction(from: &str, data: &TransactionData) -> Result<Signature, anyhow::Error> {
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

// why build transaction need client?
pub async fn build_move_call(
    sui: &SuiClient,
    singer: &str,
    pacakge_id: &str,
    gas: &str,
    pay_token: &str,
) -> Result<TransactionData, anyhow::Error> {
    let singer = SuiAddress::from_str(singer)?;
    let package_obj_id = ObjectID::from_str(pacakge_id)?;
    let gas = ObjectID::from_str(gas)?;
    let pay_token_id = ObjectID::from_str(pay_token)?;

    // Create a sui transfer transaction
    let transfer_tx = sui
        .transaction_builder()
        .move_call(
            singer,
            package_obj_id,
            "package",
            "create_fair",
            // TODO change "0x2::sui::SUI" to SuiTypeTag
            vec![],
            vec![
                SuiJsonValue::from_object_id(pay_token_id),
                SuiJsonValue::new(JsonValue::Number(Number::from(1000000)))?,
                SuiJsonValue::new(JsonValue::Number(Number::from(2)))?,
            ],
            Some(gas),
            1000,
        )
        .await?;

    Ok(transfer_tx)
}

#[cfg(test)]
mod tests {
    use sui_sdk::SuiClient;

    use crate::transactions::build_move_call;

    #[test]
    fn test_build_move_call() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let cli = rt.block_on(SuiClient::new_rpc_client(
            "https://fullnode.devnet.sui.io:443",
            None,
        )).unwrap();
        let res = rt
            .block_on(build_move_call(
                &cli,
                "0xd2a75c7f37f4d540c3b0313e8e8c97ad21b84f3a",
                "0xba8d7546e8b68f1ad56190d14eada311d320a9be",
                "0x13c9396fb50fc06141f7ce0c9ddd5ecd8fe41102",
                "0x3aab6a4a738b1efb019912556f2e153ed127c4f3",
            ))
            .unwrap();

        println!("{:?}", res);
    }
}
