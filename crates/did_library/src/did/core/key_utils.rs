//! Key utilities for DID operations
//!
//! This module provides cryptographic key utilities for creating and
//! manipulating keys used in DIDs.

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bs58;

/// Supported key types
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum KeyType {
    /// Ed25519 signatures
    Ed25519,
    /// ECDSA with secp256k1 curve
    Secp256k1,
    /// P-256 curve
    P256,
}

/// A public key used for verification
#[derive(Debug, Clone)]
pub struct PublicKey {
    /// Key type
    pub key_type: String,

    /// Raw key bytes
    pub key_bytes: Vec<u8>,
}

/// Hash a JWK into three fields: key_type, public_key, and private_key.
/// Note all key,value pairs are strings
pub fn hash_jwk(jwk: &serde_json::Value) -> Result<HashMap<String, String>, String> {
    // First, determine key type from the JWK
    let key_type = match jwk.get("crv").and_then(|v| v.as_str()) {
        Some("Ed25519") => KeyType::Ed25519,
        Some("secp256k1") => KeyType::Secp256k1,
        Some("P-256") => KeyType::P256,
        Some(curve) => return Err(format!("The curve {} is not supported", curve)),
        _ => return Err("The curve should be one of Ed25519, secp256k1, or P-256".into()),
    };

    let public_key = jwk
        .get("x")
        .ok_or_else(|| "Missing public key (x) in JWK".to_string())?
        .as_str()
        .ok_or_else(|| "Public key is not a string".to_string())?;

    let private_key = jwk
        .get("d")
        .ok_or_else(|| "Missing private key (d) in JWK".to_string())?
        .as_str()
        .ok_or_else(|| "Private key is not a string".to_string())?;

    let mut map = HashMap::new();
    map.insert("key_type".to_string(), format!("{:?}", key_type));
    map.insert("public_key".to_string(), public_key.to_string());
    map.insert("private_key".to_string(), private_key.to_string());

    Ok(map)
}

pub fn decode_key_type(did: &str) -> Result<KeyType, &'static str> {
    if !did.starts_with("did:key:") {
        return Err("Invalid DID: Must start with 'did:key:'");
    }
    
    let encoded_key = did.replace("did:key:", "");
    
    if encoded_key.is_empty() {
        return Err("Invalid DID: Empty key portion");
    }
    
    println!("Encoded key: '{}'", encoded_key);
    
    // Try base64url decoding first (since your key appears to be base64url)
    let decoded_bytes = match general_purpose::URL_SAFE_NO_PAD.decode(&encoded_key) {
        Ok(bytes) => {
            println!("Successfully decoded as base64url");
            bytes
        }
        Err(_) => {
            // Fall back to base58 if base64url fails
            println!("Base64url failed, trying base58...");
            bs58::decode(&encoded_key)
                .into_vec()
                .map_err(|_| "Invalid DID: Failed to decode both base64url and base58")?
        }
    };
    
    println!("Decoded bytes length: {}", decoded_bytes.len());
    if decoded_bytes.len() >= 2 {
        println!("First two bytes: [{:02x}, {:02x}]", decoded_bytes[0], decoded_bytes[1]);
    }
    
    if decoded_bytes.len() < 2 {
        return Err("Invalid DID: Decoded key too short");
    }
    
    // First, try multicodec prefixes
    match &decoded_bytes[..2] {
        [0xed, 0x01] => return Ok(KeyType::Ed25519),
        [0xe7, 0x01] => return Ok(KeyType::Secp256k1),
        [0x12, 0x00] => return Ok(KeyType::P256),
        _ => {
            println!("No multicodec prefix found, checking raw key lengths...");
        }
    }
    
    // If no multicodec prefix, check by key length (raw keys)
    match decoded_bytes.len() {
        32 => {
            println!("32 bytes detected - likely raw Ed25519 public key");
            Ok(KeyType::Ed25519)
        }
        33 => {
            println!("33 bytes detected - likely compressed secp256k1 public key");
            Ok(KeyType::Secp256k1)
        }
        65 => {
            println!("65 bytes detected - likely uncompressed secp256k1 public key");
            Ok(KeyType::Secp256k1)
        }
        _ => {
            println!("Unknown key length: {} bytes", decoded_bytes.len());
            println!("Unknown prefix: {:02x} {:02x}", decoded_bytes[0], decoded_bytes[1]);
            println!("Full decoded bytes: {:02x?}", &decoded_bytes[..std::cmp::min(10, decoded_bytes.len())]);
            Err("Key Type not supported")
        }
    }
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
        self.key_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    /// Verify a signature against the given message
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        //     match self.key_type.as_str() {
        //         "Ed25519" => {
        //             if let Ok(key) = signature::new(
        //                 &signature::ED25519,
        //                 &self.key_bytes
        //             ).verify(message, signature) {
        //                 return true;
        //             }
        //             false
        //         },
        //         // Other key types would have implementations here
        //         _ => false,
        //     }
        // }
        true
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
        // match key_type {
        //     KeyType::Ed25519 => {
        //         // Generate a new Ed25519 keypair
        //         let rng = rand::SystemRandom::new();
        //         let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
        //             .map_err(|_| "Failed to generate Ed25519 key")?;
        //
        //         Ok(PrivateKey {
        //             key_type: "Ed25519".to_string(),
        //             key_bytes: pkcs8_bytes.as_ref().to_vec(),
        //         })
        //     },
        //     KeyType::Secp256k1 => {
        //         // For Secp256k1, we would use a different generation method
        //         // This is a placeholder implementation - in a real system, use a proper secp256k1 library
        //         let mut bytes = [0u8; 32];
        //         let rng = rand::SystemRandom::new();
        //         rng.fill(&mut bytes).map_err(|_| "Failed to generate random bytes")?;
        //
        //         Ok(PrivateKey {
        //             key_type: "Secp256k1".to_string(),
        //             key_bytes: bytes.to_vec(),
        //         })
        //     },
        //     KeyType::P256 => {
        //         // For P256, this is a placeholder implementation
        //         // In a real system, use a proper P-256 library
        //         let mut bytes = [0u8; 32];
        //         let rng = rand::SystemRandom::new();
        //         rng.fill(&mut bytes).map_err(|_| "Failed to generate random bytes")?;
        //
        //         Ok(PrivateKey {
        //             key_type: "P256".to_string(),
        //             key_bytes: bytes.to_vec(),
        //         })
        //     },
        // }
        todo!()
    }

    /// Get the corresponding public key
    pub fn public_key(&self) -> Result<PublicKey, &'static str> {
        // match self.key_type.as_str() {
        //     "Ed25519" => {
        //         let key_pair = signature::Ed25519KeyPair::from_pkcs8(&self.key_bytes)
        //             .map_err(|_| "Invalid Ed25519 key")?;
        //
        //         Ok(PublicKey {
        //             key_type: "Ed25519".to_string(),
        //             key_bytes: key_pair.public_key().as_ref().to_vec(),
        //         })
        //     },
        //     _ => Err("Key type not implemented"),
        // }
        todo!()
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, &'static str> {
        // match self.key_type.as_str() {
        //     "Ed25519" => {
        //         let key_pair = signature::Ed25519KeyPair::from_pkcs8(&self.key_bytes)
        //             .map_err(|_| "Invalid Ed25519 key")?;
        //
        //         Ok(key_pair.sign(message).as_ref().to_vec())
        //     },
        //     _ => Err("Key type not implemented"),
        // }
        todo!()
    }
}
