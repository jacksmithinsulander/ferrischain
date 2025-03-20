use k256::{ecdsa::signature::SignerMut, schnorr::SigningKey};
use k256::schnorr;
use hex;
use super::transaction;
use sha3::{Digest, Sha3_256};

pub struct Wallet {
    private_key: SigningKey,
    pub public_key: String,
    pub balance: u64,
    pub transactions: Option<Vec<Transaction>>
}

pub enum Transaction {
    Sent(transaction::Transaction),
    Received(transaction::Transaction)
}

impl Wallet {
    pub fn new(seed: String) -> Result<Self, String> {
        let bytes_array_seed = match hex::decode(seed) {
            Ok(seed_arr) => seed_arr,
            Err(_) => return Err(String::from("Rekt"))
        };

        let mut hasher = Sha3_256::new();

        hasher.update(bytes_array_seed);

        let hash = hasher.finalize();

        let signer = match SigningKey::from_bytes(&hash) {
            Ok(signing_key) => signing_key,
            Err(_) => return Err(String::from("Rekt"))
        };

        let pub_key = hex::encode(signer.verifying_key().to_bytes());

        Ok(Wallet {
            private_key: signer,
            public_key: pub_key,
            balance: 0,
            transactions: None
        })
    }

    pub fn sign_message(mut self, message: String) ->  Result<schnorr::Signature, String> {
        let message_bytes = match hex::decode(message) {
            Ok(res) => res,
            Err(_) => return Err(String::from("rekt"))
        };

        Ok(self.private_key.sign(&message_bytes))
    }

    pub fn get_wallet(public_key: String) -> Self {
        todo!();
    }
}