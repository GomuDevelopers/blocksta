//Blockchain data Structure to handle transaction per node
#[derive(Debug)]
pub struct BlockChain {
    field: Type,
}

trait BlockChain {
    fn create_blockchain(&self);
    fn update_blocks_tree(&self);
    fn new_blockchain(&self);
    fn mine_block(&self);
    fn add_block(&self);
}

//Creation of nodes data structures to simultaneously handle blockchains As we know a node contains a copy of the blockchain.
#[derive(Debug)]
struct Node {}

trait Node {
    fn add_node();
    fn evict_node();
    fn get_nodes();
    fn node_is_known();
}
