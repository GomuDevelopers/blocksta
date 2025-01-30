use std::usize;

use super::block::Block;
use BigInt;
#[derive(Debug)]
pub struct ProofOfWork {
    block: Block,
    target: BigInt,
    //the target is required for mining, where the target is a mathematical result of a formula converted into
    //a hexadecimal number that dictates the mining difficulty/
}

impl ProofOfWork {
    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_digest(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_byte_be(Sign::Plus, hash.as_slice());

            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
                self.prepare_digest()
            }
        }
        println!("");
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
    pub fn new_proof_of_work(block: &Block) -> ProofOfWork {}
    pub fn prepare_digest(nonce: usize) -> Vec<u32> {}
}
