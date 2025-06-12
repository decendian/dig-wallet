use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use shared::dto::DataTransferObject;

/// Represents a policy data transfer object (DTO) used to define access control policies.
///
/// A policy consists of an ID, name, optional description, a list of resources,
/// a list of actions, optional conditions, and an effect (`Allow` or `Deny`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDTO {
  /// Unique identifier for the policy.
  pub id: String,

  /// Human-readable name for the policy.
  pub name: String,

  /// Optional description of the policy.
  pub description: Option<String>,

  /// List of resources that the policy applies to.
  pub resources: Vec<String>,

  /// List of actions that are permitted or denied by the policy.
  pub actions: Vec<String>,

  /// Optional conditions for the policy, represented as key-value pairs.
  /// The value is a generic JSON value to allow for flexible condition definitions.
  pub conditions: Option<HashMap<String, serde_json::Value>>,

  /// The effect of the policy: either `Allow` or `Deny`.
  pub effect: PolicyEffect,
}

/// Represents the possible effects of a policy: either allowing or denying access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyEffect {
  /// Allow effect: grants access to the specified actions on the resources.
  #[serde(rename = "allow")]
  Allow,

  /// Deny effect: denies access to the specified actions on the resources.
  #[serde(rename = "deny")]
  Deny,
}

impl DataTransferObject for PolicyDTO {
  /// Validates the `PolicyDTO` to ensure required fields are not empty.
  ///
  /// # Returns
  ///
  /// - `Ok(())` if the policy is valid.
  /// - `Err(String)` with a descriptive message if any required field is missing or empty.
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
