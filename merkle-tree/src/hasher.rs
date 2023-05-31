use sha2::{Digest, Sha256};

///  # Hasher Trait
///
///  This trait defines the interface for a hash function that can be used with
/// a  Merkle tree. By implementing this trait, you can use any hash function
/// with  the Merkle tree, making it more flexible and customizable.
///
///  ## Usage
///
///  To use this trait, you need to implement the `Hasher` trait for your hash
///  function. The trait has a single method, `hash`, which takes a byte slice
///  and returns a fixed-size array of bytes:
///
///  ```rust
///  use sha2::{Digest, Sha256};
///  use merkle_tree::Hasher;
///
///  struct Sha256Hasher;
///
///  impl Hasher for Sha256Hasher {
///      type Hash = [u8; 32];
///
///      fn hash<T: AsRef<[u8]>>(data: T) -> Self::Hash {
///          let mut hasher = Sha256::new();
///          hasher.update(data);
///          hasher.finalize().into()
///      }
///  }
///  let hash = Sha256Hasher::hash("hello world".as_bytes());
///  assert_eq!(hash, [185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218,
///  125, 171, 250, 196, 132, 239, 227, 122, 83, 128, 238, 144, 136, 247, 172,
///  226, 239, 205, 233]);
/// ```
pub trait Hasher {
    /// The output type of the hash function.
    type Hash: Clone + AsRef<[u8]> + Default + PartialEq;

    /// Computes the hash of the given data.
    fn hash<T: AsRef<[u8]>>(data: T) -> Self::Hash;
}

pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
    type Hash = [u8; 32];

    fn hash<T: AsRef<[u8]>>(data: T) -> Self::Hash {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

