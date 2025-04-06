use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::verifiable_credentials::{CredentialRequest, issue_credential};

#[derive(Deserialize)]
pub struct IssueCredentialRequest {
    pub subject: serde_json::Value,
    pub credential_type: Vec<String>,
    pub issuer_did: String,
    pub expiration_date: Option<String>,
}

pub async fn issue_credential_handler(
    req: web::Json<IssueCredentialRequest>,
) -> impl Responder {
    // Convert the request to our internal format
    let subject_data = req.subject.clone();
    
    // Extract the name from the subject data
    let name = match subject_data.get("name") {
        Some(name) => match name.as_str() {
            Some(s) => s.to_string(),
            None => return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Subject name must be a string"
            })),
        },
        None => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Subject must include a name"
        })),
    };
    
    // Create the credential request
    let request = CredentialRequest {
        subject: crate::verifiable_credentials::CredentialSubject {
            id: subject_data.get("id").and_then(|id| id.as_str()).map(String::from),
            name,
            attributes: subject_data,
        },
        type_: req.credential_type.clone(),
        issuer_did: req.issuer_did.clone(),
        expiration_date: req.expiration_date.clone(),
    };
    
    // Issue the credential
    match issue_credential(request) {
        Ok(credential) => HttpResponse::Ok().json(credential),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}