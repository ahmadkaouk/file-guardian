use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MerkleTreeError {
    #[error("Empty data")]
    EmptyData,
    #[error("Invalid index")]
    InvalidIndex,
    #[error("Invalid proof")]
    InvalidProof,
}
