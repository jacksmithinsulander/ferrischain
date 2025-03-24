//! # This module responsible for everything regarding Wallet maneuvers 

use k256::{ecdsa::signature::SignerMut, schnorr::SigningKey, schnorr::VerifyingKey};
use k256::schnorr;
use hex;
use super::transaction;
use sha3::{Digest, Sha3_256};

/// Wallet struct, holds all of the relevant fields to build a wallet and sign messages
/// 
/// # Fields
/// 
/// - 'private_key': Private field, full signing key instance
/// - 'public_key': Public field, the public key as a string
/// - 'balance': The balance of the wallet 
/// - 'transactions': List of transactions that this wallet has sent to the blockchain
#[derive(Clone)]
pub struct Wallet {
    private_key: SigningKey,
    pub public_key: String,
    pub balance: u64,
    pub transactions: Option<Vec<Transaction>>
}


#[derive(Clone)]
pub enum Transaction {
    Sent(transaction::Transaction),
    Received(transaction::Transaction)
}

impl Wallet {
    /// Creates a new wallet
    /// 
    /// # Returns
    /// 
    /// The full Wallet type is returned from this function
    pub fn new(seed: String) -> Result<Self, String> {
        let bytes_array_seed = hex::encode(seed);

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

    /// Signs a message using the private key of the wallet
    /// 
    /// # Returns
    /// 
    /// The cryptographic signature is returned
    pub fn sign_message(mut self, message: String) ->  Result<schnorr::Signature, String> {
        let message_bytes = hex::encode(message);

        Ok(self.private_key.sign(&message_bytes.as_bytes()))
    }

    /// Verifies that the message is actually signed by this wallet
    /// 
    /// am a bit unsure about this imlementation
    /// 
    /// # Returns
    /// 
    /// Should be a bool returned
    pub fn verify_message(self, message: String, signature: schnorr::Signature) -> Result<(), String> {
        let public_key_bytes = match hex::decode(self.public_key) {
            Ok(pub_key_bytes) => pub_key_bytes,
            Err(_) => return Err(String::from("Rekt"))
        };

        let verifyer = match VerifyingKey::from_bytes(&public_key_bytes) {
            Ok(verifyer) => verifyer,
            Err(_) => return Err(String::from("Rekt again"))
        };

        let checker = hex::encode(message);

        match verifyer.verify_raw(checker.as_bytes(), &signature) {
            Ok(result) => {
                dbg!(result);
                return Ok(result)
            },
            Err(_) => return Err(String::from("Rekt thrice"))
        };
    }
}