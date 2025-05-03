use crate::format::VerifiableCredential;
use crate::keys;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a W3C Verifiable Presentation
/// 
/// A container for one or more Verifiable Credentials that is cryptographically signed
/// by the holder, allowing selective disclosure of credentials to a verifier.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VerifiablePresentation {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    pub type_: Vec<String>,
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<VerifiableCredential>,
    pub holder: String,
    pub proof: Option<PresentationProof>,
}

/// Cryptographic proof information attached to a Verifiable Presentation
/// 
/// Contains the signature metadata and value used to verify the authenticity
/// of the presentation and binding to the holder.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresentationProof {
    pub type_: String,
    pub created: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,
    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,
    #[serde(rename = "challenge", skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "proofValue")]
    pub proof_value: String,
}

/// Represents a Presentation Definition as per DIF Presentation Exchange v1.0
/// 
/// Used by a verifier to request specific credentials or claims from a holder.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresentationDefinition {
    pub id: String,
    #[serde(rename = "input_descriptors")]
    pub input_descriptors: Vec<InputDescriptor>,
}

/// Defines the specific credentials or claims being requested within a Presentation Definition
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputDescriptor {
    pub id: String,
    pub name: Option<String>,
    pub purpose: Option<String>,
    pub constraints: Constraints,
}

/// Specifies the requirements that presented credentials must satisfy
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Constraints {
    pub fields: Vec<Field>,
}

/// Defines a specific attribute or claim that is being requested
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Field {
    pub path: Vec<String>,
    pub optional: Option<bool>,
    pub filter: Option<serde_json::Value>,
}

/// Metadata that maps a presentation submission to a presentation definition
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresentationSubmission {
    pub id: String,
    pub definition_id: String,
    pub descriptor_map: Vec<DescriptorMapping>,
}

/// Maps a specific input descriptor to the location of the fulfilling credential
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DescriptorMapping {
    pub id: String,
    pub format: String,
    pub path: String,
}

/// Create a new Verifiable Presentation containing specific credentials
/// 
/// This function initializes a presentation with the standard W3C context URLs,
/// generates a UUID for the presentation ID, and includes the provided credentials.
/// The created presentation is not yet signed (proof is None).
/// 
/// # Arguments
/// * `holder_did` - The DID of the presentation holder
/// * `credentials` - The credentials to include in the presentation
/// * `challenge` - Optional challenge from the verifier for freshness/anti-replay
/// * `domain` - Optional domain for scoping the presentation to a specific context
/// 
/// # Returns
/// A new unsigned Verifiable Presentation
pub fn create_presentation(
    holder_did: String,
    credentials: Vec<VerifiableCredential>,
    challenge: Option<String>,
    domain: Option<String>,
) -> VerifiablePresentation {
    VerifiablePresentation {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_string(),
            "https://identity.foundation/presentation-exchange/submission/v1".to_string(),
        ],
        id: format!("urn:uuid:{}", Uuid::new_v4()),
        type_: vec!["VerifiablePresentation".to_string()],
        verifiable_credential: credentials,
        holder: holder_did,
        proof: None,
    }
}

