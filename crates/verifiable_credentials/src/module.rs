// Simply re-export everything from the modules
pub use crate::format::*;
pub use crate::verifier::*;

use serde::{Deserialize, Serialize};
use crate::CredentialSubject; // Use the `CredentialSubject` from `lib.rs`

#[derive(Serialize, Deserialize)]
pub struct CredentialRequest {
    pub subject: CredentialSubject,
    pub type_: Vec<String>,
    pub issuer_did: String,
    pub expiration_date: Option<String>,
}

pub fn issue_credential(request: CredentialRequest) -> Result<VerifiableCredential, String> {
    // Create a new credential based on the request
    let mut credential = create_credential(
        request.issuer_did,
        request.subject,
        request.type_,
        request.expiration_date,
    );

    // Sign the credential
    match sign_credential(&mut credential) {
        Ok(_) => Ok(credential),
        Err(e) => Err(format!("Failed to sign credential: {}", e)),
    }
}