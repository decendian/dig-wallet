// tests/presentation_tests.rs
use verifiable_credentials::presentation::*;
use verifiable_credentials::format::*;
use verifiable_credentials::*;
use did_library::did::core::key_utils::KeyType;
use chrono::Utc;

#[cfg(test)]
mod presentation_tests {
    use super::*;

    fn create_test_credential() -> VerifiableCredential {
        let subject = CredentialSubject {
            id: Some("did:example:student".to_string()),
            name: "Alice Smith".to_string(),
            attributes: serde_json::json!({"degree": "Bachelor of Science"}),
        };
        
        let now = Utc::now();
        let formatted_issuance_date = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let future = now + chrono::Duration::days(180);
        let formatted_expiration_date = future.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        VerifiableCredential {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: "did:example:issuer".to_string(),
            key_type: KeyType::Ed25519,
            authentication: vec![],
            assertion_method: vec![],
            capability_invocation: vec![],
            capability_delegation: vec![],
            key_agreement: vec![],
            verification_method: vec![],
            service: vec![],
            type_: vec!["VerifiableCredential".to_string(), "UniversityDegree".to_string()],
            issuer: "did:example:issuer".to_string(),
            issuance_date: formatted_issuance_date,
            expiration_date: Some(formatted_expiration_date),
            credential_subject: subject,
            proof: None,
        }
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
        
        let presentation = create_presentation(holder_did.clone(), credentials.clone());
        
        // Basic presentation structure checks (similar to test_create_presentation_basic)
        assert_eq!(presentation.holder, holder_did);
        assert_eq!(presentation.verifiable_credential.len(), 2);
        assert!(presentation.type_.contains(&"VerifiablePresentation".to_string()));
        assert!(presentation.proof.is_none()); // Not signed initially
        assert!(presentation.id.starts_with("urn:uuid:"));
        assert!(presentation.context.contains(&"https://www.w3.org/2018/credentials/v1".to_string()));
        
        // Verify both credentials are present and have expected content
        for (i, credential) in presentation.verifiable_credential.iter().enumerate() {
            // Each credential should have basic required properties
            assert!(!credential.id.is_empty(), "Credential {} should have non-empty ID", i);
            assert!(!credential.issuer.is_empty(), "Credential {} should have non-empty issuer", i);
            assert!(credential.type_.contains(&"VerifiableCredential".to_string()), 
                   "Credential {} should contain base type", i);
            assert!(credential.type_.contains(&"UniversityDegree".to_string()),
                   "Credential {} should contain UniversityDegree type", i);
            
            // Verify credential subject
            assert_eq!(credential.credential_subject.name, "Alice Smith",
                      "Credential {} should have expected subject name", i);
            assert_eq!(credential.credential_subject.id, Some("did:example:student".to_string()),
                      "Credential {} should have expected subject ID", i);
            
            // Verify timestamps are valid
            assert!(chrono::DateTime::parse_from_rfc3339(&credential.issuance_date).is_ok(),
                   "Credential {} should have valid issuance date", i);
            if let Some(ref exp_date) = credential.expiration_date {
                assert!(chrono::DateTime::parse_from_rfc3339(exp_date).is_ok(),
                       "Credential {} should have valid expiration date", i);
            }
        }
        
        // Verify the credentials in the presentation match the original credentials
        assert_eq!(presentation.verifiable_credential[0].id, credentials[0].id);
        assert_eq!(presentation.verifiable_credential[1].id, credentials[1].id);
        assert_eq!(presentation.verifiable_credential[0].credential_subject.name, 
                  credentials[0].credential_subject.name);
        assert_eq!(presentation.verifiable_credential[1].credential_subject.name, 
                  credentials[1].credential_subject.name);
    }

    #[test]
    fn test_create_presentation_empty_credentials() {
        let holder_did = "did:example:holder".to_string();
        let credentials = vec![];
        
        let presentation = create_presentation(holder_did.clone(), credentials);
        
        assert_eq!(presentation.holder, holder_did);
        assert_eq!(presentation.verifiable_credential.len(), 0);
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
    fn test_sign_presentation_without_challenge_domain() {
        let holder_did = "did:example:holder".to_string();
        let credentials = vec![create_test_credential()];
        
        let mut presentation = create_presentation(holder_did, credentials);
        
        let result = sign_presentation(&mut presentation, None, None);
        
        assert!(result.is_ok());
        assert!(presentation.proof.is_some());
        
        let proof = presentation.proof.unwrap();
        assert_eq!(proof.challenge, None);
        assert_eq!(proof.domain, None);
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
    fn test_create_presentation_definition_with_custom_prefix() {
        let request_id = "request123".to_string();
        let credential_types = vec!["UniversityDegree".to_string()];
        let fields = vec![("name".to_string(), false)];
        let custom_prefix = Some("$.vc.credentialSubject".to_string());
        
        let definition = create_presentation_definition(
            request_id,
            credential_types,
            fields,
            None,
            custom_prefix,
            None,
        );
        
        let custom_field = &definition.input_descriptors[0].constraints.fields[1];
        assert_eq!(custom_field.path, vec!["$.vc.credentialSubject.name".to_string()]);
    }

    #[test]
    fn test_create_presentation_submission() {
        let definition_id = "request123".to_string();
        let descriptor_ids = vec!["desc1".to_string(), "desc2".to_string()];
        
        let submission = create_presentation_submission(definition_id.clone(), descriptor_ids.clone());
        
        assert_eq!(submission.definition_id, definition_id);
        assert!(submission.id.starts_with("submission_"));
        assert_eq!(submission.descriptor_map.len(), 2);
        
        // Check first descriptor mapping
        assert_eq!(submission.descriptor_map[0].id, descriptor_ids[0]);
        assert_eq!(submission.descriptor_map[0].format, "ldp_vp");
        assert_eq!(submission.descriptor_map[0].path, "$.verifiableCredential[0]");
        
        // Check second descriptor mapping
        assert_eq!(submission.descriptor_map[1].id, descriptor_ids[1]);
        assert_eq!(submission.descriptor_map[1].format, "ldp_vp");
        assert_eq!(submission.descriptor_map[1].path, "$.verifiableCredential[1]");
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

    #[test]
    fn test_signed_presentation_serialization() {
        let holder_did = "did:example:holder".to_string();
        let credentials = vec![create_test_credential()];
        let challenge = Some("test-challenge".to_string());
        
        let mut presentation = create_presentation(holder_did, credentials);
        sign_presentation(&mut presentation, challenge.clone(), None).unwrap();
        
        let json = serde_json::to_string(&presentation).unwrap();
        let deserialized: VerifiablePresentation = serde_json::from_str(&json).unwrap();
        
        assert_eq!(presentation.id, deserialized.id);
        assert!(deserialized.proof.is_some());
        
        let original_proof = presentation.proof.unwrap();
        let deserialized_proof = deserialized.proof.unwrap();
        assert_eq!(original_proof.challenge, deserialized_proof.challenge);
        assert_eq!(original_proof.proof_value, deserialized_proof.proof_value);
    }

    #[test]
    fn test_presentation_with_signed_credential() {
        let holder_did = "did:example:holder".to_string();
        let mut credential = create_test_credential();
        
        // Sign the credential first
        let _ = sign_credential(&mut credential);
        let credentials = vec![credential];
        
        let mut presentation = create_presentation(holder_did, credentials);
        let result = sign_presentation(&mut presentation, None, None);
        
        assert!(result.is_ok());
        assert!(presentation.proof.is_some());
        assert!(presentation.verifiable_credential[0].proof.is_some());
    }
}