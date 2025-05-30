// crates/verifiable_credentials/src/dto/credential.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Either use an absolute path
// Define the trait directly in this file
pub trait DataTransferObject {
    fn validate(&self) -> Result<(), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredentialDTO {
  #[serde(rename = "@context")]
  pub context: Vec<String>,
  pub id: String,
  pub type_: Vec<String>,
  pub issuer: IssuerDTO,
  pub issuance_date: DateTime<Utc>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiration_date: Option<DateTime<Utc>>,
  pub credential_subject: CredentialSubjectDTO,
  pub proof: Option<ProofDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuerDTO {
  String(String),
  Object {
    id: String,
    name: Option<String>,
    #[serde(flatten)]
    additional_properties: HashMap<String, serde_json::Value>,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubjectDTO {
  pub id: String,
  #[serde(flatten)]
  pub claims: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofDTO {
  pub type_: String,
  pub created: DateTime<Utc>,
  pub verification_method: String,
  pub proof_purpose: String,
  pub proof_value: String,
}

impl DataTransferObject for VerifiableCredentialDTO {
  fn validate(&self) -> Result<(), String> {
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