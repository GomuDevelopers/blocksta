use sled::Db;
use std::sync::{Arc, RwLock};
//here we are using Arc cuz it's safe for sharing Shared owned data accross  Threads.
//using an atomic reference counter which as default of 1 , when it's not shared .
//instead of simple referencing/borrowing who's not trait save

pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>,
    db: Db,
}

impl Blockchain {
    pub fn new_blockchain() -> Blockchain {}
    pub fn create_blockchain(genesis_addr: &str) -> Blockchain {}
    pub fn get_db(&self) {}
    pub fn get_tip_hash(&self) -> String {}
    pub fn set_tip_hash(&self, new_tip_hash: &str) {}
    pub fn iterator(&self) {}
}
