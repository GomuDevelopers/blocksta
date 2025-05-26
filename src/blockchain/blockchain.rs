use sled::{Db, Tree};
use std::{
    collections::HashMap,
    env::current_dir,
    sync::{Arc, RwLock},
    u8, usize,
};

use super::{
    block::Block,
    transaction::{TXOutput, Transaction},
};
//here we are using Arc cuz it's safe for sharing Shared owned data accross  Threads.
//using an atomic reference counter which as default of 1 , when it's not shared .
//instead of simple referencing/borrowing who's not trait save

const BLOCK_TREE: &str = "block";
const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";

pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>,
    db: Db,
}
impl Blockchain {
    pub fn new_blockchain() -> Blockchain {
    }
    pub fn create_blockchain(genesis_addr: &str) -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCK_TREE).unwrap();
        let data= blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash;

        if data.is_none(){
            let coinbase_tx=
        }
        
    }
    pub fn update_block_tree(block_tree: &Tree, block: &Block) {}
    pub fn get_db(&self) {}
    pub fn get_tip_hash(&self) -> String {}
    pub fn set_tip_hash(&self, new_tip_hash: &str) {}
    pub fn mine_block(&self, transactons: &[Transaction]) -> Block {}
    pub fn iterator(&self) {}
    pub fn find_utxo(&self) -> HashMap<String, Vec<TXOutput>> {}
    pub fn find_transaction(&self, txid: &[u8]) -> Option<Transaction> {}
    pub fn add_block(&self, block: &Block) {}
    pub fn get_best_height(&self) -> usize {}
    pub fn get_block(&self, block_hash: &[u8]) -> Option<Block> {}
    pub fn get_block_hashes(&self) -> Vec<Vec<u8>> {}
}

pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}
impl BlockchainIterator {
    fn new(tip_hash: String, db: Db) -> BlockchainIterator {}
    pub fn next(&mut self) -> Option<Block> {}
}
