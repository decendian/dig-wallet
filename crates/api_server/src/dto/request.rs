// crates/api_server/src/dto/request.rs

use std::collections::HashMap;
use std::future::{ready, Ready};
use actix_web::{FromRequest, HttpRequest, error::Error as ActixError};
use actix_web::dev::Payload;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use shared::dto::DataTransferObject;
use verifiable_credentials::dto::credential::VerifiableCredentialDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct CreateDidRequestDTO {
  pub method: String,
  pub key_type: Option<String>,
  pub options: Option<HashMap<String, serde_json::Value>>,
}

//TODO: Not supported in the frontend, uncomment when it is
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ResolveDidRequestDTO {
//   pub did: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCredentialRequestDTO {
  pub credential_type: Vec<String>,
  pub issuer: String,
  pub subject: String,
  pub claims: HashMap<String, serde_json::Value>,
  pub expiration_days: Option<u32>,
}

// Validation =============================================================================================

impl DataTransferObject for CreateDidRequestDTO {
  fn validate(&self) -> Result<(), String> {
    if self.method.is_empty() {
      return Err("DID method cannot be empty".to_string());
    }
    Ok(())
  }
}

impl FromRequest for CreateDidRequestDTO {
  type Error = ActixError;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
    todo!()
  }
}



//TODO: Not supported in the frontend, uncomment when it is
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VerifyCredentialRequestDTO {
//   pub credential: VerifiableCredentialDTO,
// }

//TODO: Not supported in the frontend, uncomment when it is
// impl DataTransferObject for ResolveDidRequestDTO {
//   fn validate(&self) -> Result<(), String> {
//     if self.did.is_empty() {
//       return Err("DID cannot be empty".to_string());
//     }
//     if !self.did.starts_with("did:") {
//       return Err("Invalid DID format, must start with 'did:'".to_string());
//     }
//     Ok(())
//   }
// }

impl DataTransferObject for IssueCredentialRequestDTO {
  fn validate(&self) -> Result<(), String> {
    if self.credential_type.is_empty() {
      return Err("Credential type cannot be empty".to_string());
    }
    if self.issuer.is_empty() {
      return Err("Issuer cannot be empty".to_string());
    }
    if self.subject.is_empty() {
      return Err("Subject cannot be empty".to_string());
    }
    if self.claims.is_empty() {
      return Err("Claims cannot be empty".to_string());
    }
    Ok(())
  }
}

//TODO: Not supported in the frontend, uncomment when it is
// impl DataTransferObject for VerifyCredentialRequestDTO {
//   fn validate(&self) -> Result<(), String> {
//     Ok(())
//   }
// }