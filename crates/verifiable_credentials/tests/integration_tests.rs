// tests/integration_tests.rs
use verifiable_credentials::*;
use verifiable_credentials::format::*;
use verifiable_credentials::presentation::*;
use did_library::did::core::key_utils::KeyType;
use chrono::Utc;
use std::collections::HashMap;

#[cfg(test)]
mod integration_tests {
    use super::*;

    // Helper function to create test credential without DID library dependencies
    fn create_test_credential_for_integration(
        issuer_did: String,
        subject: CredentialSubject,
        types: Vec<String>,
        expiration_date: Option<String>,
    ) -> VerifiableCredential {
        let now = Utc::now();
        let formatted_issuance_date = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let formatted_expiration_date = expiration_date.unwrap_or_else(|| {
            let future = now + chrono::Duration::days(180);
            future.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        });

        let mut credential_types = vec!["VerifiableCredential".to_string()];
        credential_types.extend(types);

        VerifiableCredential {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: issuer_did.clone(),
            key_type: KeyType::Ed25519,
            authentication: vec![],
            assertion_method: vec![],
            capability_invocation: vec![],
            capability_delegation: vec![],
            key_agreement: vec![],
            verification_method: vec![],
            service: vec![],
            type_: credential_types,
            issuer: issuer_did,
            issuance_date: formatted_issuance_date,
            expiration_date: Some(formatted_expiration_date),
            credential_subject: subject,
            proof: None,
        }
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
        // 1. Create a credential
        let subject = CredentialSubject {
            id: Some("did:example:student".to_string()),
            name: "Bob Johnson".to_string(),
            attributes: serde_json::json!({
                "degree": "Master of Science",
                "university": "Tech University"
            }),
        };
        
        let mut credential = create_test_credential_for_integration(
            "did:example:university".to_string(),
            subject,
            vec!["UniversityDegree".to_string()],
            None,
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
        
        // 4. Test serialization with data integrity verification
        let cred_json = serde_json::to_string(&credential).unwrap();
        let pres_json = serde_json::to_string(&presentation).unwrap();
        
        let cred_deserialized: VerifiableCredential = serde_json::from_str(&cred_json).unwrap();
        let pres_deserialized: VerifiablePresentation = serde_json::from_str(&pres_json).unwrap();
        
        // Verify credential data integrity after serialization roundtrip
        assert_eq!(credential.id, cred_deserialized.id);
        assert_eq!(credential.issuer, cred_deserialized.issuer);
        assert_eq!(credential.type_, cred_deserialized.type_);
        assert_eq!(credential.issuance_date, cred_deserialized.issuance_date);
        assert_eq!(credential.expiration_date, cred_deserialized.expiration_date);
        assert_eq!(credential.credential_subject.name, cred_deserialized.credential_subject.name);
        assert_eq!(credential.credential_subject.id, cred_deserialized.credential_subject.id);
        assert_eq!(credential.credential_subject.attributes, cred_deserialized.credential_subject.attributes);
        
        // Verify proof integrity
        assert_eq!(credential.proof.is_some(), cred_deserialized.proof.is_some());
        if let (Some(original_proof), Some(deserialized_proof)) = (&credential.proof, &cred_deserialized.proof) {
            assert_eq!(original_proof.type_, deserialized_proof.type_);
            assert_eq!(original_proof.created, deserialized_proof.created);
            assert_eq!(original_proof.verification_method, deserialized_proof.verification_method);
            assert_eq!(original_proof.proof_purpose, deserialized_proof.proof_purpose);
            assert_eq!(original_proof.proof_value, deserialized_proof.proof_value);
        }
        
        // Verify presentation data integrity after serialization roundtrip
        assert_eq!(presentation.id, pres_deserialized.id);
        assert_eq!(presentation.holder, pres_deserialized.holder);
        assert_eq!(presentation.type_, pres_deserialized.type_);
        assert_eq!(presentation.context, pres_deserialized.context);
        assert_eq!(presentation.verifiable_credential.len(), pres_deserialized.verifiable_credential.len());
        
        // Verify the credential within the presentation is intact
        let orig_cred_in_pres = &presentation.verifiable_credential[0];
        let deser_cred_in_pres = &pres_deserialized.verifiable_credential[0];
        assert_eq!(orig_cred_in_pres.id, deser_cred_in_pres.id);
        assert_eq!(orig_cred_in_pres.issuer, deser_cred_in_pres.issuer);
        assert_eq!(orig_cred_in_pres.credential_subject.name, deser_cred_in_pres.credential_subject.name);
        assert_eq!(orig_cred_in_pres.credential_subject.attributes, deser_cred_in_pres.credential_subject.attributes);
    }
    
    #[test]
    fn test_presentation_exchange_workflow() {
        use verifiable_credentials::presentation::exchange::*;
        
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
        
        let credential = create_test_credential_for_integration(
            "did:example:university".to_string(),
            subject,
            vec!["UniversityDegree".to_string()],
            None,
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
    fn test_credential_with_multiple_subjects() {
        let subjects = vec![
            CredentialSubject {
                id: Some("did:example:student1".to_string()),
                name: "Student One".to_string(),
                attributes: serde_json::json!({"major": "Computer Science"}),
            },
            CredentialSubject {
                id: Some("did:example:student2".to_string()),
                name: "Student Two".to_string(),
                attributes: serde_json::json!({"major": "Mathematics"}),
            },
        ];

        let mut credentials = Vec::new();
        for (i, subject) in subjects.into_iter().enumerate() {
            let credential = create_test_credential_for_integration(
                format!("did:example:university{}", i),
                subject,
                vec!["UniversityDegree".to_string()],
                None,
            );
            credentials.push(credential);
        }

        assert_eq!(credentials.len(), 2);
        assert_eq!(credentials[0].credential_subject.name, "Student One");
        assert_eq!(credentials[1].credential_subject.name, "Student Two");

        // Create presentation with multiple credentials
        let presentation = create_presentation(
            "did:example:presenter".to_string(),
            credentials,
        );

        assert_eq!(presentation.verifiable_credential.len(), 2);
    }

    #[test]
    fn test_credential_lifecycle_with_signing() {
        // Test the complete lifecycle: create -> sign -> present -> verify
        
        // 1. Create credential
        let subject = CredentialSubject {
            id: Some("did:example:graduate".to_string()),
            name: "Graduate Student".to_string(),
            attributes: serde_json::json!({
                "degree": "PhD",
                "field": "Computer Science",
                "graduation_year": 2024,
                "dissertation": "Advanced AI Systems"
            }),
        };

        let mut credential = create_test_credential_for_integration(
            "did:example:mit".to_string(),
            subject,
            vec!["UniversityDegree".to_string(), "DoctorateDegree".to_string()],
            Some("2030-12-31T23:59:59Z".to_string()),
        );

        // 2. Sign credential
        let sign_result = sign_credential(&mut credential);
        assert!(sign_result.is_ok());
        
        let proof = credential.proof.as_ref().unwrap();
        assert_eq!(proof.type_, "Ed25519Signature2020");
        assert!(!proof.proof_value.is_empty());

        // 3. Create presentation
        let mut presentation = create_presentation(
            "did:example:graduate".to_string(),
            vec![credential.clone()],
        );

        // 4. Sign presentation
        let challenge = Some("employer-verification-2024".to_string());
        let domain = Some("acme-corp.com".to_string());
        
        let sign_pres_result = sign_presentation(
            &mut presentation,
            challenge.clone(),
            domain.clone(),
        );
        assert!(sign_pres_result.is_ok());

        let pres_proof = presentation.proof.as_ref().unwrap();
        assert_eq!(pres_proof.challenge, challenge);
        assert_eq!(pres_proof.domain, domain);

        // 5. Verify complete structure
        assert!(presentation.verifiable_credential[0].proof.is_some());
        assert!(presentation.proof.is_some());
        assert_eq!(presentation.verifiable_credential.len(), 1);
    }

    #[test]
    fn test_issue_credential_error_handling() {
        // Test the issue_credential function's error handling
        let request = CredentialRequest {
            subject: CredentialSubject {
                id: Some("did:example:test".to_string()),
                name: "Test Subject".to_string(),
                attributes: serde_json::json!({"test": true}),
            },
            credential_type: vec!["TestCredential".to_string()],
            expiration_date: None,
        };

        let result = issue_credential(request);
        
        // This will likely fail in test environment, which is expected
        match result {
            Ok(credential) => {
                // If successful, verify basic structure
                assert!(!credential.id.is_empty());
                assert!(!credential.issuer.is_empty());
                assert!(credential.type_.contains(&"VerifiableCredential".to_string()));
                assert!(credential.proof.is_some());
            },
            Err(err) => {
                // Expected errors when DID registry is not available
                assert!(
                    err.contains("Could not find project root") ||
                    err.contains("No DID found in registry") ||
                    err.contains("DID document not found") ||
                    err.contains("Cannot issue credential") ||
                    err.contains("NotPresent")
                );
            }
        }
    }
  
#[test]
    fn test_complex_presentation_exchange() {

        // Create a complex presentation definition with filters
        let credential_types = vec!["UniversityDegree".to_string(), "EmploymentCredential".to_string()];
        let fields = vec![
            ("name".to_string(), false),
            ("graduation_year".to_string(), false),
            ("gpa".to_string(), true),
            ("experience_years".to_string(), true),
        ];

        let mut filters = HashMap::new();
        filters.insert(
            "graduation_year".to_string(),
            serde_json::json!({
                "type": "number",
                "minimum": 2020
            })
        );

        let definition = create_presentation_definition(
            "complex-verification".to_string(),
            credential_types.clone(),
            fields.clone(),
            Some("Complex employment verification".to_string()),
            Some("$.vc.credentialSubject".to_string()),
            Some(filters),
        );

        // Verify the definition structure
        assert_eq!(definition.id, "complex-verification");
        assert_eq!(definition.input_descriptors.len(), 1);

        let descriptor = &definition.input_descriptors[0];
        assert_eq!(descriptor.purpose, Some("Complex employment verification".to_string()));
        
        // Should have at least the fields we requested plus the type field
        assert!(descriptor.constraints.fields.len() >= fields.len());
        
        // Verify that credential types are properly set in the type constraint
        let type_field = descriptor.constraints.fields.iter()
            .find(|field| field.path.iter().any(|path| path.contains("$.type")))
            .expect("Should have a type constraint field");
        
        assert!(type_field.filter.is_some());
        let type_filter = type_field.filter.as_ref().unwrap();
        
        // Check if the filter contains our credential types
        if let Some(contains) = type_filter.get("contains") {
            if let Some(enum_values) = contains.get("enum") {
                if let Some(enum_array) = enum_values.as_array() {
                    for cred_type in &credential_types {
                        assert!(enum_array.contains(&serde_json::json!(cred_type)),
                               "Type filter should contain credential type: {}", cred_type);
                    }
                }
            }
        }
        
        // Look for a field that has our custom filter
        let has_custom_filter = descriptor.constraints.fields.iter()
            .any(|field| {
                if let Some(filter) = &field.filter {
                    filter.get("minimum").is_some()
                } else {
                    false
                }
            });
        
        assert!(has_custom_filter, "Should have at least one field with a custom filter");
        
        // Verify custom prefix was applied to at least one field
        let has_custom_prefix = descriptor.constraints.fields.iter()
            .any(|field| {
                field.path.iter().any(|path| path.contains("$.vc.credentialSubject"))
            });
        
        assert!(has_custom_prefix, "Should have fields with custom prefix");
    }
}