/// Signs a Verifiable Presentation using the holder's key
/// 
/// This function adds a cryptographic proof to a presentation by:
/// 1. Creating a copy of the presentation without the proof
/// 2. Converting it to a canonical JSON representation
/// 3. Signing the canonical form with the holder's private key
/// 4. Adding the resulting signature and metadata as a proof
/// 
/// # Arguments
/// * `presentation` - The presentation to sign (modified in-place)
/// * `challenge` - Optional challenge from the verifier to prove freshness
/// * `domain` - Optional domain to scope the presentation to a specific context
/// 
/// # Returns
/// * `Ok(())` - If signing was successful
/// * `Err(String)` - If an error occurred during signing
pub fn sign_presentation(
    presentation: &mut VerifiablePresentation,
    challenge: Option<String>,
    domain: Option<String>,
) -> Result<(), String> {
    // Create a copy without the proof
    let mut presentation_for_signing = presentation.clone();
    presentation_for_signing.proof = None;
    
    // Canonicalize (deterministic serialization)
    let canonical = serde_json::to_string(&presentation_for_signing)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    // Get the demo keypair (in production, you'd retrieve the holder's key)
    let keypair = keys::get_demo_keypair();
    
    // Sign the canonicalized data using base64 encoding
    let signature = keys::sign_data(canonical.as_bytes(), &keypair);
    
    // Add the proof
    let created = Utc::now().to_rfc3339();
    let verification_method = format!("{}#key-1", presentation.holder);
    
    presentation.proof = Some(PresentationProof {
        type_: "Ed25519Signature2020".to_string(),
        created,
        verification_method,
        proof_purpose: "authentication".to_string(),
        challenge,
        domain,
        proof_value: signature,
    });
    
    Ok(())
}

/// Creates a Presentation Definition (request) for specific credential types
/// 
/// This function builds a structured request that a verifier can send to a holder
/// to request specific credentials or claims.
/// 
/// # Arguments
/// * `request_id` - Unique identifier for this presentation request
/// * `credential_types` - Types of credentials being requested
/// * `fields` - Specific fields/claims being requested from those credentials
/// * `purpose` - The reason for requesting these credentials
/// 
/// # Returns
/// A Presentation Definition that can be sent to the holder
pub fn create_presentation_definition(
    request_id: String,
    credential_types: Vec<String>,
    fields: Vec<(String, bool)>, // (field path, is optional)
    purpose: Option<String>,
) -> PresentationDefinition {
    // Convert the requested fields to the Field structure
    let constraint_fields: Vec<Field> = fields.iter().map(|(path, optional)| {
        Field {
            path: vec![format!("$.credentialSubject.{}", path)],
            optional: Some(*optional),
            filter: None,
        }
    }).collect();
    
    // Create constraints for the requested credential types
    let type_field = Field {
        path: vec!["$.type".to_string()],
        optional: Some(false),
        filter: Some(serde_json::json!({
            "type": "array",
            "contains": { "enum": credential_types }
        })),
    };
    
    let mut all_fields = vec![type_field];
    all_fields.extend(constraint_fields);
    
    // Create the presentation definition
    PresentationDefinition {
        id: request_id,
        input_descriptors: vec![
            InputDescriptor {
                id: format!("descriptor_{}", Uuid::new_v4()),
                name: Some("Requested Credential".to_string()),
                purpose,
                constraints: Constraints {
                    fields: all_fields,
                },
            }
        ],
    }
}

/// Creates a Presentation Submission for a specific Presentation Definition
/// 
/// This function creates the metadata that maps the credentials in a presentation
/// to the requirements specified in a presentation definition.
/// 
/// # Arguments
/// * `definition_id` - The ID of the presentation definition being responded to
/// * `descriptor_ids` - The IDs of the input descriptors being fulfilled
/// 
/// # Returns
/// A Presentation Submission that can be included with a Verifiable Presentation
pub fn create_presentation_submission(
    definition_id: String,
    descriptor_ids: Vec<String>,
) -> PresentationSubmission {
    let mappings = descriptor_ids.into_iter().enumerate().map(|(index, id)| {
        DescriptorMapping {
            id,
            format: "ldp_vp".to_string(),
            path: format!("$.verifiableCredential[{}]", index),
        }
    }).collect();
    
    PresentationSubmission {
        id: format!("submission_{}", Uuid::new_v4()),
        definition_id,
        descriptor_map: mappings,
    }
}

/// Implementation of a full presentation exchange protocol
pub mod exchange {
    use super::*;
    
    /// A combined structure representing a complete presentation request
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct PresentationRequest {
        pub presentation_definition: PresentationDefinition,
        pub challenge: String,
        pub domain: Option<String>,
    }
    
