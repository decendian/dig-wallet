// crates/api_server/src/dto/response.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use shared::dto::DataTransferObject;
pub(crate) use crate::dto::error::ErrorDTO;
use crate::verifiable_credentials::dto::credential::VerifiableCredentialDTO;
use did_library::dto::document::DIDDocumentDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseDTO<T> {
  pub success: bool,
  pub data: Option<T>,
  pub error: Option<ErrorDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDidResponseDTO {
  pub did: String,
  pub document: DIDDocumentDTO,
  pub keys: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCredentialResponseDTO {
  pub credential: VerifiableCredentialDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyCredentialResponseDTO {
  pub is_valid: bool,
  pub verification_results: HashMap<String, VerificationResultDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResultDTO {
  pub check: String,
  pub status: VerificationStatus,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
  #[serde(rename = "success")]
  Success,
  #[serde(rename = "failure")]
  Failure,
  #[serde(rename = "warning")]
  Warning,
}

impl<T: DataTransferObject> DataTransferObject for ApiResponseDTO<T> {
  fn validate(&self) -> Result<(), String> {
    if let Some(data) = &self.data {
      data.validate()?;
    }
    if !self.success && self.error.is_none() {
      return Err("Error field must be present when success is false".to_string());
    }
    Ok(())
  }
}