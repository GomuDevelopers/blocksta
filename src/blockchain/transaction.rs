use serde::{Deserialize, Serialize};

use super::wallet::Wallet;

const SUBSIDY: i32 = 10;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),
            vout: vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }

    pub fn get_signature(&self) -> &[u8] {
        self.signature.as_slice()
    }

    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }

    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        // compare method if input transaction but key is same as Wallet pub key
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

impl Transaction {
    pub fn get_id(&self) {}
}
