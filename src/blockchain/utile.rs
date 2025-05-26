use std::{
    i64,
    io::{repeat, Read},
    time::{SystemTime, UNIX_EPOCH},
};

use crypto::{digest::Digest, ripemd160};
use ring::digest::{Context, SHA256};

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_millis() as i64
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = ripemd160::Ripemd160::new();
    ripemd160.input(data);
    let mut buf: Vec<u8> = repeat(0).take(ripemd160.output_bytes()).collection();
    ripemd160.result(&mut buf);
    return buf;
}
