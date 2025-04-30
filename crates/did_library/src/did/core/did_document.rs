//! DID Document implementation
//!
//! A DID Document contains information associated with a DID,
//! including public keys, authentication methods, and services.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a W3C compliant DID Document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    /// The DID that the document is about
    pub id: String,

    /// Controller DIDs that have authority over this DID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<String>>,
    
    /// Public keys associated with this DID
    pub verification_method: Vec<VerificationMethod>,
    
    /// Authentication methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authentication: Vec<Authentication>,

    /// Assertion methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub assertion_method: Vec<String>,

    /// Key agreement methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub key_agreement: Vec<String>,

    /// Service endpoints associated with this DID
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub service: Vec<Service>,
}

/// Verification Method representing a public key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    /// ID of the verification method
    pub id: String,

    /// Type of the verification method
    #[serde(rename = "type")]
    pub vm_type: String,

    /// The DID that controls this verification method
    pub controller: String,

    /// Public key material, could be JWK, Multibase, etc.
    #[serde(flatten)]
    pub key_material: KeyMaterial,
}

/// Authentication method, can be a reference or embedded
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Authentication {
    /// Reference to a verification method by ID
    Reference(String),
    /// Embedded verification method
    Embedded(VerificationMethod),
}

/// Different formats of key material
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyMaterial {
    /// JSON Web Key
    JWK {
        /// JWK data
        #[serde(rename = "publicKeyJwk")]
        public_key_jwk: serde_json::Value,
    },
    /// Multibase encoded key
    Multibase {
        /// Multibase data
        #[serde(rename = "publicKeyMultibase")]
        public_key_multibase: String,
    },
    /// Hex encoded key (mostly for Ethereum)
    Hex {
        /// Hex data
        #[serde(rename = "publicKeyHex")]
        public_key_hex: String,
    },
}

/// Service endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// ID of the service
    pub id: String,

    /// Type of service
    #[serde(rename = "type")]
    pub service_type: String,

    /// Endpoint URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,

    /// Additional properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

impl DIDDocument {
    /// Create a new DID Document
    pub fn new(id: &str) -> Self {
        DIDDocument {
            id: id.to_string(),
            controller: None,
            verification_method: Vec::new(),
            authentication: Vec::new(),
            assertion_method: Vec::new(),
            key_agreement: Vec::new(),
            service: Vec::new(),
        }
    }

    /// Add a verification method to the document
    pub fn add_verification_method(&mut self, method: VerificationMethod) {
        self.verification_method.push(method)
    }

    /// Add an authentication method
    pub fn add_authentication(&mut self, auth: Authentication) {
        self.authentication.push(auth);
    }

    /// Add a service to the document
    pub fn add_service(&mut self, service: Service) {
        self.service.push(service);
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // serde_json::to_string_pretty(self)
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}