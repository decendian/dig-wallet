//! Main module for the DID library
//!
//! This module serves as the entry point for the DID library,
//! organizing submodules for core functionality, DID methods, and registry.

pub mod core;
pub mod methods;
pub mod registry;

/// DID URI format according to W3C specification
pub const DID_URI_FORMAT: &str = "did:<method-name>:<method-specific-id>";

/// The DID trait defines common functionality across all DID types
pub trait DID {
    /// Returns the DID method name
    fn method(&self) -> &str;

    /// Returns the method-specific identifier
    fn id(&self) -> &str;

    /// Converts the DID to its string representation
    fn to_string(&self) -> String {
        format!("did:{}:{}", self.method(), self.id())
    }

    /// Resolves the DID document associated with this DID
    fn resolve(&self) -> Result<crate::DIDDocument, Box<dyn std::error::Error>>;
}