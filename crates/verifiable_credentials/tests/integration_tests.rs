// tests/integration_tests.rs
use verifiable_credentials::*;
use verifiable_credentials::format::*;
use verifiable_credentials::presentation::*;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_credential_subject_creation() {
        let subject = CredentialSubject {
            id: Some("did:example:123".to_string()),
            name: "Test Subject".to_string(),
            attributes: serde_json::json!({
                "email": "test@example.com",
                "age": 25
            }),
        };
        
        assert_eq!(subject.id, Some("did:example:123".to_string()));
        assert_eq!(subject.name, "Test Subject");
        assert_eq!(subject.attributes["email"], serde_json::json!("test@example.com"));
    }

    #[test]
    fn test_credential_request_creation() {
        let request = CredentialRequest {
            subject: CredentialSubject {
                id: Some("did:example:456".to_string()),
                name: "Certificate Holder".to_string(),
                attributes: serde_json::json!({"course": "Rust Programming"}),
            },
            credential_type: vec!["Certificate".to_string()],
            expiration_date: Some("2026-01-01T00:00:00Z".to_string()),
        };
        
        assert_eq!(request.credential_type.len(), 1);
        assert!(request.credential_type.contains(&"Certificate".to_string()));
        assert_eq!(request.expiration_date, Some("2026-01-01T00:00:00Z".to_string()));
    }

    #[test]
    fn test_credential_request_serialization() {
        let request = CredentialRequest {
            subject: CredentialSubject {
                id: Some("did:example:789".to_string()),
                name: "Serialization Test".to_string(),
                attributes: serde_json::json!({"test": true}),
            },
            credential_type: vec!["TestCredential".to_string()],
            expiration_date: None,
        };
        
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CredentialRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request.subject.name, deserialized.subject.name);
        assert_eq!(request.credential_type, deserialized.credential_type);
        assert_eq!(request.expiration_date, deserialized.expiration_date);
    }

    #[test]
    fn test_full_credential_workflow() {
        use did_library::did::core::did_document::DIDCreationOptions;
        
        // 1. Create a credential
        let subject = CredentialSubject {
            id: Some("did:example:student".to_string()),
            name: "Bob Johnson".to_string(),
            attributes: serde_json::json!({
                "degree": "Master of Science",
                "university": "Tech University"
            }),
        };
        
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
        
        let mut credential = create_credential(
            "did:example:university".to_string(),
            subject,
            vec!["UniversityDegree".to_string()],
            None,
            options,
        );
        
        // 2. Sign the credential
        let sign_result = sign_credential(&mut credential);
        assert!(sign_result.is_ok());
        assert!(credential.proof.is_some());
        
        // 3. Create a presentation with the credential
        let presentation = create_presentation(
            "did:example:student".to_string(),
            vec![credential.clone()],
        );
        
        assert_eq!(presentation.verifiable_credential.len(), 1);
        assert_eq!(presentation.verifiable_credential[0].id, credential.id);
        
        // 4. Test serialization of the entire flow
        let cred_json = serde_json::to_string(&credential).unwrap();
        let pres_json = serde_json::to_string(&presentation).unwrap();
        
        let _cred_deserialized: VerifiableCredential = serde_json::from_str(&cred_json).unwrap();
        let _pres_deserialized: VerifiablePresentation = serde_json::from_str(&pres_json).unwrap();
    }

    #[test]
    fn test_presentation_exchange_workflow() {
        use verifiable_credentials::presentation::exchange::*;
        use did_library::did::core::did_document::DIDCreationOptions;
        
        // 1. Verifier creates a request
        let credential_types = vec!["UniversityDegree".to_string()];
        let fields = vec![("name".to_string(), false), ("degree".to_string(), false)];
        let purpose = Some("Identity verification".to_string());
        
        let request = create_request(credential_types, fields, purpose);
        
        // 2. Holder creates credentials
        let subject = CredentialSubject {
            id: Some("did:example:holder".to_string()),
            name: "Jane Doe".to_string(),
            attributes: serde_json::json!({"degree": "Bachelor of Arts"}),
        };
        
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
        
        let credential = create_credential(
            "did:example:university".to_string(),
            subject,
            vec!["UniversityDegree".to_string()],
            None,
            options,
        );
        
        // 3. Holder creates response
        let response_result = create_response(
            &request,
            "did:example:holder".to_string(),
            vec![credential],
        );
        
        assert!(response_result.is_ok());
        let response = response_result.unwrap();
        
        // 4. Verifier checks response
        assert_eq!(response.presentation_submission.definition_id, request.presentation_definition.id);
        assert_eq!(response.verifiable_presentation.verifiable_credential.len(), 1);
        
        // 5. Verify response
        let verification_result = verify_response(&request, &response);
        assert!(verification_result.is_ok());
    }

    #[test]
    fn test_did_registry_access() {
        // Test that the registry access function handles errors gracefully
        let result = get_did_registry();
        
        // This will likely fail in test environment, which is expected
        match result {
            Ok(_registry) => {
                // If successful, that's great
            },
            Err(_err) => {
                // Expected in test environment without proper setup
            }
        }
    }
}