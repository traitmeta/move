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

pub async fn default_conduct_transaction(sui: &SuiClient) -> Result<(), anyhow::Error> {
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v
            .join(".sui")
            .join("sui_config")
            .join("5ac37128a02c0e83360e315c01a50103b3bd212c26f96818a0748e27a4159de0.key"),
        None => panic!("Cannot obtain home directory path"),
    };

    let my_address = SuiAddress::from_str("0x5ac37128a02c0e83360e315c01a50103b3bd212c")?;
    let gas_object_id = ObjectID::from_str("0x09aa590503d408433e433b15a70133d4e3e9fa2e")?;
    let recipient = SuiAddress::from_str("0x20ef8251e36d6247b372975c374914f2e2b16120")?;

    //     let object = sui.read_api().get_parsed_object(gas_object_id).await?;
    //     let object = object.into_object()?.reference.to_object_ref();
    //     let transfer_tx = TransactionData::new_transfer_sui(
    //         recipient, my_address, Some(1000), object, 1000,
    //    );
    // Create a sui transfer transaction
    let transfer_tx = sui
        .transaction_builder()
        .transfer_sui(my_address, gas_object_id, 1000, recipient, Some(1000))
        .await?;

    // Get signer from keystore
    let keystore = KeystoreType::File(keystore_path).init()?;
    let addresses = keystore.addresses();
    println!("{:?}", addresses);
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
