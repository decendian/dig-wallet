use std::ops::Add;
use ssi::JWK;
use crate::did::core::did_document::{DIDDocument, VerificationMethod, KeyMaterial};
use crate::did::core::traits::DIDMethod;

pub struct KeyDID;

/// Helpers specific to the Key DID method.
impl KeyDID {
	pub fn new() -> Self {
		Self {}
	}
}

/// Implementation of DIDMethod trait for KeyDID
#[allow(unused_variables)]
impl DIDMethod for KeyDID {
	/**
	* Generates a new did:key DID and returns a new DIDDocument containing the did:key DID
	*/
	fn create_did() -> DIDDocument {
		let did_prefix = String::from("did:key:");
		let jwk = JWK::generate_ed25519().unwrap();
		let did_key = format!("{}{}", did_prefix, jwk.to_string());
		DIDDocument::new(&did_key)
	}

	/*
	* Retrieves the current DID Document for an existing DID
	*/
	fn resolve_did(did: &str) -> Result<DIDDocument, &'static str> {
		if !did.starts_with("did:key:") {
			return Err("Invalid DID: Must start with 'did:key:'");
		}

		// For did:key method, resolution is deterministic based on the key
		// The DID Document is constructed from the key material in the DID
		let mut doc = DIDDocument::new(did);

		// Extract the key material from the DID
		let key_material = did.strip_prefix("did:key:").unwrap();

		// Create verification method from the key material
		let method = VerificationMethod {
			id: format!("{}#{}", did, key_material),
			vm_type: "Ed25519VerificationKey2020".to_string(),
			controller: did.to_string(),
			key_material: KeyMaterial::Multibase {
				public_key_multibase: key_material.to_string(),
			},
		};

		doc.add_verification_method(method);
		Ok(doc)
	}

	/*
	* Modifies an existing DID Document (adding keys, rotating keys, changing services, etc.).
	 */
	fn update_did(did: &str, verification_method: Option<VerificationMethod>) -> Result<String, &'static str> {
		if !did.starts_with("did:key:") {
			return Err("Invalid DID: Must start with 'did:key:'");
		}

		let mut did_document = DIDDocument::new(did);

		if let Some(method) = verification_method {
			did_document.add_verification_method(method);
		} else {
			let jwk = JWK::generate_ed25519().unwrap();
			let method = VerificationMethod {
				id: format!("{}#{}", did_document.id, jwk.to_string()),
				vm_type: "Ed25519VerificationKey2020".to_string(),
				controller: did_document.id.clone(),
				key_material: KeyMaterial::JWK {
					public_key_jwk: serde_json::to_value(jwk).unwrap(),
				},
			};
			did_document.add_verification_method(method);
		}

		Ok(did_document.to_json().unwrap_or_else(|_| String::from("{}")))
	}
}