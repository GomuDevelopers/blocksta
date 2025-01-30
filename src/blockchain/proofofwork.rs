use std::usize;

use super::block::Block;
use num_bigint::{BigInt, Sign};
#[derive(Debug)]
pub struct ProofOfWork {
    block: Block,
    target: &BigInt,
    //the target is required for mining, where the target is a mathematical result of a formula converted into
    //a hexadecimal number that dictates the mining difficulty/
}

impl ProofOfWork {
    pub fn prepare_digest(nonce: usize) -> Vec<u32> {}

    pub fn sha256_digest() {}

    pub fn run(&self) -> (i64, String) {
        const MAX_NONCE: u32 = 100;
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_digest(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, bytes);

            if hash_int.lt(self.target) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
                self.prepare_digest()
            }
        }
        println!(" Block running ");
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }

    pub fn new_proof_of_work(block: &Block) -> ProofOfWork {}
}
