//! # This module is for everything regarding blocks, and how to build them

use super::transaction::Transaction;

/// Block struct, this will be created once a block is validated and signed
/// 
/// # Fields
/// 
/// - 'transactions': An array of transactions, fetched from the mempool and used here
/// - 'miner': Public key of whoever created the block
/// - 'timestamp': Timestamp that the block got validated
/// - 'nonce': Nonce used to get the correct hash
/// - 'hash': The actuall hash of the block
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub miner: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub hash: String
}