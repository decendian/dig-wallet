// crates/api_server/src/dto/error.rs
use serde::{Deserialize, Serialize};
use shared::dto::DataTransferObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDTO {
  pub code: String,
  pub message: String,
  pub details: Option<serde_json::Value>,
}

impl DataTransferObject for ErrorDTO {
  fn validate(&self) -> Result<(), String> {
    if self.code.is_empty() {
      return Err("Error code cannot be empty".to_string());
    }
    if self.message.is_empty() {
      return Err("Error message cannot be empty".to_string());
    }
    Ok(())
  }
}