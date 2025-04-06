use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::CredentialSubject;  // Use CredentialSubject from the parent module

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
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

pub fn create_credential(
    issuer_did: String,
    subject: CredentialSubject,
    types: Vec<String>,
    expiration_date: Option<String>,
) -> VerifiableCredential {
    let mut credential_types = vec!["VerifiableCredential".to_string()];
    credential_types.extend(types);
    
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

// Needs work: This function should sign the credential using the issuer's private key
pub fn sign_credential(credential: &mut VerifiableCredential) -> Result<(), String> {
    // In a real implementation, you would:
    // 1. Serialize the credential without the proof
    // 2. Hash the serialized data
    // 3. Sign the hash with the issuer's private key
    // 4. Add the signature to the proof
    
    // This is a simplified example
    let created = Utc::now().to_rfc3339();
    
    credential.proof = Some(Proof {
        type_: "Ed25519Signature2020".to_string(),
        created,
        verification_method: format!("{}#key-1", credential.issuer),
        proof_purpose: "assertionMethod".to_string(),
        proof_value: "zBYw5msGyeBRfMRe7JQLVuMh6xxZJFTN9qFhCVWx5Y2JFBrZSL9CCLFUH9EMkEzGbQy7qZheswmuzEB3TrTJbCDy".to_string(),
    });
    
    Ok(())
}