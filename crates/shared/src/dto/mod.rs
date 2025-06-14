use serde::{Deserialize, Serialize};

/// Base trait for all DTOs
pub trait DataTransferObject: Serialize + Deserialize<'static> + Clone {
  fn validate(&self) -> Result<(), String> {
    Ok(())
  }
}
