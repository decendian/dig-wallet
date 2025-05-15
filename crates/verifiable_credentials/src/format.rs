use chrono::{Months, Utc};
use serde::{Deserialize, Serialize};
// use uuid::Uuid; // Uncomment if you need UUID generation
use std::fs;
use std::path::Path;
// use sha2::digest::generic_array::typenum::op; // Uncomment if you need SHA2 hashing
// use did_library::did::methods::key::handler; // Uncomment if you need DID methods
use did_library::did::core::did_document::{Authentication, DIDCreationOptions, Service, VerificationMethod};
use did_library::did::core::key_utils::KeyType;
use did_library::did::core::traits::DIDMethod;
use did_library::did::methods::key::handler::KeyDID;
use super::CredentialSubject;  // Use CredentialSubject from the parent module
use super::keys;

/// Path to the resources directory that stores local copies of context files
/// This points one directory up from the crate to access the shared resources folder
const RESOURCES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources");

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
    /// The type of key signature algorithm used for creating did method
    #[serde(rename = "type")]
    pub key_type: KeyType,
    /// Assertion methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub assertion_method: Vec<String>,
    /// Authentication methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authentication: Vec<Authentication>,
    pub capability_invocation: Vec<String>,
    pub capability_delegation: Vec<String>,
    pub key_agreement: Vec<String>,
    pub verification_method: Vec<VerificationMethod>,
    pub service: Vec<Service>,
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
    options: DIDCreationOptions
) -> VerifiableCredential {

    let now = Utc::now();
    // Format current time for issuanceDate (RFC3339 format)
    let formatted_issuance_date = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true); // `true` adds the 'Z' for UTC

    // Calculate expiration date (6 months from now)
    // This uses checked_add_months which requires the `chrono-months` crate or similar functionality.
    let future_datetime = now.checked_add_months(Months::new(6))
      .expect("Failed to add 6 months to current date. Ensure date is valid."); // Consider more robust error handling

    // Format future time for expirationDate (RFC3339 format)
    let formatted_expiration_date = future_datetime.to_rfc3339_opts(chrono::SecondsFormat::Secs, true); // `true` adds the 'Z' for UTC
    let mut credential_types = vec!["VerifiableCredential".to_string()];
    credential_types.extend(types);

    // We can append this later as our definitions become defined
    // context: vec![
    //
    //     "https://www.w3.org/2018/credentials/v1".to_string(),
    //     "https://www.w3.org/2018/credentials/examples/v1".to_string(),
    // ],

    let key_did = KeyDID::new().create_did(options);

    // Make it based it on the did_registry.json created file
    VerifiableCredential {
        context: key_did.context,
        id: issuer_did.clone(),
        key_type: key_did.key_type,
        authentication: key_did.authentication,
        assertion_method: key_did.assertion_method,
        capability_invocation: key_did.capability_invocation,
        capability_delegation: key_did.capability_delegation,
        key_agreement: key_did.key_agreement,
        //TODO: Tie verification_method to the did_registry.json did
        verification_method: key_did.verification_method,
        service: key_did.service,
        type_: credential_types,
        issuer: issuer_did.clone(),
        issuance_date: formatted_issuance_date,
        expiration_date: expiration_date.or(Some(formatted_expiration_date)),
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