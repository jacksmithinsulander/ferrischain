use k256::schnorr::SigningKey;
use hex;
use super::transaction;

pub struct Wallet {
    private_key: SigningKey,
    pub public_key: String,
    pub balance: u64,
    pub transactions: Vec<Transaction>
}

pub enum Transaction {
    Sent(transaction::Transaction),
    Received(transaction::Transaction)
}

impl Wallet {
    fn new(seed: u128) -> Self {
        todo!();
    }

    fn sign_message(self, message: String) -> String {
        todo!();
    }

    fn get_wallet(public_key: String) -> Self {
        todo!();
    }
}