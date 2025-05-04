use crate::did::core::did_document::{DIDDocument, VerificationMethod, KeyMaterial, DIDCreationOptions};
use crate::did::core::key_utils::KeyType;
use crate::did::core::traits::DIDMethod;
use ssi::jwk::JWK;


pub struct KeyDID;

fn format_did_key(jwk: &serde_json::Value) -> Result<String, &'static str> {
	// First, determine key type from the JWK
	let key_type = match jwk.get("crv").and_then(|v| v.as_str()) {
		Some("Ed25519") => "ed25519",
		Some("P-256") => "p256",
		Some("secp256k1") => "secp256k1",
		_ => return Err("Unsupported or missing curve type in JWK"),
	};

	// Initialize key bytes
	let mut key_bytes = Vec::new();

	match key_type {
		"ed25519" => {
			// Ed25519 keys - multicodec prefix 0xed01
			key_bytes.push(0xed);
			key_bytes.push(0x01);

			// Get the x coordinate (public key)
			if let Some(x) = jwk.get("x").and_then(|v| v.as_str()) {
				let decoded = base64::decode(x).map_err(|_| "Invalid base64 in Ed25519 key")?;
				key_bytes.extend_from_slice(&decoded);
			} else {
				return Err("Missing x coordinate in Ed25519 key");
			}
		},
		"p256" => {
			// P-256 keys - multicodec prefix 0x1200
			key_bytes.push(0x12);
			key_bytes.push(0x00);

			// Get x and y coordinates
			let x = jwk.get("x").and_then(|v| v.as_str())
				.ok_or("Missing x coordinate in P-256 key")?;
			let y = jwk.get("y").and_then(|v| v.as_str())
				.ok_or("Missing y coordinate in P-256 key")?;

			// Decode x and y
			let x_bytes = base64::decode(x).map_err(|_| "Invalid base64 in x coordinate")?;
			let y_bytes = base64::decode(y).map_err(|_| "Invalid base64 in y coordinate")?;

			// Determine if y is odd or even for compressed format
			let is_y_odd = y_bytes.last().map(|b| b & 1 == 1).unwrap_or(false);
			let prefix = if is_y_odd { 0x03 } else { 0x02 };

			// Add format prefix and x coordinate
			key_bytes.push(prefix);
			key_bytes.extend_from_slice(&x_bytes);
		},
		"secp256k1" => {
			// secp256k1 keys - multicodec prefix 0xe701
			key_bytes.push(0xe7);
			key_bytes.push(0x01);

			// Get x and y coordinates
			let x = jwk.get("x").and_then(|v| v.as_str())
				.ok_or("Missing x coordinate in secp256k1 key")?;
			let y = jwk.get("y").and_then(|v| v.as_str())
				.ok_or("Missing y coordinate in secp256k1 key")?;

			// Decode x and y
			let x_bytes = base64::decode(x).map_err(|_| "Invalid base64 in x coordinate")?;
			let y_bytes = base64::decode(y).map_err(|_| "Invalid base64 in y coordinate")?;

			// Determine if y is odd or even for compressed format
			let is_y_odd = y_bytes.last().map(|b| b & 1 == 1).unwrap_or(false);
			let prefix = if is_y_odd { 0x03 } else { 0x02 };

			// Add format prefix and x coordinate
			key_bytes.push(prefix);
			key_bytes.extend_from_slice(&x_bytes);
		},
		_ => return Err("Unsupported key type"),
	};

	// Convert to Base58
	// Note: You'll need to add bs58 to your dependencies
	let encoded = bs58::encode(key_bytes).into_string();

	Ok(format!("did:key:{}", encoded))
}



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
		
		if encoded_key.len() < 2 {
			return Err("Invalid DID: Must be at least 2 characters long");
		}
		let prefix_bytes: &[u8] = &encoded_key.as_bytes()[..2];
		
		match prefix_bytes {
		[0xed, 0x01] => Ok(KeyType::Ed25519),
		[0xe7, 0x01] => Ok(KeyType::Secp256k1),
		[0x12, 0x00] => Ok(KeyType::P256),
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
		
		let key_type = options.key_type.unwrap_or(KeyType::Ed25519);

		let jwk_string = match key_type {
			KeyType::Ed25519 => JWK::generate_ed25519().expect("Failed to generate Ed25519 key"),
			KeyType::Secp256k1 => JWK::generate_secp256k1(),
			KeyType::P256 => JWK::generate_p256(),
		}.to_string();

		// let jwk = serde_json::from_str(&jwk_string).unwrap();
		// println!("{:?}", jwk);
		// let did_key = format_did_key(&jwk).unwrap();
		
		let key_material = did_prefix + uuid::Uuid::new_v4().simple().to_string().as_str();
		
		let mut document = DIDDocument::new(key_material.as_str(), key_type);
		

		// Use `if let Some(values)` and `extend` where appropriate
		if options.verification_method.is_none() {
			// Create a verification method from the key material

			let vm_type: &str = match key_type {
				KeyType::Ed25519 => "Ed25519VerificationKey2020",
				KeyType::Secp256k1 =>"EcdsaSecp256k1VerificationKey2019", 
				KeyType::P256 => "P256VerificationKey2021", 
			};
			
			let method = VerificationMethod {
				id: format!("#{}", key_material.to_string()), 
				vm_type: vm_type.to_string(),
				controller: key_material.to_string(),
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
	fn resolve_did(&self, did: &str) -> Result<DIDDocument, &'static str> {
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