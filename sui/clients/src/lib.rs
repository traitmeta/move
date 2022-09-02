use sui_sdk::crypto::{SuiKeystore, KeystoreType};

pub mod address;
pub mod transactions;
pub mod ws;


pub fn local_keystore() -> Result<SuiKeystore, anyhow::Error> {
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("5ac37128a02c0e83360e315c01a50103b3bd212c26f96818a0748e27a4159de0.key"),
        None => panic!("Cannot obtain home directory path"),
    };

    let keystore = KeystoreType::File(keystore_path).init()?;
    Ok(keystore)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn load_keystore_test() {
        let _result = local_keystore().unwrap();
        // println!("{:#?}",result)
    }

}
