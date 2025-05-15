// This is the main entry point for the crate
pub mod format;
pub mod keys;
pub mod presentation; // Add this line

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use did_library::did::core::did_document::*;
use did_library::did::core::key_utils::KeyType;
use crate::format::VerifiableCredential;
use std::env;
use std::path::{PathBuf};

// Re-export important types and functions
#[derive(Serialize, Deserialize, Clone)]
pub struct CredentialRequest {
    pub subject: CredentialSubject,
    pub type_: Vec<String>,
    pub issuer_did: String,
    pub expiration_date: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CredentialSubject {
    pub id: Option<String>,
    pub name: String,
    pub attributes: serde_json::Value,
}

/// Find the project root by looking for the workspace Cargo.toml
fn find_project_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    
    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        
        if cargo_toml_path.exists() {
            if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
                // Check if this is the workspace root Cargo.toml
                if content.contains("[workspace]") && content.contains("\"did_library\"") {
                    return Some(current_dir);
                }
            }
        }
        
        if !current_dir.pop() {
            break;
        }
    }
    
    None
}

/// Provides access to the DID registry JSON file
pub fn get_did_registry() -> Result<Value, Box<dyn std::error::Error>> {
    // Find the project root
    let project_root = find_project_root()
        .ok_or("Could not find project root")?;
    
    // Construct the path to the registry file
    let registry_path = project_root
        .join("did_library/resources/did_registry.json");
    
    let file_content = fs::read_to_string(&registry_path)?;
    let registry: Value = serde_json::from_str(&file_content)?;
    Ok(registry)
}

// Public function to issue credentials
pub fn issue_credential(request: CredentialRequest) -> Result<VerifiableCredential, String> {

    let option1 = DIDCreationOptions {
        // Set key type to P256 (PS256) for digital signatures
        key_type: Some(KeyType::P256),

        // Populate verification methods with P256 key material
        verification_method: Some(vec![
            VerificationMethod {
                id: format!("#key-{}", uuid::Uuid::new_v4().simple()),
                vm_type: "P256VerificationKey2021".to_string(),
                controller: format!("did:key:{}", uuid::Uuid::new_v4().simple()),
                key_material: KeyMaterial::JWK {
                    public_key_jwk: serde_json::json!({
                    "kty": "EC",
                    "crv": "P-256",
                    "x": base64::encode(&rand::random::<[u8; 32]>()),
                    "y": base64::encode(&rand::random::<[u8; 32]>())
                })
                }
            },
            VerificationMethod {
                id: format!("#key-{}", uuid::Uuid::new_v4().simple()),
                vm_type: "P256VerificationKey2021".to_string(),
                controller: format!("did:key:{}", uuid::Uuid::new_v4().simple()),
                key_material: KeyMaterial::Multibase {
                    public_key_multibase: format!("z12M{}", base64::encode(&rand::random::<[u8; 16]>()))
                }
            }
        ]),

        // Populate authentication methods using P256 key type
        authentication: Some(vec![
            Authentication::Reference(format!("#key-{}", uuid::Uuid::new_v4().simple())),
            Authentication::Embedded(
                VerificationMethod {
                    id: format!("#auth-key-{}", uuid::Uuid::new_v4().simple()),
                    vm_type: "P256VerificationKey2021".to_string(),
                    controller: format!("did:key:{}", uuid::Uuid::new_v4().simple()),
                    key_material: KeyMaterial::JWK {
                        public_key_jwk: serde_json::json!({
                        "kty": "EC",
                        "crv": "P-256",
                        "x": base64::encode(&rand::random::<[u8; 32]>()),
                        "y": base64::encode(&rand::random::<[u8; 32]>())
                    })
                    }
                }
            )
        ]),

        // The remaining fields stay the same
        assertion_method: Some(vec![
            format!("#key-{}", uuid::Uuid::new_v4().simple()),
            format!("#key-{}", uuid::Uuid::new_v4().simple())
        ]),

        key_agreement: Some(vec![
            format!("#key-{}", uuid::Uuid::new_v4().simple()),
            format!("#key-{}", uuid::Uuid::new_v4().simple())
        ]),

        capability_invocation: Some(vec![
            format!("#key-{}", uuid::Uuid::new_v4().simple()),
            format!("#key-{}", uuid::Uuid::new_v4().simple())
        ]),

        capability_delegation: Some(vec![
            format!("#key-{}", uuid::Uuid::new_v4().simple())
        ]),

        service: Some(vec![
            Service {
                id: format!("#service-{}", uuid::Uuid::new_v4().simple()),
                service_type: "DIDCommMessaging".to_string(),
                service_endpoint: Some("https://example.com/endpoint".to_string()),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("priority".to_string(), serde_json::json!(1));
                    props.insert("description".to_string(), serde_json::json!("Secure messaging service"));
                    props.insert("version".to_string(), serde_json::json!("1.0"));
                    props
                }
            },
            Service {
                id: format!("#service-{}", uuid::Uuid::new_v4().simple()),
                service_type: "LinkedDomains".to_string(),
                service_endpoint: Some("https://myidentity.example.org".to_string()),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("domain".to_string(), serde_json::json!("example.org"));
                    props.insert("verified".to_string(), serde_json::json!(true));
                    props
                }
            },
            Service {
                id: format!("#service-{}", uuid::Uuid::new_v4().simple()),
                service_type: "CredentialRegistry".to_string(),
                service_endpoint: Some("https://registry.example.com/credentials".to_string()),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("supportedCredentials".to_string(), serde_json::json!(["VerifiableCredential", "VerifiablePresentation"]));
                    props.insert("accessControl".to_string(), serde_json::json!("permissioned"));
                    props
                }
            }
        ])
    };
    
    let option2 = DIDCreationOptions {
        key_type: None,
        verification_method: None,
        authentication: None,
        assertion_method: None,
        key_agreement: None,
        capability_invocation: None,
        capability_delegation: None,
        service: None,
    };
    let mut did_string = String::new();
    let registry = get_did_registry().unwrap();
    if let Some(obj) = registry.as_object() {
        for (did_key, document) in obj {
            did_string = did_key.to_string();
        }
    }
    // Create a new credential based on the request
    let mut credential = format::create_credential(
        did_string,
        request.subject,
        request.type_,
        request.expiration_date,
        option2
    );
    
    // Sign the credential
    match format::sign_credential(&mut credential) {
        Ok(_) => Ok(credential),
        Err(e) => Err(format!("Failed to sign credential: {}", e)),
    }
}