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
	
	pub fn decode_key_type(did: &str) -> Result<KeyType, &'static str> {
		if !did.starts_with("did:key:") {
			return Err("Invalid DID: Must start with 'did:key:'");
		}
		let encoded_key = did.replace("did:key:", "");
		
		if(encoded_key.len() < 2){
			return Err("Invalid DID: Must be at least 2 characters long");
		}
		let prefix_bytes: &[u8] = &encoded_key.as_bytes()[..2];
		
		match prefix_bytes {
		[0xed, 0x01] => Ok(KeyType::Ed25519),
		[0xe7, 0x01] => Ok(KeyType::Secp256k1),
		[0x85, 0x24] => Ok(KeyType::RSA),
		_ => Err("Key Type not supported")
		}
	}
}

/// Implementation of DIDMethod trait for KeyDID
impl DIDMethod for KeyDID {
	/**
	* Generates a new did:key DID and returns a new DIDDocument containing the did:key DID
	*/
	fn create_did(&self, options: DIDCreationOptions) -> DIDDocument {
		let did_prefix = String::from("did:key:");
		//TODO: ERROR HANDLING: proper error handling instead of unwrap/expect in production code
		let key_material = JWK::generate_ed25519().expect("Failed to generate JWK");

		// Adjust JWK formatting if needed for the specific DID key method
		let did_key = format!("{}{}", did_prefix, key_material.to_string());

		let mut document = DIDDocument::new(&did_key, options.key_type.unwrap_or(KeyType::Ed25519));

		// Use `if let Some(values)` and `extend` where appropriate

		if (options.verification_method.is_none()) {
			// Create a verification method from the key material
			let method = VerificationMethod {
				id: format!("{}#{}", did_prefix, key_material.to_string()),
				vm_type: "Ed25519VerificationKey2020".to_string(),
				controller: did_key.to_string(),
				key_material: KeyMaterial::Multibase {
					public_key_multibase: key_material.to_string(),
				},
			};
			document.add_verification_method(&vec![method]);
		} else {
			document.add_verification_method(&options.verification_method.unwrap());
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
	fn resolve_did(&self, did: &str, ) -> Result<DIDDocument, &'static str> {
		if !did.starts_with("did:key:") {
			return Err("Invalid DID: Must start with 'did:key:'");
		}
		let key_type = Self::decode_key_type(did);

		match key_type {
			Ok(key_type) => Ok(DIDDocument::new(did, key_type)),
			Err(err) => Err(err),
		}
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