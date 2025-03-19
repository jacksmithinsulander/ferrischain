//! # This module responsible for everything regarding transactions

/// Transaction struct, contains all relevant info that together creates the transaction
/// 
/// # Fields
/// 
/// - 'payee': Public key of who sent the transaction
/// - 'recipient': Public key of who is receiving the transaction
/// - 'value': Value of the transaction
/// - 'nonce': Payees nonce, linked to this transaction, typically an ascending number
/// - 'gas': Amount of gas sent to the transaction as reward for the miner including it to the chain
/// - 'hash': All information of this transaction, hashed together
/// - 'message': An optional message field that the payee can use to write something fun to the recipient
pub struct Transaction {
    pub payee: String,
    pub recipient: String,
    pub value: u64,
    pub nonce: u64,
    pub gas: u64,
    pub hash: String,
    pub message: String
}

impl Transaction {
    /// Creates a new transaction
    /// 
    /// # Returns
    /// 
    /// The full Transaction type is returned from this function
    pub fn new(payee: String, recipient: String, value: u64, nonce: u64, gas: u64, hash: String, message: String) -> Self {
        Self {
            payee,
            recipient,
            value,
            nonce,
            gas,
            hash,
            message
        }
    }
}