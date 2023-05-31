#[derive(Debug)]
pub enum MerkleTreeError {
    EmptyData,
    InvalidIndex,
    InvalidProof,
}
