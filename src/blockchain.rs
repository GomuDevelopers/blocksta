
use sled::Db;
pub struct Blockchain {}
impl Blockchain {
    pub fn new_blockchain() -> Blockchain {}
    pub fn create_blockchain(genesis_addr: &str) -> Blockchain {}
    pub fn get_db(&self) {}
    pub fn get_tip_hash(&self) -> String {}
    pub fn set_tip_hash(&self, new_tip_hash: &str) {}
    pub fn iterator(&self) {}
}
