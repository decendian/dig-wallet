use std::fmt::{ self, Debug, Formatter };
use super::*;
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    /// Format a `Block` using the format string:
    ///
    /// `Block {index: {}, timestamp: {}, hash: {}, nonce: {}, payload: {}}`
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
       write!(f, "Block {{index: {}, timestamp: {}, hash: {}, nonce: {}, payload: {}}}",
              &self.index,
              &self.timestamp,
              hex::encode(&self.hash),
              &self.nonce,
              &self.payload)
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        previous_hash: Vec<u8>,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            previous_hash,
            hash: vec![0; 16],
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    /// Convert the block into a byte array for hashing.
    ///
    /// # Fields Used in the Byte Array
    /// 1. index: u32
    /// 2. timestamp: u128
    /// 3. previous_hash: Vec<u8>
    /// 4. payload: String
    ///
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.previous_hash);
        bytes.extend(self.payload.as_bytes());
        bytes

    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    todo!("Check if the hash meets the difficulty requirement");
}