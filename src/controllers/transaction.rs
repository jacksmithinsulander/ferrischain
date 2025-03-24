//! # This module responsible for everything regarding transactions

use k256::schnorr::Signature;
use crate::controllers::wallet::Wallet;
use std::fmt;

/// Transaction struct, contains all relevant info that together creates the transaction
/// 
/// # Fields
/// 
/// - 'payee': Public key of who sent the transaction
/// - 'recipient': Public key of who is receiving the transaction
/// - 'value': Value of the transaction
/// - 'gas': Amount of gas sent to the transaction as reward for the miner including it to the chain
/// - 'nonce': Payees nonce, linked to this transaction, typically an ascending number
/// - 'timestamp': Timestamp where the transaction was sent to the mempool
/// - 'message': An optional message field that the payee can use to write something fun to the recipient
#[derive(Clone)]
pub struct TxInfo {
    pub payee: String,
    pub recipient: String,
    pub value: u64,
    pub gas: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub message: String
}

#[derive(Clone)]
pub struct Transaction {
    pub tx_info: TxInfo,
    pub hash: Signature
}

impl fmt::Display for TxInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {} {} {} {}", self.payee, self.recipient, self.value, self.gas, self.nonce, self.timestamp, self.message)
    }
}

impl Transaction {
    /// Creates a new transaction
    /// 
    /// # Returns
    /// 
    /// The full Transaction type is returned from this function
    pub fn new(signer: Wallet, recipient: String, value: u64, gas: u64, nonce: u64, timestamp: u64, message: String) -> Result<Self, String> {

        let transaction_info = TxInfo {
            payee: signer.public_key.clone(),
            recipient: recipient,
            value: value,
            gas: gas,
            nonce: nonce,
            timestamp: timestamp,
            message: message,
        };

        let info_string = transaction_info.to_string();

        match signer.sign_message(info_string) {
            Ok(hash) => {
                Ok(Transaction { tx_info: transaction_info, hash: hash })
            },
            Err(e) => return Err(e),
        }
    }
}