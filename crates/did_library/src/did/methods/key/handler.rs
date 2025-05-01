use ssi::JWK;
use crate::did::core::did_document::{DIDDocument, VerificationMethod, KeyMaterial, DIDCreationOptions};
use crate::did::core::key_utils::KeyType;
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
	fn create_did(&self, options: DIDCreationOptions) -> DIDDocument {
		let did_prefix = String::from("did:key:");
		//TODO: ERROR HANDLING: proper error handling instead of unwrap/expect in production code
		let jwk = JWK::generate_ed25519().expect("Failed to generate JWK");
		
		// Adjust JWK formatting if needed for the specific DID key method
		let did_key = format!("{}{}", did_prefix, jwk.to_string());

		let mut document = DIDDocument::new(&did_key, KeyType::Ed25519);

		// Use `if let Some(values)` and `extend` where appropriate

		if let Some(methods) = options.verification_method {
			document.add_verification_method(&methods);
		}

		if let Some(auths) = options.authentication {
			document.add_authentication(&auths);
		}

		if let Some(methods) = options.assertion_method {
			document.add_assertion_method(&methods);
		}

		if let Some(keys) = options.key_agreement {
			document.add_key_agreement(&keys);
		}

		if let Some(keys) = options.capability_invocation {
			document.add_capability_invocation(&keys)
		}
		
		if let Some(keys) = options.capability_delegation {
			document.add_capability_delegation(&keys)
		}

		if let Some(services) = options.service {
			document.add_service(&services);
		}
		
		document
	}
	/*
	* Retrieves the current DID Document for an existing DID
	*/
fn resolve_did(&self, did: &str) -> Result<DIDDocument, &'static str> {
	if !did.starts_with("did:key:") {
		return Err("Invalid DID: Must start with 'did:key:'");
	}

	// For did:key method, resolution is deterministic based on the key
	// The DID Document is constructed from the key material in the DID
	let mut doc = DIDDocument::new(did, KeyType::Ed25519);

	// Extract the key material from the DID
	let key_material = did.strip_prefix("did:key:").unwrap();

	// Create a verification method from the key material
	let method = VerificationMethod {
		id: format!("{}#{}", did, key_material),
		vm_type: "Ed25519VerificationKey2020".to_string(),
		controller: did.to_string(),
		key_material: KeyMaterial::Multibase {
			public_key_multibase: key_material.to_string(),
		},
	};

	doc.add_verification_method(&vec![method]);
	Ok(doc)
}

	/*
	* Modifies an existing DID Document (adding keys, rotating keys, changing services, etc.).
	 */
	fn update_did(&self, did: &str, options: DIDCreationOptions) -> Result<DIDDocument, &'static str> {
		// Validate DID format
		if !did.starts_with("did:key:") {
			return Err("Invalid DID: Must start with 'did:key:'");
		}

		// Resolve existing document
		let mut document = self.resolve_did(did)?;

		if let Some(methods) = options.verification_method {
			document.add_verification_method(&methods);
		}

		if let Some(auths) = options.authentication {
			document.add_authentication(&auths);
		}

		if let Some(methods) = options.assertion_method {
			document.add_assertion_method(&methods);
		}

		if let Some(keys) = options.key_agreement {
			document.add_key_agreement(&keys);
		}

		if let Some(keys) = options.capability_invocation {
			document.add_capability_invocation(&keys)
		}

		if let Some(keys) = options.capability_delegation {
			document.add_capability_delegation(&keys)
		}

		if let Some(services) = options.service {
			document.add_service(&services);
		}
		
		Ok(document)
	}
}