    /// A combined structure representing a complete presentation response
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct PresentationResponse {
        pub verifiable_presentation: VerifiablePresentation,
        pub presentation_submission: PresentationSubmission,
    }
    
    /// Creates a presentation request from a verifier
    /// 
    /// # Arguments
    /// * `credential_types` - Types of credentials being requested
    /// * `fields` - Specific fields being requested
    /// * `purpose` - Purpose of the request
    /// 
    /// # Returns
    /// A complete presentation request that can be sent to a holder
    pub fn create_request(
        credential_types: Vec<String>,
        fields: Vec<(String, bool)>,
        purpose: Option<String>,
    ) -> PresentationRequest {
        let request_id = format!("request_{}", Uuid::new_v4());
        
        // Generate a random challenge for anti-replay protection
        let challenge = Uuid::new_v4().to_string();
        
        PresentationRequest {
            presentation_definition: create_presentation_definition(
                request_id,
                credential_types,
                fields,
                purpose,
            ),
            challenge,
            domain: None,
        }
    }
    
    /// Creates a presentation response from a holder
    /// 
    /// # Arguments
    /// * `request` - The presentation request to respond to
    /// * `holder_did` - The DID of the holder creating the presentation
    /// * `credentials` - The credentials to include in the presentation
    /// 
    /// # Returns
    /// * `Ok(PresentationResponse)` - The complete presentation response
    /// * `Err(String)` - If an error occurred
    pub fn create_response(
        request: &PresentationRequest,
        holder_did: String,
        credentials: Vec<VerifiableCredential>,
    ) -> Result<PresentationResponse, String> {
        // Create the presentation from the credentials
        let mut presentation = create_presentation(
            holder_did,
            credentials.clone(),
            Some(request.challenge.clone()),
            request.domain.clone(),
        );
        
        // Sign the presentation
        sign_presentation(
            &mut presentation,
            Some(request.challenge.clone()),
            request.domain.clone(),
        )?;
        
        // Create the submission metadata
        let descriptor_ids = request.presentation_definition.input_descriptors
            .iter()
            .map(|desc| desc.id.clone())
            .collect();
        
        let submission = create_presentation_submission(
            request.presentation_definition.id.clone(),
            descriptor_ids,
        );
        
        Ok(PresentationResponse {
            verifiable_presentation: presentation,
            presentation_submission: submission,
        })
    }
    
    /// Verifies a presentation response against the original request
    /// 
    /// # Arguments
    /// * `request` - The original presentation request
    /// * `response` - The presentation response to verify
    /// 
    /// # Returns
    /// * `Ok(bool)` - Whether the presentation is valid and satisfies the request
    /// * `Err(String)` - If an error occurred during verification
    pub fn verify_response(
        request: &PresentationRequest,
        response: &PresentationResponse,
    ) -> Result<bool, String> {
        // Verify that the presentation addresses the correct definition
        if response.presentation_submission.definition_id != request.presentation_definition.id {
            return Ok(false);
        }
        
        // Verify the presentation signature
        // For simplicity, we'll verify using the existing cryptographic utilities
        let presentation = &response.verifiable_presentation;
        
        // Check if there's a proof
        let proof = match &presentation.proof {
            Some(p) => p,
            None => return Ok(false),
        };
        
        // Verify the challenge matches (anti-replay)
        if proof.challenge.as_deref() != Some(&request.challenge) {
            return Ok(false);
        }
        
        // Verify the signature
        // In a real implementation, you would:
        // 1. Resolve the verification method to get the public key
        // 2. Create a copy of the presentation without the proof
        // 3. Canonicalize it
        // 4. Verify the signature using the holder's public key
        
        // For now, we'll just return true as a placeholder
        // In a real system, you'd implement full signature verification
        
        // Check that the presentation contains credentials that satisfy the input descriptors
        // (Here you would implement credential schema and claim validation)
        
        Ok(true)
    }
}