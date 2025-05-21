extern crate bincode;
use super::proofofwork::ProofOfWork;
use super::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{u8, usize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Block {
    timestamp: u64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(
        pre_block_hash: String,
        trasactions_input: &[Transaction],
        height: usize,
    ) -> Block {
        let block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions_input.to_vec(),
            nonce: 0,
            height,
        };

        let pow = ProofOfWork::new_proof_of_work(block);
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }
    pub fn deserialization(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }
    pub fn serialization(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        Block::new_block(String::from("None"), &transactions, 0)
    }
    pub fn hash_transaction(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in self.transactions {
            txhashs.extend(transaction.get_id());
        }
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> &str {
        self.pre_block_hash.as_ref()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_ref()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
