//! Key utilities for DID operations
//!
//! This module provides cryptographic key utilities for creating and
//! manipulating keys used in DIDs.

use base64::Engine;

/// Supported key types
pub enum KeyType {
    /// Ed25519 signatures
    Ed25519,
    /// ECDSA with secp256k1 curve (for Ethereum)
    Secp256k1,
    /// RSA signatures
    RSA,
}

/// A public key used for verification
#[derive(Debug, Clone)]
pub struct PublicKey {
    /// Key type
    pub key_type: String,

    /// Raw key bytes
    pub key_bytes: Vec<u8>,
}

/// A private key used for signing
#[derive(Debug)]
pub struct PrivateKey {
    /// Key type
    pub key_type: String,

    /// Raw key bytes (sensitive)
    key_bytes: Vec<u8>,
}

impl PublicKey {
    /// Create a new public key
    pub fn new(key_type: &str, key_bytes: Vec<u8>) -> Self {
        PublicKey {
            key_type: key_type.to_string(),
            key_bytes,
        }
    }

    /// Convert to base64 representation
    pub fn to_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.key_bytes)
    }

    /// Convert to hex representation
    pub fn to_hex(&self) -> String {
        self.key_bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    /// Verify a signature against the given message
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        match self.key_type.as_str() {
            "Ed25519" => {
                if let Ok(key) = signature::UnparsedPublicKey::new(
                    &signature::ED25519,
                    &self.key_bytes
                ).verify(message, signature) {
                    return true;
                }
                false
            },
            // Other key types would have implementations here
            _ => false,
        }
    }
}

impl PrivateKey {
    /// Create a new private key
    pub fn new(key_type: &str, key_bytes: Vec<u8>) -> Self {
        PrivateKey {
            key_type: key_type.to_string(),
            key_bytes,
        }
    }

    /// Generate a new private key of the specified type
    pub fn generate(key_type: KeyType) -> Result<Self, &'static str> {
        match key_type {
            KeyType::Ed25519 => {
                let rng = rand::SystemRandom::new();
                let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
                    .map_err(|_| "Failed to generate Ed25519 key")?;

                Ok(PrivateKey {
                    key_type: "Ed25519".to_string(),
                    key_bytes: pkcs8_bytes.as_ref().to_vec(),
                })
            },
            _ => Err("Key type not implemented"),
        }
    }

    /// Get the corresponding public key
    pub fn public_key(&self) -> Result<PublicKey, &'static str> {
        match self.key_type.as_str() {
            "Ed25519" => {
                let key_pair = signature::Ed25519KeyPair::from_pkcs8(&self.key_bytes)
                    .map_err(|_| "Invalid Ed25519 key")?;

                Ok(PublicKey {
                    key_type: "Ed25519".to_string(),
                    key_bytes: key_pair.public_key().as_ref().to_vec(),
                })
            },
            _ => Err("Key type not implemented"),
        }
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, &'static str> {
        match self.key_type.as_str() {
            "Ed25519" => {
                let key_pair = signature::Ed25519KeyPair::from_pkcs8(&self.key_bytes)
                    .map_err(|_| "Invalid Ed25519 key")?;

                Ok(key_pair.sign(message).as_ref().to_vec())
            },
            _ => Err("Key type not implemented"),
        }
    }
}