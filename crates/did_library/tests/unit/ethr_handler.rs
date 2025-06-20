use did_library::did::core::did_document::DIDCreationOptions;
use did_library::did::core::traits::DIDMethod;
use did_library::did::methods::key::handler::KeyDID;
use did_library::did::core::key_utils::KeyType;
use did_library::did::core::did_document::{Authentication, Service, VerificationMethod};
use std::collections::HashMap;

#[test]
fn test_create_did_with_default_options() {
  let options = DIDCreationOptions {
    key_type: None,
    verification_method: None,
    authentication: None,
    assertion_method: None,
    key_agreement: None,
    capability_invocation: None,
    capability_delegation: None,
    service: None,
    network: None,
    chain_id: None,
  };

  let did_doc = KeyDID::create_did(options);

  assert_eq!(did_doc.status, "active");
  assert_eq!(did_doc.key_type, KeyType::Ed25519);
  assert!(!did_doc.verification_method.is_empty());
  assert!(did_doc.service.is_empty());
}

#[test]
fn test_create_did_with_full_options() {
  let vm = VerificationMethod {
    id: String::from("#key1"),
    vm_type: String::from("Ed25519VerificationKey2020"),
    controller: String::from("did:example:123"),
    key_material: did_library::did::core::did_document::KeyMaterial::Multibase {
      public_key_multibase: String::from("z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK")
    },
  };

  let auth = Authentication::Reference(String::from("#key1"));

  let mut service_props = HashMap::new();
  service_props.insert(String::from("priority"), serde_json::json!(1));

  let service = Service {
    id: String::from("#service1"),
    service_type: String::from("ExampleService"),
    service_endpoint: Some(String::from("https://example.com")),
    properties: service_props,
  };

  let options = DIDCreationOptions {
    key_type: Some(KeyType::Ed25519),
    verification_method: Some(vec![vm]),
    authentication: Some(vec![auth]),
    assertion_method: Some(vec![String::from("#key1")]),
    key_agreement: Some(vec![String::from("#key1")]),
    capability_invocation: Some(vec![String::from("#key1")]),
    capability_delegation: Some(vec![String::from("#key1")]),
    service: Some(vec![service]),
    network: None,
    chain_id: None,
  };

  let did_doc = KeyDID::create_did(options);

  assert_eq!(did_doc.status, "active");
  assert_eq!(did_doc.key_type, KeyType::Ed25519);
  assert_eq!(did_doc.verification_method.len(), 1);
  assert_eq!(did_doc.authentication.len(), 1);
  assert_eq!(did_doc.assertion_method.len(), 1);
  assert_eq!(did_doc.key_agreement.len(), 1);
  assert_eq!(did_doc.capability_invocation.len(), 1);
  assert_eq!(did_doc.capability_delegation.len(), 1);
  assert_eq!(did_doc.service.len(), 1);
}