use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs;
use std::path::Path;
use super::CredentialSubject;  // Use CredentialSubject from the parent module
use super::keys;

/// Path to the resources directory that stores local copies of context files
/// This points one directory up from the crate to access the shared resources folder
const RESOURCES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../resources");

/// Represents a W3C Verifiable Credential
/// 
/// This implementation follows the W3C Verifiable Credentials Data Model 1.0
/// specification (https://www.w3.org/TR/vc-data-model/)
/// 
/// Local copies of the context files are stored in the resources directory
/// to avoid external network dependencies.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    pub type_: Vec<String>,
    pub issuer: String,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: String,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: CredentialSubject,
    pub proof: Option<Proof>,
}

/// Cryptographic proof information attached to a Verifiable Credential
/// 
/// Contains the signature metadata and value used to verify the authenticity
/// of the credential.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Proof {
    pub type_: String,
    pub created: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,
    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,
    #[serde(rename = "proofValue")]
    pub proof_value: String,
}

/// Maps standard W3C credential context URLs to local file paths
/// 
/// This function translates the standard context URLs used in Verifiable Credentials
/// to the paths of local copies stored in the resources directory.
/// 
/// # Arguments
/// * `url` - The context URL to map to a local file
/// 
/// # Returns
/// * `Some(String)` - The full path to the local file if a mapping exists
fn get_local_context_path(url: &str) -> Option<String> {
    match url {
        "https://www.w3.org/2018/credentials/v1" => Some("credentials-v1.jsonld"),
        "https://www.w3.org/2018/credentials/examples/v1" => Some("credentials-examples-v1.jsonld"),
        // Add more mappings as needed
        _ => None,
    }.map(|filename| Path::new(RESOURCES_PATH).join(filename).to_string_lossy().to_string())
}


/// Creates a new Verifiable Credential with the provided information
/// 
/// This function initializes a credential with the standard W3C context URLs,
/// generates a UUID for the credential ID, and sets up the basic credential fields.
/// The created credential is not yet signed (proof is None).
/// 
/// # Arguments
/// * `issuer_did` - The DID (Decentralized Identifier) of the credential issuer
/// * `subject` - The credential subject containing claims about an entity
/// * `types` - Additional credential types beyond the base "VerifiableCredential" type
/// * `expiration_date` - Optional expiration date in RFC3339 format
/// 
/// # Returns
/// A new unsigned Verifiable Credential
pub fn create_credential(
    issuer_did: String,
    subject: CredentialSubject,
    types: Vec<String>,
    expiration_date: Option<String>,
) -> VerifiableCredential {
    let mut credential_types = vec!["VerifiableCredential".to_string()];
    credential_types.extend(types);
    
    // We still use the URL strings as identifiers for compatibility
    // but we're now ready to use local files if needed
    VerifiableCredential {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_string(),
            "https://www.w3.org/2018/credentials/examples/v1".to_string(),
        ],
        id: format!("urn:uuid:{}", Uuid::new_v4()),
        type_: credential_types,
        issuer: issuer_did.clone(),
        issuance_date: Utc::now().to_rfc3339(),
        expiration_date,
        credential_subject: subject,
        proof: None,
    }
}

/// Loads the content of a context file given its standard URL
/// 
/// This function retrieves the content of a locally stored context file
/// instead of fetching it from the web. This makes the application more
/// reliable and removes external network dependencies.
/// 
/// # Arguments
/// * `url` - The standard context URL corresponding to the local file
/// 
/// # Returns
/// * `Ok(String)` - The content of the context file if found
/// * `Err(String)` - An error message if the file cannot be loaded
pub fn load_context_content(url: &str) -> Result<String, String> {
    if let Some(path) = get_local_context_path(url) {
        fs::read_to_string(path)
            .map_err(|e| format!("Failed to load context file for {}: {}", url, e))
    } else {
        Err(format!("No local context file mapping for URL: {}", url))
    }
}


/// Signs a Verifiable Credential using the issuer's key
/// 
/// This function adds a cryptographic proof to a credential by:
/// 1. Creating a copy of the credential without the proof
/// 2. Converting it to a canonical JSON representation
/// 3. Signing the canonical form with the issuer's private key
/// 4. Adding the resulting signature and metadata as a proof
/// 
/// # Arguments
/// * `credential` - The credential to sign (modified in-place)
/// 
/// # Returns
/// * `Ok(())` - If signing was successful
/// * `Err(String)` - If an error occurred during signing
pub fn sign_credential(credential: &mut VerifiableCredential) -> Result<(), String> {
    // Step 1: Create a copy without the proof
    let mut credential_for_signing = credential.clone();
    credential_for_signing.proof = None;
    
    // Step 2: Canonicalize (deterministic serialization)
    let canonical = serde_json::to_string(&credential_for_signing)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    // Step 3: Get the demo keypair (in production, you'd retrieve the right key)
    let keypair = keys::get_demo_keypair();
    
    // Step 4: Sign the canonicalized data using base64 encoding
    let signature = keys::sign_data(canonical.as_bytes(), &keypair);
    
    // Step 5: Add the proof
    let created = Utc::now().to_rfc3339();
    let verification_method = format!("{}#key-1", credential.issuer);
    
    credential.proof = Some(Proof {
        type_: "Ed25519Signature2020".to_string(),
        created,
        verification_method,
        proof_purpose: "assertionMethod".to_string(),
        proof_value: signature,
    });
    
    Ok(())
}