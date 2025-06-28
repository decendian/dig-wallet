// tests/presentation_tests.rs
use verifiable_credentials::presentation::*;
use verifiable_credentials::format::*;
use verifiable_credentials::*;
use did_library::did::core::did_document::DIDCreationOptions;

#[cfg(test)]
mod presentation_tests {
    use super::*;

    fn create_test_credential() -> VerifiableCredential {
        let subject = CredentialSubject {
            id: Some("did:example:student".to_string()),
            name: "Alice Smith".to_string(),
            attributes: serde_json::json!({"degree": "Bachelor of Science"}),
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
        
        create_credential(
            "did:example:issuer".to_string(),
            subject,
            vec!["UniversityDegree".to_string()],
            None,
            options,
        )
    }

    #[test]
    fn test_create_presentation_basic() {
        let holder_did = "did:example:holder".to_string();
        let credentials = vec![create_test_credential()];
        
        let presentation = create_presentation(holder_did.clone(), credentials.clone());
        
        assert_eq!(presentation.holder, holder_did);
        assert_eq!(presentation.verifiable_credential.len(), 1);
        assert!(presentation.type_.contains(&"VerifiablePresentation".to_string()));
        assert!(presentation.proof.is_none());
        assert!(presentation.id.starts_with("urn:uuid:"));
    }

    #[test]
    fn test_create_presentation_multiple_credentials() {
        let holder_did = "did:example:holder".to_string();
        let credentials = vec![
            create_test_credential(),
            create_test_credential(),
        ];
        
        let presentation = create_presentation(holder_did.clone(), credentials);
        
        assert_eq!(presentation.holder, holder_did);
        assert_eq!(presentation.verifiable_credential.len(), 2);
    }

    #[test]
    fn test_sign_presentation_success() {
        let holder_did = "did:example:holder".to_string();
        let credentials = vec![create_test_credential()];
        let challenge = Some("challenge123".to_string());
        let domain = Some("example.com".to_string());
        
        let mut presentation = create_presentation(holder_did.clone(), credentials);
        
        let result = sign_presentation(&mut presentation, challenge.clone(), domain.clone());
        
        assert!(result.is_ok());
        assert!(presentation.proof.is_some());
        
        let proof = presentation.proof.unwrap();
        assert_eq!(proof.proof_purpose, "authentication");
        assert_eq!(proof.verification_method, format!("{}#key-1", holder_did));
        assert_eq!(proof.challenge, challenge);
        assert_eq!(proof.domain, domain);
        assert!(!proof.proof_value.is_empty());
    }

    #[test]
    fn test_create_presentation_definition() {
        let request_id = "request123".to_string();
        let credential_types = vec!["UniversityDegree".to_string()];
        let fields = vec![
            ("name".to_string(), false),
            ("degree".to_string(), false),
        ];
        let purpose = Some("Verify education credentials".to_string());
        
        let definition = create_presentation_definition(
            request_id.clone(),
            credential_types.clone(),
            fields.clone(),
            purpose.clone(),
            None,
            None,
        );
        
        assert_eq!(definition.id, request_id);
        assert_eq!(definition.input_descriptors.len(), 1);
        
        let descriptor = &definition.input_descriptors[0];
        assert_eq!(descriptor.purpose, purpose);
        assert_eq!(descriptor.constraints.fields.len(), 3); // 1 type + 2 custom fields
    }

    #[test]
    fn test_create_presentation_submission() {
        let definition_id = "request123".to_string();
        let descriptor_ids = vec!["desc1".to_string(), "desc2".to_string()];
        
        let submission = create_presentation_submission(definition_id.clone(), descriptor_ids.clone());
        
        assert_eq!(submission.definition_id, definition_id);
        assert!(submission.id.starts_with("submission_"));
        assert_eq!(submission.descriptor_map.len(), 2);
        
        assert_eq!(submission.descriptor_map[0].id, descriptor_ids[0]);
        assert_eq!(submission.descriptor_map[0].format, "ldp_vp");
        assert_eq!(submission.descriptor_map[0].path, "$.verifiableCredential[0]");
    }

    #[test]
    fn test_presentation_serialization() {
        let holder_did = "did:example:holder".to_string();
        let credentials = vec![create_test_credential()];
        
        let presentation = create_presentation(holder_did, credentials);
        
        let json = serde_json::to_string(&presentation).unwrap();
        let deserialized: VerifiablePresentation = serde_json::from_str(&json).unwrap();
        
        assert_eq!(presentation.id, deserialized.id);
        assert_eq!(presentation.holder, deserialized.holder);
        assert_eq!(presentation.type_, deserialized.type_);
    }
}