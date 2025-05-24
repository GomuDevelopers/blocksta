use std::{sync::RwLock, u8};

pub struct Node {
    addr: String,
}

pub struct Nodes {
    inner: RwLock<Vec<u8>>,
}
