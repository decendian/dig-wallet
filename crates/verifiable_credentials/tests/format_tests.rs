// tests/format_tests.rs
use verifiable_credentials::format::*;
use verifiable_credentials::*;
use did_library::did::core::did_document::DIDCreationOptions;

#[cfg(test)]
mod format_tests {
    use super::*;

    fn create_test_credential_subject() -> CredentialSubject {
        CredentialSubject {
            id: Some("did:example:student".to_string()),
            name: "Alice Smith".to_string(),
            attributes: serde_json::json!({
                "degree": "Bachelor of Science",
                "university": "Example University"
            }),
        }
    }

    fn create_test_options() -> DIDCreationOptions {
        DIDCreationOptions {
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
        }
    }

    #[test]
    fn test_create_credential_basic() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec!["UniversityDegree".to_string()];
        let options = create_test_options();
        
        let credential = create_credential(
            issuer_did.clone(),
            subject.clone(),
            types.clone(),
            None,
            options,
        );
        
        assert_eq!(credential.issuer, issuer_did);
        assert_eq!(credential.id, issuer_did);
        assert!(credential.type_.contains(&"VerifiableCredential".to_string()));
        assert!(credential.type_.contains(&"UniversityDegree".to_string()));
        assert_eq!(credential.credential_subject.name, subject.name);
        assert!(credential.proof.is_none());
    }

    #[test]
    fn test_create_credential_with_expiration() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec!["UniversityDegree".to_string()];
        let custom_expiration = "2025-12-31T23:59:59Z".to_string();
        let options = create_test_options();
        
        let credential = create_credential(
            issuer_did,
            subject,
            types,
            Some(custom_expiration.clone()),
            options,
        );
        
        assert_eq!(credential.expiration_date, Some(custom_expiration));
    }

    #[test]
    fn test_create_credential_multiple_types() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec![
            "UniversityDegree".to_string(),
            "BachelorDegree".to_string(),
        ];
        let options = create_test_options();
        
        let credential = create_credential(
            issuer_did,
            subject,
            types.clone(),
            None,
            options,
        );
        
        assert_eq!(credential.type_.len(), 3); // 2 custom + VerifiableCredential
        assert!(credential.type_.contains(&"VerifiableCredential".to_string()));
        for type_ in &types {
            assert!(credential.type_.contains(type_));
        }
    }

    #[test]
    fn test_sign_credential_success() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec!["UniversityDegree".to_string()];
        let options = create_test_options();
        
        let mut credential = create_credential(
            issuer_did.clone(),
            subject,
            types,
            None,
            options,
        );
        
        let result = sign_credential(&mut credential);
        
        assert!(result.is_ok());
        assert!(credential.proof.is_some());
        
        let proof = credential.proof.unwrap();
        assert_eq!(proof.type_, "Ed25519Signature2020");
        assert_eq!(proof.proof_purpose, "assertionMethod");
        assert_eq!(proof.verification_method, format!("{}#key-1", issuer_did));
        assert!(!proof.proof_value.is_empty());
    }

    #[test]
    fn test_credential_serialization() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec!["UniversityDegree".to_string()];
        let options = create_test_options();
        
        let credential = create_credential(
            issuer_did,
            subject,
            types,
            None,
            options,
        );
        
        let json = serde_json::to_string(&credential).unwrap();
        let deserialized: VerifiableCredential = serde_json::from_str(&json).unwrap();
        
        assert_eq!(credential.id, deserialized.id);
        assert_eq!(credential.issuer, deserialized.issuer);
        assert_eq!(credential.type_, deserialized.type_);
    }

    #[test]
    fn test_load_context_content_invalid_url() {
        let result = load_context_content("https://invalid.url/context");
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No local context file mapping for URL"));
    }
}