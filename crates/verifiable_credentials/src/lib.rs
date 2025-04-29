// This is the main entry point for the crate
pub mod format;
pub mod verifier;
pub mod keys;
pub mod module;

use serde::{Deserialize, Serialize};
use crate::format::VerifiableCredential;

// Re-export important types and functions
#[derive(Serialize, Deserialize, Clone)]
pub struct CredentialRequest {
    pub subject: CredentialSubject,
    pub type_: Vec<String>,
    pub issuer_did: String,
    pub expiration_date: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CredentialSubject {
    pub id: Option<String>,
    pub name: String,
    pub attributes: serde_json::Value,
}

// Public function to issue credentials
pub fn issue_credential(request: CredentialRequest) -> Result<VerifiableCredential, String> {
    // Create a new credential based on the request
    let mut credential = format::create_credential(
        request.issuer_did,
        request.subject,
        request.type_,
        request.expiration_date,
    );
    
    // Sign the credential
    match format::sign_credential(&mut credential) {
        Ok(_) => Ok(credential),
        Err(e) => Err(format!("Failed to sign credential: {}", e)),
    }
}