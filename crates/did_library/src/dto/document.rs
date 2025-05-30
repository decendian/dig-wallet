// crates/did_library/src/dto/document.rs

use serde::{Deserialize, Serialize};
use crate::did::core::key_utils::KeyType;
use std::collections::HashMap;

use shared::dto::DataTransferObject;
use crate::did::core::did_document::KeyMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocumentDTO {
    pub context: Vec<String>,
    pub id: String,
    pub key_type: KeyType,
    pub verification_method: Vec<VerificationMethodDTO>,
    pub authentication: Vec<AuthenticationDTO>,
    pub assertion_method: Vec<String>,
    pub key_agreement: Vec<String>,
    pub capability_invocation: Vec<String>,
    pub capability_delegation: Vec<String>,
    pub service: Vec<ServiceDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethodDTO {
    pub id: String,
    pub vm_type: String,
    pub controller: String,
    pub key_material: KeyMaterial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticationDTO {
    Reference(String),
    Embedded(VerificationMethodDTO),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDTO {
    pub id: String,
    pub service_type: String,
    pub service_endpoint: Option<String>,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyMaterialDTO {
    JWK {
        public_key_jwk: serde_json::Value,
    },
    Multibase {
        public_key_multibase: String,
    },
    Hex {
        public_key_hex: String,
    },
}

impl DataTransferObject for DIDDocumentDTO {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("DID identifier cannot be empty".to_string());
        }
        if self.context.is_empty() {
            return Err("Context cannot be empty".to_string());
        }
        Ok(())
    }
}