use crypto_hash::{Algorithm, digest};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    // Returns the hash of the bytes
    fn hash(&self) -> Vec<u8> {
        digest(Algorithm::SHA256, &self.bytes())
    }
}