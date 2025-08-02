use std::env;
use crate::did::core::did_document::*;
use crate::did::core::key_utils::*;
use crate::did::core::traits::DIDMethod;
use ssi_jwk::JWK;
use dotenv::dotenv;
use crate::did::methods::ethr::handler::EthrHandler;

pub struct KeyDID;

/// Helpers specific to the Key DID method.
impl KeyDID {
}

/*
 * Helper function to reconstruct a did:key document from the DID string
 */
fn resolve_did_key(did: &str) -> Result<DIDDocument, &'static str> {
    let key_type = decode_key_type(did)?;
    
    // Extract the public key from the DID
    let public_key = did.replace("did:key:", "");
    
    // Create a new document with the key type
    let mut document = DIDDocument::new(did, key_type);
    
    // Create verification method based on key type
    let vm_type = match key_type {
        KeyType::Ed25519 => "Ed25519VerificationKey2020",
        KeyType::Secp256k1 => "EcdsaSecp256k1VerificationKey2019", 
        KeyType::P256 => "P256VerificationKey2021",
    };
    
    let method = VerificationMethod {
        id: format!("#{}", did),
        vm_type: vm_type.to_string(),
        controller: did.to_string(),
        key_material: KeyMaterial::Multibase {
            public_key_multibase: public_key,
        },
    };
    
    // Add the verification method
    document.add_verification_method(&vec![method]);
    
    // Add default authentication capability - reference to the verification method
    document.add_authentication(&vec![Authentication::Reference(format!("#{}", did))]);
    
    Ok(document)
}

/// Implementation of DIDMethod trait for KeyDID
impl DIDMethod for KeyDID {
    /**
    * Generates a new did:key DID and returns a new DIDDocument containing the did:key DID
    */
    fn create_did(options: DIDCreationOptions) -> DIDDocument {
        let did_prefix = String::from("did:key:");

        let key_type = options.key_type.unwrap_or(KeyType::Ed25519);
        let jwk_string = match key_type {
            KeyType::Ed25519 => JWK::generate_ed25519().expect("Failed to generate Ed25519 key"),
            KeyType::Secp256k1 => JWK::generate_secp256k1(),
            KeyType::P256 => JWK::generate_p256(),
        }
        .to_string();

        let serialize_jwk: serde_json::Value =
            serde_json::from_str(&jwk_string).expect("Failed to parse JWK");
        let key_map = hash_jwk(&serialize_jwk).expect("Failed to extract key components");
        let public_key = key_map.get("public_key").expect("Missing public key");
        let key_material = did_prefix + public_key;
        let mut document = DIDDocument::new(&key_material, key_type);

        // Use `if let Some(values)` and `extend` where appropriate
        if options.verification_method.is_none() {
            // Create a verification method from the key material
            let vm_type: &str = match key_type {
                KeyType::Ed25519 => "Ed25519VerificationKey2020",
                KeyType::Secp256k1 => "EcdsaSecp256k1VerificationKey2019",
                KeyType::P256 => "P256VerificationKey2021",
            };
            let method = VerificationMethod {
                id: format!("#{}", key_material.to_string()),
                vm_type: vm_type.to_string(),
                controller: key_material.to_string(),
                key_material: KeyMaterial::Multibase {
                    public_key_multibase: key_material.to_string(),
                },
            };
            document.add_verification_method(&vec![method]);
        } else {
            document.add_verification_method(&options.verification_method.unwrap());
        }

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
            document.add_capability_invocation(&keys)
        }

        if let Some(keys) = options.capability_delegation {
            document.add_capability_delegation(&keys)
        }

        if let Some(services) = options.service {
            document.add_service(&services);
        }
        // Initialize the storage solutions and store the created document
        dotenv().ok();
        let registry_path: String = env::var("DID_REGISTRY_PATH").unwrap();
        crate::did::registry::init_registry(Some(registry_path));
        
        if let Err(err) = crate::did::registry::get_registry().store(document.clone()) {
            // Log error but continue - the document is still valid
            eprintln!("Failed to store DID in registry: {}", err);
        }
        document
    }

    /*
    * Retrieves the current DID Document for an existing DID
    */
    fn resolve_did( did: &str) -> Result<DIDDocument, &'static str> {
        // Validate DID format - support key method
        if !did.starts_with("did:") {
            return Err("Invalid DID: Must start with 'did:'");
        }

        let method = did.split(':').nth(1).unwrap_or("");
        match method {
            "key" => {},
            _ => return Err("Invalid DID: Method must be 'key'"),
        }

        // First, try to retrieve from registry
        let registry_path = env::var("DID_REGISTRY_PATH").unwrap_or_default();
        println!("DID_REGISTRY_PATH: {}", registry_path);
        crate::did::registry::init_registry(Some(registry_path));
        
        if let Ok(Some(stored_document)) = crate::did::registry::get_registry().get(did) {
            return Ok(stored_document);
        }
        
        // If not found in registry, reconstruct from DID string (for did:key method)
        if method == "key" {
            return resolve_did_key(did);
        }

        // For ethr and web methods, we need the registry since they can't be reconstructed
        Err("DID not found in registry and cannot be reconstructed")
    }


    /*
    	* Modifies an existing DID Document (adding keys, rotating keys, changing services, etc.).
     */
    fn update_did(
        did: &str,
        options: DIDCreationOptions,
    ) -> Result<DIDDocument, &'static str> {
        // Validate DID format
        if !did.starts_with("did:key:") {
            return Err("Invalid DID: Must start with 'did:key:'");
        }

        // Resolve existing document
        let mut document = EthrHandler::resolve_did(did)?;

        if let Some(methods) = options.verification_method {
            document.add_verification_method(&methods);
        }

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
            document.add_capability_invocation(&keys)
        }

        if let Some(keys) = options.capability_delegation {
            document.add_capability_delegation(&keys)
        }

        if let Some(services) = options.service {
            document.add_service(&services);
        }

        Ok(document)
    }

    /**
     * Invalidates (deactivates) an existing DID
     */
    fn invalidate_did(did: &str) -> Result<DIDDocument, &'static str> {
        // Validate DID format
        if !did.starts_with("did:key:") {
            return Err("Invalid DID: unsupported key method");
        }

        // Resolve existing document
        let mut document = KeyDID::resolve_did(did)?;

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
            eprintln!("Failed to store deactivated DID in registry: {}", err);
            return Err("Failed to store deactivated DID");
        }

        Ok(document)
    }
}
