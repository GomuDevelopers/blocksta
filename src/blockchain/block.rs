extern crate bincode;
use super::proofofwork::ProofOfWork;
use super::transaction::Transaction;
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{u8, usize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Block {
    timestamp: u64,
    pre_block_hash: String,
    hash: String,
    transaction: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            pre_block_hash,
            hash: String::new(),
            transaction: transactions.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(&block);
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }
    pub fn deserialization(bytes: &[u8]) -> Block {
        deserialize(bytes).unwrap()
    }
    pub fn serialization(&self) -> Vec<u8> {
        serialize(self).unwrap().to_vec()
    }
}
