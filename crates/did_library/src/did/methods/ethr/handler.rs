use crate::did::core::did_document::*;
use crate::did::core::key_utils::*;
use crate::did::core::traits::DIDMethod;
use crate::DIDDocument;
use ssi::jwk::JWK;
use std::env;


// Additional imports for ethr DID method
use ethereum_types::H160;
use tiny_keccak::{Hasher, Keccak};
use hex;
use base64;

pub struct EthrHandler;

impl EthrHandler {
    pub fn new() -> Self {
        Self {}
    }
}

// This function correctly derives an Ethereum address from a secp256k1 public key
fn derive_ethereum_address(public_key: &[u8]) -> Result<String, &'static str> {
    // Ensure we have a valid public key - uncompressed secp256k1 public key is 65 bytes
    // (It should start with 0x04 followed by the x and y coordinates, each 32 bytes)
    if public_key.len() < 65 || public_key[0] != 0x04 {
        return Err("Invalid public key format for Ethereum address derivation");
    }

    // Remove the 0x04 prefix - we only hash the x and y coordinates
    let key_without_prefix = &public_key[1..];

    // Calculate Keccak-256 hash of the public key (without the prefix)
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];
    keccak.update(key_without_prefix);
    keccak.finalize(&mut hash);

    // Take the last 20 bytes of the hash for the Ethereum address
    let eth_address = &hash[12..32];
    
    // Format as hex string with 0x prefix
    let address_hex = format!("0x{}", hex::encode(eth_address));
    
    Ok(address_hex.to_lowercase())
}


fn decode_base64url(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    // Convert base64url to base64 by replacing URL-safe characters
    let mut base64_string = input.replace('-', "+").replace('_', "/");
    
    // Add proper padding if needed
    let padding_needed = (4 - (base64_string.len() % 4)) % 4;
    for _ in 0..padding_needed {
        base64_string.push('=');
    }
    
    base64::decode(&base64_string)
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
fn parse_ethr_did(did: &str) -> Result<(String), &'static str> {
    // Remove the "did:ethr:" prefix
    let remainder = did.strip_prefix("did:ethr:")
        .ok_or("Invalid ethr DID format")?;
    
    // Check if network is specified
    if remainder.starts_with("0x") {
        // No network specified, default to mainnet
        let address = remainder.to_string();
        // Ok(("mainnet".to_string(), address)) # Uncomment when network is implemented
        Ok(address) // Temporary return for address only
    } else {
        // Network is specified, parse it
        let parts: Vec<&str> = remainder.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("Invalid ethr DID format: missing address after network");
        }
        
        let network = parts[0].to_string();
        let address = parts[1].to_string();
        
        // Validate network name (basic validation)
        if network.is_empty() {
            return Err("Invalid ethr DID format: invalid network or address");
        }
        
        Ok(address) // add network to return when network is implemented
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
    
    
    Ok(())
}

// Helper function to verify EIP-55 checksum, for strict validation
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
        println!("Creating ethr DID");
        let did_prefix = String::from("did:ethr:");

        let network = format!("{}:", options.network.unwrap_or("mainnet".to_string()));
        
        // For ethr DID, we primarily use Secp256k1 keys (Ethereum standard)
        let key_type = options.key_type.unwrap_or(KeyType::Secp256k1);
        
        // Generate the secp256k1 key
        let jwk_result: Result<JWK, std::io::Error> = match key_type {
            KeyType::Secp256k1 => Ok(JWK::generate_secp256k1()),
            // For ethr, we should primarily use secp256k1, but allow other types x
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unsupported key type for ethr DID",
            )),
        };
        let jwk_string = jwk_result.expect("Unsupported key type for ethr DID").to_string();

        let serialize_jwk: serde_json::Value =
            serde_json::from_str(&jwk_string).expect("Failed to parse JWK");
        
        // Extract key info using hash_jwk function
        let key_map = hash_jwk(&serialize_jwk).expect("Failed to extract key components");
        let public_key = key_map.get("public_key").expect("Missing public key");
        
        // For Ethereum address derivation, we still need raw public key bytes
        // Extract x and y coordinates directly from JWK for secp256k1 keys
        let ethereum_address = match key_type {
            KeyType::Secp256k1 => {
                // Extract x and y coordinates from JWK
                let x_coord = serialize_jwk.get("x")
                    .and_then(|v| v.as_str())
                    .expect("Missing x coordinate in JWK");
                let y_coord = serialize_jwk.get("y")
                    .and_then(|v| v.as_str())
                    .expect("Missing y coordinate in JWK");
                
                // Decode base64url encoded coordinates with proper padding
                let x_bytes = decode_base64url(x_coord)
                    .expect("Failed to decode x coordinate");
                let y_bytes = decode_base64url(y_coord)
                    .expect("Failed to decode y coordinate");
                
                // Create uncompressed public key format: 0x04 + x + y
                let mut public_key_bytes = Vec::with_capacity(65);
                public_key_bytes.push(0x04); // Uncompressed point format
                public_key_bytes.extend_from_slice(&x_bytes);
                public_key_bytes.extend_from_slice(&y_bytes);
                
                // Derive Ethereum address from public key
                derive_ethereum_address(&public_key_bytes)
                    .expect("Failed to derive Ethereum address")
            },
            _ => {
                // For non-secp256k1 keys, create a placeholder address
                // In practice, ethr DID should primarily use secp256k1
                format!("0x{}", hex::encode(&public_key.as_bytes()[..20]))
            }
        };
        
        // Build the complete DID identifier with the Ethereum address
        // Apply EIP-55 checksumming to the address
        let checksummed_address = if ethereum_address.starts_with("0x") {
            let h160_addr = ethereum_address[2..].parse::<H160>()
                .expect("Failed to parse Ethereum address");
            checksum_address(&h160_addr)
        } else {
            ethereum_address
        };
        
        // Remove 0x prefix for DID identifier
        let did_identifier = format!("{}{}{}", did_prefix, network, checksummed_address);
        
        let mut document = DIDDocument::new(&did_identifier, key_type);

        // Handle verification methods
        if options.verification_method.is_none() {
            // Create a verification method from the Ethereum address
            let vm_type: &str = match key_type {
                KeyType::Secp256k1 => "EcdsaSecp256k1RecoveryMethod2020", // Ethereum standard
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
        let ethereum_address: String = parse_ethr_did(did)?;
        
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

    /**
     * Invalidates (deactivates) an existing Ethereum DID
     */
    fn invalidate_did(&self, did: &str) -> Result<DIDDocument, &'static str> {
        // Validate DID format - only accept Ethereum DIDs
        if !did.starts_with("did:ethr:") {
            return Err("Invalid DID: only Ethereum DID method is supported");
        }

        // Resolve existing document
        let mut document = self.resolve_did(did)?;

        // Check if already inactive
        if document.status != "active" {
            return Err("DID is already inactive");
        }

        // Change status to deactivated
        document.status = "deactivated".to_string();

        // Store the updated document
        let registry_path = env::var("DID_REGISTRY_PATH")
            .unwrap_or_else(|_| "./did_registry".to_string());
        crate::did::registry::init_registry(Some(registry_path));
        
        if let Err(err) = crate::did::registry::get_registry().store(document.clone()) {
            eprintln!("Failed to store deactivated Ethereum DID in registry: {}", err);
            return Err("Failed to store deactivated DID");
        }

        Ok(document)
    }
}
