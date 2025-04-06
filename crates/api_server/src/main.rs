use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use verifiable_credentials::{self, CredentialRequest, CredentialSubject};

#[derive(Deserialize)]
struct IssueCredentialRequest {
    subject: serde_json::Value,
    credential_type: Vec<String>,
    issuer_did: String,
    expiration_date: Option<String>,
}

async fn issue_credential_handler(
    req: web::Json<IssueCredentialRequest>,
) -> impl Responder {
    // Extract the name from the subject data
    let subject_data = req.subject.clone();
    
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
        subject: CredentialSubject {
            id: subject_data.get("id").and_then(|id| id.as_str()).map(String::from),
            name,
            attributes: subject_data,
        },
        type_: req.credential_type.clone(),
        issuer_did: req.issuer_did.clone(),
        expiration_date: req.expiration_date.clone(),
    };
    
    // Issue the credential
    match verifiable_credentials::issue_credential(request) {
        Ok(credential) => HttpResponse::Ok().json(credential),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/credentials")
                            .route("/issue", web::post().to(issue_credential_handler))
                    )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}