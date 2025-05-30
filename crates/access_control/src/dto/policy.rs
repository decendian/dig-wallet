
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use shared::dto::DataTransferObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDTO {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub resources: Vec<String>,
  pub actions: Vec<String>,
  pub conditions: Option<HashMap<String, serde_json::Value>>,
  pub effect: PolicyEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyEffect {
  #[serde(rename = "allow")]
  Allow,
  #[serde(rename = "deny")]
  Deny,
}

impl DataTransferObject for PolicyDTO {
  fn validate(&self) -> Result<(), String> {
    if self.id.is_empty() {
      return Err("Policy ID cannot be empty".to_string());
    }
    if self.name.is_empty() {
      return Err("Policy name cannot be empty".to_string());
    }
    if self.resources.is_empty() {
      return Err("Resources cannot be empty".to_string());
    }
    if self.actions.is_empty() {
      return Err("Actions cannot be empty".to_string());
    }
    Ok(())
  }
}