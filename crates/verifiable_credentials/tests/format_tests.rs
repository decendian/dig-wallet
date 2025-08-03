// tests/format_tests.rs
use verifiable_credentials::format::*;
use verifiable_credentials::*;
use did_library::did::core::did_document::*;
use did_library::did::core::key_utils::KeyType;
use chrono::Utc;

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

    // Create a test credential manually without going through DID creation
    fn create_test_credential_manual(
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
    fn test_create_credential_basic() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec!["UniversityDegree".to_string()];
        
        let credential = create_test_credential_manual(
            issuer_did.clone(),
            subject.clone(),
            types.clone(),
            None,
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
        
        let credential = create_test_credential_manual(
            issuer_did,
            subject,
            types,
            Some(custom_expiration.clone()),
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
        
        let credential = create_test_credential_manual(
            issuer_did,
            subject,
            types.clone(),
            None,
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
        
        let mut credential = create_test_credential_manual(
            issuer_did.clone(),
            subject,
            types,
            None,
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
        
        let credential = create_test_credential_manual(
            issuer_did,
            subject,
            types,
            None,
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

    #[test]
    fn test_credential_has_valid_timestamps() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec!["UniversityDegree".to_string()];
        
        let credential = create_test_credential_manual(
            issuer_did,
            subject,
            types,
            None,
        );
        
        // Check that issuance date is valid RFC3339
        assert!(chrono::DateTime::parse_from_rfc3339(&credential.issuance_date).is_ok());
        
        // Check that expiration date is valid RFC3339 and in the future
        assert!(credential.expiration_date.is_some());
        let exp_date = chrono::DateTime::parse_from_rfc3339(
            &credential.expiration_date.unwrap()
        ).unwrap();
        assert!(exp_date > chrono::Utc::now());
    }

    #[test]
    fn test_credential_with_complex_subject_attributes() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = CredentialSubject {
            id: Some("did:example:student".to_string()),
            name: "Complex Student".to_string(),
            attributes: serde_json::json!({
                "degrees": [
                    {"type": "Bachelor", "field": "Computer Science", "year": 2020},
                    {"type": "Master", "field": "Data Science", "year": 2022}
                ],
                "gpa": 3.85,
                "honors": ["Magna Cum Laude", "Dean's List"],
                "thesis": {
                    "title": "Advanced Machine Learning Applications",
                    "advisor": "Dr. Smith",
                    "pages": 150
                }
            }),
        };
        let types = vec!["EducationCredential".to_string()];
        
        let credential = create_test_credential_manual(
            issuer_did,
            subject.clone(),
            types,
            None,
        );
        
        assert_eq!(credential.credential_subject.name, subject.name);
        assert_eq!(credential.credential_subject.attributes["gpa"], serde_json::json!(3.85));
        assert!(credential.credential_subject.attributes["degrees"].is_array());
    }

    #[test]
    fn test_signed_credential_serialization() {
        let issuer_did = "did:example:issuer".to_string();
        let subject = create_test_credential_subject();
        let types = vec!["UniversityDegree".to_string()];
        
        let mut credential = create_test_credential_manual(
            issuer_did,
            subject,
            types,
            None,
        );
        
        // Sign the credential
        sign_credential(&mut credential).unwrap();
        
        // Test serialization of signed credential
        let json = serde_json::to_string(&credential).unwrap();
        let deserialized: VerifiableCredential = serde_json::from_str(&json).unwrap();
        
        assert_eq!(credential.id, deserialized.id);
        assert!(deserialized.proof.is_some());
        
        let original_proof = credential.proof.unwrap();
        let deserialized_proof = deserialized.proof.unwrap();
        assert_eq!(original_proof.type_, deserialized_proof.type_);
        assert_eq!(original_proof.proof_value, deserialized_proof.proof_value);
    }
}