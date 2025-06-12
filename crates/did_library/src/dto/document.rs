// crates/did_library/src/dto/document.rs

use serde::{Deserialize, Serialize};
use crate::did::core::key_utils::KeyType;
use std::collections::HashMap;

use shared::dto::DataTransferObject;
use crate::did::core::did_document::KeyMaterial;

/// Data Transfer Object representing a Decentralized Identifier (DID) Document.
/// This struct is used to serialize and deserialize DID Documents for transport or storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocumentDTO {
    /// The `@context` field in the DID Document, typically a list of context URIs.
    pub context: Vec<String>,

    /// The unique DID identifier.
    pub id: String,

    /// The key type used for the primary verification methods.
    pub key_type: KeyType,

    /// A list of verification methods associated with the DID.
    pub verification_method: Vec<VerificationMethodDTO>,

    /// A list of authentication methods. These can be embedded or referenced.
    pub authentication: Vec<AuthenticationDTO>,

    /// A list of assertion methods supported by this DID.
    pub assertion_method: Vec<String>,

    /// A list of key agreements supported by this DID.
    pub key_agreement: Vec<String>,

    /// A list of capability invocation methods for capability-based access control.
    pub capability_invocation: Vec<String>,

    /// A list of capability delegation methods for capability-based delegation.
    pub capability_delegation: Vec<String>,

    /// A list of services associated with the DID.
    pub service: Vec<ServiceDTO>,
}

/// DTO representing a Verification Method inside the DID Document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethodDTO {
    /// Identifier for the verification method.
    pub id: String,

    /// The type of verification method (e.g., Ed25519VerificationKey2018, JsonWebKey2020, etc.).
    pub vm_type: String,

    /// The controller of the verification method.
    pub controller: String,

    /// The cryptographic key material.
    pub key_material: KeyMaterial,
}

/// DTO representing authentication methods which can be either a reference to
/// a verification method by ID, or an embedded verification method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticationDTO {
    /// Reference to a verification method by its ID.
    Reference(String),

    /// Embedded verification method.
    Embedded(VerificationMethodDTO),
}

/// DTO representing a service entry inside the DID Document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDTO {
    /// Identifier for the service.
    pub id: String,

    /// The type of service (e.g., LinkedDomains, DIDCommMessaging).
    pub service_type: String,

    /// Optional service endpoint URI.
    pub service_endpoint: Option<String>,

    /// Additional properties associated with the service.
    pub properties: HashMap<String, serde_json::Value>,
}

/// DTO representing supported formats of key material inside the verification method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyMaterialDTO {
    /// Key material represented as a JSON Web Key (JWK).
    JWK {
        public_key_jwk: serde_json::Value,
    },

    /// Key material represented using multibase encoding.
    Multibase {
        public_key_multibase: String,
    },

    /// Key material represented as a hexadecimal string.
    Hex {
        public_key_hex: String,
    },
}

/// Implements validation for the DIDDocumentDTO to ensure required fields are populated.
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
