use crate::did::core::did_document::*;
use crate::did::core::key_utils::*;
use crate::did::core::traits::DIDMethod;
use crate::DIDDocument;
use ssi::jwk::JWK;

// Additional imports for ethr DID method
use ethereum_types::H160;
use tiny_keccak::{Hasher, Keccak};
use hex;

pub struct EthrHandler;

impl EthrHandler {
    pub fn new() -> Self {
        Self {}
    }
}

// Helper function to derive Ethereum address from public key
fn derive_ethereum_address_from_public_key(public_key_hex: &str) -> Result<String, &'static str> {
    // Remove any prefix (0x) and decode hex
    let public_key_hex = public_key_hex.strip_prefix("0x").unwrap_or(public_key_hex);
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|_| "Invalid public key hex encoding")?;
    
    // For secp256k1, we expect 64 bytes (32 bytes x + 32 bytes y, uncompressed without 0x04 prefix)
    // If it includes the 0x04 prefix, remove it
    let key_bytes = if public_key_bytes.len() == 65 && public_key_bytes[0] == 0x04 {
        &public_key_bytes[1..]
    } else if public_key_bytes.len() == 64 {
        &public_key_bytes
    } else {
        return Err("Invalid public key length for secp256k1");
    };
    
    // Perform Keccak-256 hash
    let mut hasher = Keccak::v256();
    hasher.update(key_bytes);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    
    // Take last 20 bytes as Ethereum address
    let address_bytes = &hash[12..];
    let address = H160::from_slice(address_bytes);
    
    // Apply EIP-55 checksumming
    let checksummed_address = checksum_address(&address);
    
    Ok(checksummed_address)
}

// Helper function to apply EIP-55 checksumming to Ethereum address
fn checksum_address(address: &H160) -> String {
    let address_str = format!("{:x}", address);
    
    // Hash the address string (lowercase, no 0x prefix)
    let mut hasher = Keccak::v256();
    hasher.update(address_str.as_bytes());
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    
    // Apply checksumming
    let mut result = String::with_capacity(42);
    result.push_str("0x");
    
    for (i, c) in address_str.chars().enumerate() {
        if c.is_ascii_hexdigit() && c.is_alphabetic() {
            // Check if the corresponding bit in hash is set
            let byte_index = i / 2;
            let bit_index = if i % 2 == 0 { 7 } else { 3 };
            
            if (hash[byte_index] >> bit_index) & 1 == 1 {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

// Helper function to parse ethr DID and extract network and address
fn parse_ethr_did(did: &str) -> Result<(String, String), &'static str> {
    // Remove the "did:ethr:" prefix
    let remainder = did.strip_prefix("did:ethr:")
        .ok_or("Invalid ethr DID format")?;
    
    // Check if network is specified
    if remainder.starts_with("0x") {
        // No network specified, default to mainnet
        let address = remainder.to_string();
        Ok(("mainnet".to_string(), address))
    } else {
        // Network is specified, parse it
        let parts: Vec<&str> = remainder.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("Invalid ethr DID format: missing address after network");
        }
        
        let network = parts[0].to_string();
        let address = parts[1].to_string();
        
        // Validate network name (basic validation)
        if network.is_empty() || !address.starts_with("0x") {
            return Err("Invalid ethr DID format: invalid network or address");
        }
        
        Ok((network, address))
    }
}

// Helper function to validate Ethereum address format
fn validate_ethereum_address(address: &str) -> Result<(), &'static str> {
    // Check if it starts with 0x
    if !address.starts_with("0x") {
        return Err("Invalid Ethereum address: must start with 0x");
    }
    
    // Check length (0x + 40 hex characters = 42 total)
    if address.len() != 42 {
        return Err("Invalid Ethereum address: must be 42 characters long");
    }
    
    // Check if all characters after 0x are valid hexadecimal
    let hex_part = &address[2..];
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid Ethereum address: contains non-hexadecimal characters");
    }
    
    // Optionally validate EIP-55 checksum (current implementation accepts both)
    // For strict validation, you could add checksum verification here
    
    Ok(())
}

// Helper function to verify EIP-55 checksum (optional, for strict validation)
fn verify_address_checksum(address: &str) -> bool {
    if address.len() != 42 || !address.starts_with("0x") {
        return false;
    }
    
    let address_lower = address[2..].to_lowercase();
    let expected_checksum = checksum_address(
        &H160::from_slice(&hex::decode(&address_lower).unwrap_or_default())
    );
    
    address == expected_checksum
}


