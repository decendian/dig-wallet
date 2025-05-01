use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::format::VerifiableCredential;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PresentationDefinition {
    pub id: String,
    pub input_descriptors: Vec<InputDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_requirements: Option<Vec<SubmissionRequirement>>,
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputDescriptor {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub constraints: Constraints,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format_: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_disclosure: Option<String>, // "required" or "preferred"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    pub path: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<String>, // "required", "preferred"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmissionRequirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub rule: String, // "all" or "pick"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
    pub from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_nested: Option<Vec<SubmissionRequirement>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifiablePresentation {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    pub type_: Vec<String>,
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<VerifiableCredential>,
    pub holder: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_submission: Option<PresentationSubmission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<crate::format::Proof>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PresentationSubmission {
    pub id: String,
    pub definition_id: String,
    pub descriptor_map: Vec<DescriptorMapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescriptorMapping {
    pub id: String,
    pub format: String,
    pub path: String,
}

impl PresentationDefinition {
    pub fn new(id: String, input_descriptors: Vec<InputDescriptor>) -> Self {
        Self {
            id,
            input_descriptors,
            submission_requirements: None,
            context: Some(vec![
                "https://identity.foundation/presentation-exchange/submission/v1".to_string()
            ]),
        }
    }
    
    // Verify if a presentation submission satisfies this definition
    pub fn verify_submission(&self, vp: &VerifiablePresentation) -> Result<bool, String> {
        // Ensure the presentation has a submission
        let submission = match &vp.presentation_submission {
            Some(s) => s,
            None => return Err("Presentation does not contain a submission".to_string()),
        };
        
        // Check that the definition ID matches
        if submission.definition_id != self.id {
            return Err(format!(
                "Definition ID mismatch: expected {}, got {}", 
                self.id, submission.definition_id
            ));
        }
        
        // Validate each input descriptor has a corresponding mapping
        for descriptor in &self.input_descriptors {
            let mapping = submission.descriptor_map.iter()
                .find(|m| m.id == descriptor.id);
            
            if mapping.is_none() {
                return Err(format!("Missing mapping for input descriptor {}", descriptor.id));
            }
            
            // Further validation would go here, such as:
            // - Checking that the path resolves to a valid credential
            // - Verifying the credential meets the constraints
            // - etc.
        }
        
        // For a basic implementation, we'll just check that all descriptors are mapped
        Ok(true)
    }
}

impl VerifiablePresentation {
    pub fn new(
        id: String,
        verifiable_credentials: Vec<VerifiableCredential>,
        holder: String,
    ) -> Self {
        Self {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
                "https://identity.foundation/presentation-exchange/submission/v1".to_string(),
            ],
            id,
            type_: vec!["VerifiablePresentation".to_string()],
            verifiable_credential: verifiable_credentials,
            holder,
            presentation_submission: None,
            proof: None,
        }
    }
    
    pub fn add_submission(&mut self, submission: PresentationSubmission) {
        self.presentation_submission = Some(submission);
    }
}

impl PresentationSubmission {
    pub fn new(id: String, definition_id: String) -> Self {
        Self {
            id,
            definition_id,
            descriptor_map: Vec::new(),
        }
    }
    
    pub fn add_mapping(&mut self, id: String, format: String, path: String) {
        self.descriptor_map.push(DescriptorMapping {
            id,
            format,
            path,
        });
    }
}

// Helper function to create a basic presentation from a definition
pub fn create_presentation_for_definition(
    definition: &PresentationDefinition,
    credentials: Vec<VerifiableCredential>,
    holder: String,
) -> Result<VerifiablePresentation, String> {
    if credentials.is_empty() {
        return Err("No credentials provided".to_string());
    }
    
    // Create a new VP
    let mut vp = VerifiablePresentation::new(
        format!("urn:uuid:{}", uuid::Uuid::new_v4()),
        credentials.clone(),
        holder,
    );
    
    // Create a submission
    let mut submission = PresentationSubmission::new(
        format!("urn:uuid:{}", uuid::Uuid::new_v4()),
        definition.id.clone(),
    );
    
    // For this simple implementation, we're just mapping the first credential to each descriptor
    // In a real implementation, you'd need to match credentials with descriptors based on content
    for (i, descriptor) in definition.input_descriptors.iter().enumerate() {
        if i >= credentials.len() {
            return Err("Not enough credentials to satisfy all input descriptors".to_string());
        }
        
        submission.add_mapping(
            descriptor.id.clone(),
            "ldp_vp".to_string(),
            format!("$.verifiableCredential[{}]", i),
        );
    }
    
    vp.add_submission(submission);
    Ok(vp)
}