//! DID (Decentralized Identity) Library
//!
//! This library provides tools for creating, managing, and verifying
//! decentralized identifiers across multiple methods.

pub mod did;

// /// Re-export main components for easier access
pub use did::core::did_document::DIDDocument;
// pub use did::methods::{ethr::EthrDID, key::KeyDID, web::WebDID};
// pub use did::registry::Registry;
