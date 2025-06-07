//! DID Document implementation
//!
//! A DID Document contains information associated with a DID,
//! including public keys, authentication methods, and services.

use crate::did::core::key_utils::KeyType;
use chrono::{Months, Utc};
use serde::{Deserialize, Serialize};
use ssi::claims::chrono;
use ssi::xsd::Datatype::Date;
use std::collections::HashMap;

/// A structure representing a Decentralized Identifier (DID) Document.
/// This DID Document conforms to the DID specification highlighted by W3C standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,

    /// The DID that the document is about
    pub id: String,

    pub status: String,
    
    /// The type of key signature algorithm used for creating did method
    #[serde(rename = "@type")]
    pub key_type: KeyType,

    /// Assertion methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub assertion_method: Vec<String>,

    /// Authentication methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authentication: Vec<Authentication>,

    /// Part of the guardianship module:
    /// A list of verification method references which give the ability to modify the did document.
    /// This Includes adding or removing keys, service endpoints, and controllers
    pub capability_invocation: Vec<String>,

    /// Part of the guardianship module:
    /// A list of verification method references that can be used
    /// by the DID subject to delegate authorization to others. This allows
    /// the DID subject to enable others to act with some subset of the subject's
    /// capabilities on their behalf.
    pub capability_delegation: Vec<String>,

    /// Key agreement methods (references to verification methods)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub key_agreement: Vec<String>,

    /// Service endpoints associated with this DID
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub service: Vec<Service>,

    /// Public keys associated with this DID
    pub verification_method: Vec<VerificationMethod>,

}

pub struct DIDCreationOptions {
    pub key_type: Option<KeyType>,
    pub verification_method: Option<Vec<VerificationMethod>>,
    pub authentication: Option<Vec<Authentication>>,
    pub assertion_method: Option<Vec<String>>,
    pub key_agreement: Option<Vec<String>>,
    pub capability_invocation: Option<Vec<String>>,
    pub capability_delegation: Option<Vec<String>>,
    pub service: Option<Vec<Service>>,
    pub network: Option<String>,
    pub chain_id: Option<String>,
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

pub struct AssertionMethod {
    pub assertion_method: Vec<String>,
}

pub struct KeyAgreement {
    pub key_agreement: Vec<String>,
}

pub struct CapabilityInvocation {
    pub capability_invocation: Vec<String>,
}

pub struct CapabilityDelegation {
    pub capability_delegation: Vec<String>,
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
        #[serde(rename = "public_key_multibase")]
        public_key_multibase: String,
    },
    /// Hex encoded key (mostly for Ethereum)
    Hex {
        /// Hex data
        #[serde(rename = "publicKeyHex")]
        public_key_hex: String,
    },
}

impl DIDDocument {
    /// Create a new DID Document
    pub fn new(id: &str, key_type: KeyType) -> Self {
        DIDDocument {
            context: vec!["https://www.w3.org/ns/did/v1".to_string()],
            id: id.to_string(),
            status: "active".to_string(),
            key_type,
            verification_method: Vec::new(),
            authentication: Vec::new(),
            assertion_method: Vec::new(),
            key_agreement: Vec::new(),
            capability_invocation: Vec::new(),
            capability_delegation: Vec::new(),
            service: Vec::new(),
        }
    }

    /// Add a verification method to the document
    pub fn add_verification_method(&mut self, methods: &Vec<VerificationMethod>) {
        self.verification_method.extend_from_slice(methods);
    }

    /// Add an authentication method
    pub fn add_authentication(&mut self, auth: &Vec<Authentication>) {
        self.authentication.extend_from_slice(auth)
    }

    pub fn add_assertion_method(&mut self, keys: &Vec<String>) {
        self.assertion_method.extend_from_slice(keys)
    }
    
    pub fn add_key_agreement(&mut self, keys: &Vec<String>) {
        self.key_agreement.extend_from_slice(keys)
    }

    pub fn add_capability_invocation(&mut self, keys: &Vec<String>) {
        self.capability_invocation.extend_from_slice(keys)
    }

    pub fn add_capability_delegation(&mut self, keys: &Vec<String>) {
        self.capability_delegation.extend_from_slice(keys)
    }
    
    /// Add a service to the document
    pub fn add_service(&mut self, services: &Vec<Service>) {
        self.service.extend_from_slice(services);
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}