impl DIDMethod for EthrHandler {
    fn create_did(&self, options: DIDCreationOptions) -> DIDDocument {
        let did_prefix = String::from("did:ethr:");
        
        // For ethr DID, we primarily use Secp256k1 keys (Ethereum standard)
        let key_type = options.key_type.unwrap_or(KeyType::Secp256k1);
        
        // Generate the secp256k1 key
        let jwk_string = match key_type {
            KeyType::Secp256k1 => JWK::generate_secp256k1(),
            // For ethr, we should primarily use secp256k1, but allow other types
            KeyType::Ed25519 => JWK::generate_ed25519().expect("Failed to generate Ed25519 key"),
            KeyType::P256 => JWK::generate_p256(),
        }
        .to_string();

        let serialize_jwk: serde_json::Value =
            serde_json::from_str(&jwk_string).expect("Failed to parse JWK");
        let key_map = hash_jwk(&serialize_jwk).expect("Failed to extract key components");
        let public_key = key_map.get("public_key").expect("Missing public key");
        
        // For ethr DID, we need to derive an Ethereum address from the public key
        let ethereum_address = derive_ethereum_address_from_public_key(public_key)
            .expect("Failed to derive Ethereum address");
        
        // Build the complete DID identifier
        // Default to mainnet if no network specified
        let did_identifier = format!("{}{}", did_prefix, ethereum_address);
        
        let mut document = DIDDocument::new(&did_identifier, key_type);

        // Handle verification methods
        if options.verification_method.is_none() {
            // Create a verification method from the Ethereum address
            let vm_type: &str = match key_type {
                KeyType::Secp256k1 => "EcdsaSecp256k1VerificationKey2019",
                KeyType::Ed25519 => "Ed25519VerificationKey2020", 
                KeyType::P256 => "P256VerificationKey2021",
            };
            
            let method = VerificationMethod {
                id: format!("{}#controller", did_identifier),
                vm_type: vm_type.to_string(),
                controller: did_identifier.clone(),
                key_material: KeyMaterial::Multibase {
                    public_key_multibase: public_key.to_string(),
                },
            };
            document.add_verification_method(&vec![method]);
        } else {
            document.add_verification_method(&options.verification_method.unwrap());
        }

        // Add optional properties following the same pattern as key method
        if let Some(auths) = options.authentication {
            document.add_authentication(&auths);
        }

        if let Some(methods) = options.assertion_method {
            document.add_assertion_method(&methods);
        }

        if let Some(keys) = options.key_agreement {
            document.add_key_agreement(&keys);
        }

        if let Some(keys) = options.capability_invocation {
            document.add_capability_invocation(&keys);
        }

        if let Some(keys) = options.capability_delegation {
            document.add_capability_delegation(&keys);
        }

        if let Some(services) = options.service {
            document.add_service(&services);
        }

        // Store the created document in the registry (following key method pattern)
        if let Err(err) = crate::did::registry::get_registry().store(document.clone()) {
            // Log error but continue - the document is still valid
            eprintln!("Failed to store DID in registry: {}", err);
        }
        
        document
    }

    fn resolve_did(&self, did: &str) -> Result<DIDDocument, &'static str> {
        // Validate DID format
        if !did.starts_with("did:ethr:") {
            return Err("Invalid DID: Must start with 'did:ethr:'");
        }

        // Parse the DID to extract network and address
        let (_network, ethereum_address) = parse_ethr_did(did)?;
        
        // Validate the Ethereum address format
        validate_ethereum_address(&ethereum_address)?;
        
        // Verify EIP-55 checksum for additional security
        if !verify_address_checksum(&ethereum_address) {
            return Err("Invalid Ethereum address: EIP-55 checksum verification failed");
        }
        
        // For ethr DID, we use Secp256k1 as the default key type
        let key_type = KeyType::Secp256k1;
        
        // Create the DID document following the key method pattern
        // The basic document will be created, and verification methods/auth
        // can be added through the update_did method or during creation
        Ok(DIDDocument::new(did, key_type))
    }

    fn update_did(
        &self,
        did: &str,
        options: DIDCreationOptions,
    ) -> Result<DIDDocument, &'static str> {
        // Validate DID format
        if !did.starts_with("did:ethr:") {
            return Err("Invalid DID: Must start with 'did:ethr:'");
        }

        // Resolve existing document
        let mut document = self.resolve_did(did)?;

        // Add new verification methods if provided
        if let Some(methods) = options.verification_method {
            document.add_verification_method(&methods);
        }

        // Add authentication methods if provided
        if let Some(auths) = options.authentication {
            document.add_authentication(&auths);
        }

        // Add assertion methods if provided
        if let Some(methods) = options.assertion_method {
            document.add_assertion_method(&methods);
        }

        // Add key agreement methods if provided
        if let Some(keys) = options.key_agreement {
            document.add_key_agreement(&keys);
        }

        // Add capability invocation methods if provided
        if let Some(keys) = options.capability_invocation {
            document.add_capability_invocation(&keys);
        }

        // Add capability delegation methods if provided
        if let Some(keys) = options.capability_delegation {
            document.add_capability_delegation(&keys);
        }

        // Add services if provided
        if let Some(services) = options.service {
            document.add_service(&services);
        }

        // Store the updated document in the registry (following key method pattern)
        if let Err(err) = crate::did::registry::get_registry().store(document.clone()) {
            // Log error but continue - the document is still valid
            eprintln!("Failed to store updated DID in registry: {}", err);
        }

        Ok(document)
    }
}
