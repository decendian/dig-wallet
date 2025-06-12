// crates/verifiable_credentials/src/dto/credential.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Define a trait that all Data Transfer Objects (DTOs) can implement for basic validation logic
pub trait DataTransferObject {
  // Each DTO implementing this trait must define how it's validated
  fn validate(&self) -> Result<(), String>;
}

// DTO representing the issuer, which can be either a string or a structured object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuerDTO {
  // Issuer represented by a simple string (typically a DID)
  String(String),

  // Issuer represented as a JSON object with optional additional fields
  Object {
    id: String,
    name: Option<String>,

    // Flattened to allow dynamic additional fields
    #[serde(flatten)]
    additional_properties: HashMap<String, serde_json::Value>,
  },
}

// DTO representing the credential subject (the entity the credential refers to)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubjectDTO {
  // Subject identifier (typically a DID)
  pub id: String,

  // Claims about the subject, represented as arbitrary key-value pairs
  #[serde(flatten)]
  pub claims: HashMap<String, serde_json::Value>,
}

// DTO representing the cryptographic proof attached to the credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofDTO {
  // The type of proof (e.g., "Ed25519Signature2018")
  pub type_: String,

  // Timestamp when the proof was created
  pub created: DateTime<Utc>,

  // The verification method (public key reference)
  pub verification_method: String,

  // The intended purpose of the proof (e.g., "assertionMethod")
  pub proof_purpose: String,

  // The actual cryptographic signature or proof value
  pub proof_value: String,
}

// Implement the DataTransferObject trait for VerifiableCredentialDTO
impl DataTransferObject for VerifiableCredentialDTO {
  fn validate(&self) -> Result<(), String> {
    // Simple validations for required fields
    if self.id.is_empty() {
      return Err("Credential ID cannot be empty".to_string());
    }
    if self.context.is_empty() {
      return Err("Context cannot be empty".to_string());
    }
    if self.type_.is_empty() {
      return Err("Type cannot be empty".to_string());
    }
    Ok(())
  }
}