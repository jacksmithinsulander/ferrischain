use super::Transaction;

struct Block {
    transactions: Vec<Transaction>,
    miner: String
}