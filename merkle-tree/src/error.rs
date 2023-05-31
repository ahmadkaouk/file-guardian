#[derive(Debug, PartialEq)]
pub enum MerkleTreeError {
    EmptyData,
    InvalidIndex,
    InvalidProof,
}
