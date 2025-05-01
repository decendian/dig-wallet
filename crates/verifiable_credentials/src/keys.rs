use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use base64::{encode, decode};

// Simple utility to generate a random keypair for testing
pub fn generate_random_keypair() -> Keypair {
    let mut csprng = OsRng{};
    Keypair::generate(&mut csprng)
}

// Hard-coded keypair for demo purposes (NEVER do this in production!)
// In a real system, you'd use secure key storage, HSMs, etc.
pub fn get_demo_keypair() -> Keypair {
    // This is a fixed test key - NEVER use in production!
    let secret_bytes: [u8; 32] = [
        157, 97, 177, 157, 239, 253, 90, 96,
        186, 132, 74, 244, 146, 236, 44, 196,
        68, 73, 197, 105, 123, 50, 105, 25,
        112, 59, 172, 3, 28, 174, 127, 96
    ];
    
    let secret = SecretKey::from_bytes(&secret_bytes).unwrap();
    let public = PublicKey::from(&secret);
    Keypair { secret, public }
}

// Sign data with Ed25519
pub fn sign_data(data: &[u8], keypair: &Keypair) -> String {
    // Hash the data first using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    
    // Sign the hash
    // Sign the given message and return a digital signature
    /// msg: &[u8] - The message to sign
    /// keypair: &Keypair - The keypair to use for signing
    let signature = keypair.sign(&hash);
    
    // Encode the signature as base64
    encode(signature.to_bytes())
}

// Verify a signature
pub fn verify_signature(data: &[u8], signature_b64: &str, public_key: &PublicKey) -> Result<bool, String> {
    // Decode the signature
    let signature_bytes = decode(signature_b64)
        .map_err(|_| "Invalid signature encoding".to_string())?;
    let signature = Signature::from_bytes(&signature_bytes)
        .map_err(|_| "Invalid signature format".to_string())?;
    
    // Hash the data
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    
    // Verify
    match public_key.verify(&hash, &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}