//! # Merkle Tree
//!
//! This package provides a Merkle tree implementation in Rust. A Merkle tree is
//! a hash-based data structure that is used to verify the integrity of data. It
//! is commonly used in distributed  systems, such as blockchain, to ensure that
//! data has not been tampered with.
//!
//! ## Usage
//!
//! To use this package, add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! merkle-tree = "0.1.0"
//! ```
//!
//! Then, in your Rust code, you can use the `MerkleTree` struct to create a new
//! Merkle tree:
//!
//! ```rust
//! use merkle_tree::{MerkleTree, Sha256Hasher};
//!
//! let data = vec!["hello", "world"].iter().map(|s| s.as_bytes().to_vec()).collect();
//! let tree = MerkleTree::<Sha256Hasher>::new(&data);
//! ```
//!
//!
//! To see examples of how to use this package, please refer to the `examples`
//! directory in the source code.
//!
//! ## References
//!
//! * [Merkle tree - Wikipedia](https://en.wikipedia.org/wiki/Merkle_tree)
//! * [Mastering Bitcoin: Unlocking Digital Cryptocurrencies](https://www.oreilly.com/library/view/mastering-bitcoin/9781491902639/ch07.html)
mod hasher;
mod tree;
mod error;

pub use hasher::*;
pub use tree::*;