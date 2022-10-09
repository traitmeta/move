use std::{fs::File, io::BufReader};
use sui_sdk::{
    crypto::{KeystoreType, SuiKeystore},
    types::crypto::SignatureScheme,
};

pub mod address;
pub mod transactions;
pub mod ws;
pub mod block;

pub fn local_keystore() -> Result<SuiKeystore, anyhow::Error> {
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    // let reader = BufReader::new(File::open(keystore_path)?);
    // let kp_strings: Vec<String> = serde_json::from_reader(reader)?;
    // println!("{:?}", kp_strings);
    let keystore = KeystoreType::File(keystore_path).init()?;
    let addresses = keystore.addresses();
    println!("{:?}", addresses);
    Ok(keystore)
    // Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_keystore_test() {
        let _result = local_keystore().unwrap();
        // println!("{:#?}",result)
    }
}
