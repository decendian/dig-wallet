// tests/dto_tests.rs
use serde_json;
use chrono::Utc;
use std::collections::HashMap;

use verifiable_credentials::dto::credential::*;

#[cfg(test)]
mod dto_tests {
    use super::*;

    // Helper function to create a test CredentialSubjectDTO
    fn create_test_subject_dto() -> CredentialSubjectDTO {
        let mut claims = HashMap::new();
        claims.insert("name".to_string(), serde_json::json!("John Doe"));
        claims.insert("age".to_string(), serde_json::json!(30));
        
        CredentialSubjectDTO {
            id: "did:example:123456789".to_string(),
            claims,
        }
    }

    // Helper function to create a test ProofDTO
    fn create_test_proof_dto() -> ProofDTO {
        ProofDTO {
            type_: "Ed25519Signature2018".to_string(),
            created: Utc::now(),
            verification_method: "did:example:issuer#key-1".to_string(),
            proof_purpose: "assertionMethod".to_string(),
            proof_value: "base64encodedproof".to_string(),
        }
    }

    #[test]
    fn test_credential_subject_dto_creation() {
        let subject = create_test_subject_dto();
        
        assert_eq!(subject.id, "did:example:123456789");
        assert_eq!(subject.claims.len(), 2);
        assert_eq!(subject.claims["name"], serde_json::json!("John Doe"));
    }

    #[test]
    fn test_credential_subject_dto_serialization() {
        let subject = create_test_subject_dto();
        
        let json = serde_json::to_string(&subject).unwrap();
        let deserialized: CredentialSubjectDTO = serde_json::from_str(&json).unwrap();
        
        assert_eq!(subject.id, deserialized.id);
        assert_eq!(subject.claims, deserialized.claims);
    }

    #[test]
    fn test_issuer_dto_string_variant() {
        let issuer = IssuerDTO::String("did:example:issuer".to_string());
        
        let json = serde_json::to_string(&issuer).unwrap();
        let deserialized: IssuerDTO = serde_json::from_str(&json).unwrap();
        
        match deserialized {
            IssuerDTO::String(id) => assert_eq!(id, "did:example:issuer"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_issuer_dto_object_variant() {
        let mut additional_props = HashMap::new();
        additional_props.insert("website".to_string(), serde_json::json!("https://example.com"));
        
        let issuer = IssuerDTO::Object {
            id: "did:example:issuer".to_string(),
            name: Some("Example Organization".to_string()),
            additional_properties: additional_props,
        };
        
        let json = serde_json::to_string(&issuer).unwrap();
        let deserialized: IssuerDTO = serde_json::from_str(&json).unwrap();
        
        match deserialized {
            IssuerDTO::Object { id, name, .. } => {
                assert_eq!(id, "did:example:issuer");
                assert_eq!(name, Some("Example Organization".to_string()));
            },
            _ => panic!("Expected Object variant"),
        }
    }

    #[test]
    fn test_proof_dto_creation() {
        let proof = create_test_proof_dto();
        
        assert_eq!(proof.type_, "Ed25519Signature2018");
        assert_eq!(proof.verification_method, "did:example:issuer#key-1");
        assert_eq!(proof.proof_purpose, "assertionMethod");
        assert!(!proof.proof_value.is_empty());
    }

    #[test]
    fn test_verifiable_credential_dto_validation_success() {
        let subject = create_test_subject_dto();
        let proof = create_test_proof_dto();
        
        let credential = VerifiableCredentialDTO {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: "urn:uuid:12345".to_string(),
            type_: vec!["VerifiableCredential".to_string()],
            issuer: IssuerDTO::String("did:example:issuer".to_string()),
            issuance_date: Utc::now(),
            expiration_date: None,
            credential_subject: subject,
            proof: vec![proof],
            additional_properties: HashMap::new(),
        };
        
        assert!(credential.validate().is_ok());
    }

    #[test]
    fn test_verifiable_credential_dto_validation_empty_id() {
        let subject = create_test_subject_dto();
        let proof = create_test_proof_dto();
        
        let credential = VerifiableCredentialDTO {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: "".to_string(), // Empty ID should fail
            type_: vec!["VerifiableCredential".to_string()],
            issuer: IssuerDTO::String("did:example:issuer".to_string()),
            issuance_date: Utc::now(),
            expiration_date: None,
            credential_subject: subject,
            proof: vec![proof],
            additional_properties: HashMap::new(),
        };
        
        assert_eq!(credential.validate(), Err("Credential ID cannot be empty".to_string()));
    }

    #[test]
    fn test_verifiable_credential_dto_validation_empty_context() {
        let subject = create_test_subject_dto();
        let proof = create_test_proof_dto();
        
        let credential = VerifiableCredentialDTO {
            context: vec![], // Empty context should fail
            id: "urn:uuid:12345".to_string(),
            type_: vec!["VerifiableCredential".to_string()],
            issuer: IssuerDTO::String("did:example:issuer".to_string()),
            issuance_date: Utc::now(),
            expiration_date: None,
            credential_subject: subject,
            proof: vec![proof],
            additional_properties: HashMap::new(),
        };
        
        assert_eq!(credential.validate(), Err("Context cannot be empty".to_string()));
    }

    #[test]
    fn test_verifiable_credential_dto_validation_empty_type() {
        let subject = create_test_subject_dto();
        let proof = create_test_proof_dto();
        
        let credential = VerifiableCredentialDTO {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: "urn:uuid:12345".to_string(),
            type_: vec![], // Empty type should fail
            issuer: IssuerDTO::String("did:example:issuer".to_string()),
            issuance_date: Utc::now(),
            expiration_date: None,
            credential_subject: subject,
            proof: vec![proof],
            additional_properties: HashMap::new(),
        };
        
        assert_eq!(credential.validate(), Err("Type cannot be empty".to_string()));
    }
}
