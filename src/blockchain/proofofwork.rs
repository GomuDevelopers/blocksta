use std::usize;

use super::block::Block;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use ring::digest;
use std::ops::ShlAssign;

#[derive(Debug, Clone)]
pub struct ProofOfWork {
    block: Block,
    target: BigInt,
    //the target is required for mining, where the target is a mathematical result of a formula converted into
    //a hexadecimal number that dictates the mining difficulty/
}

const TARGET_BITS: i32 = 8;
const MAX_NONCE: i64 = i64::MAX;

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);

        target.shl_assign(256 - &TARGET_BITS);
        return ProofOfWork {
            block: block,
            target: target,
        };
    }

    pub fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transaaction_hash = self.block.hash_transaction();
        let timestamp = self.block.get_timestamp();
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transaaction_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());
        data_bytes
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = digest::digest(&digest::SHA256, data.as_slice())
                .as_ref()
                .to_vec();

            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            if hash_int.lt(&self.target) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        println!(" Block running ");
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
}
