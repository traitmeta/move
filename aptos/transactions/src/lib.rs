use ed25519_dalek::{PublicKey, SecretKey};
use hex::ToHex;
use rand::{rngs::OsRng, Rng, RngCore, SeedableRng};
use tiny_keccak::{Hasher, Sha3};

pub struct Account {
    signing_key: SecretKey,
}

impl Account {
    pub fn new(private_key_bytes: Option<Vec<u8>>) -> Account {
        let signing_key = match private_key_bytes {
            Some(key) => SecretKey::from_bytes(&key).unwrap(),
            None => {
                let mut rng = rand::rngs::StdRng::from_seed(OsRng.gen());
                let mut bytes = [0; 32];
                rng.fill(&mut bytes);
                SecretKey::from_bytes(&bytes).unwrap()
            }
        };
        Account { signing_key }
    }

    pub fn addresses(&self) -> String {
        self.auth_key()
    }

    pub fn auth_key(&self) -> String {
        let mut sha3 = Sha3::v256();
        sha3.update(PublicKey::from(&self.signing_key).as_bytes());
        sha3.update(&[0u8]);
        let mut output = vec![0u8; 32];
        sha3.finalize(&mut output);
        hex::encode(&output)
    }
    pub fn public_key(&self) -> String {
        hex::encode(PublicKey::from(&self.signing_key).as_bytes())
    }
}
