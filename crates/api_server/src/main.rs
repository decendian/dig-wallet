use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use verifiable_credentials::{self, CredentialRequest, CredentialSubject};

#[derive(Deserialize)]
struct CreatePresentationRequest {
    holder_did: String,
    credentials: Vec<verifiable_credentials::format::VerifiableCredential>,
    challenge: Option<String>,
    domain: Option<String>,
}

#[derive(Deserialize)]
struct PresentationDefinitionRequest {
    credential_types: Vec<String>,
    fields: Vec<(String, bool)>, // (field path, is optional)
    purpose: Option<String>,
}

#[derive(Deserialize)]
struct VerifyPresentationRequest {
    original_request: verifiable_credentials::presentation::exchange::PresentationRequest,
    presentation_response: verifiable_credentials::presentation::exchange::PresentationResponse,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

// Function to load configuration from files and environment variables
fn load_config() -> Result<ServerConfig, ConfigError> {
    let mut config_builder = Config::builder();
    
    // First, try to load from a default config file
    if let Ok(config_path) = env::var("CONFIG_PATH") {
        config_builder = config_builder.add_source(File::with_name(&config_path));
    } else {
        // Try default locations
        config_builder = config_builder
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/config").required(false));
    }
    
    // Then, override with environment-specific config if it exists
    if let Ok(env_name) = env::var("RUN_ENV") {
        config_builder = config_builder
            .add_source(File::with_name(&format!("config/{}", env_name)).required(false));
    }
    
    // Finally, override with environment variables (API_SERVER_HOST, API_SERVER_PORT)
    config_builder = config_builder
        .add_source(Environment::with_prefix("API_SERVER").separator("_"));
    
    // Build and deserialize the configuration
    let config = config_builder.build()?;
    let server_config: ServerConfig = config.try_deserialize().unwrap_or_default();
    
    Ok(server_config)
}

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
    
    // Extract subject ID if present, converting it to an Option<String>
    // We extract this separately for easier access, even though it's already in attributes
    let subject_id = subject_data
        .get("id")
        .and_then(|id| id.as_str())
        .map(String::from);

    // Create the credential subject with the extracted fields and all attributes
    let credential_subject = CredentialSubject {
        id: subject_id,
        name,  // We already extracted and validated this above
        attributes: subject_data.clone(),  // Store all subject data for flexibility
    };

    // Create the final credential request
    let request = CredentialRequest {
        subject: credential_subject,
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

// Add these new handlers
async fn create_presentation_handler(
    req: web::Json<CreatePresentationRequest>,
) -> impl Responder {
    // Create a presentation from the provided credentials
    let mut presentation = verifiable_credentials::presentation::create_presentation(
        req.holder_did.clone(),
        req.credentials.clone(),
        req.challenge.clone(),
        req.domain.clone(),
    );
    
    // Sign the presentation
    match verifiable_credentials::presentation::sign_presentation(
        &mut presentation,
        req.challenge.clone(),
        req.domain.clone(),
    ) {
        Ok(_) => HttpResponse::Ok().json(presentation),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn create_presentation_request_handler(
    req: web::Json<PresentationDefinitionRequest>,
) -> impl Responder {
    // Create a presentation request
    let request = verifiable_credentials::presentation::exchange::create_request(
        req.credential_types.clone(),
        req.fields.clone(),
        req.purpose.clone(),
    );
    
    HttpResponse::Ok().json(request)
}

async fn verify_presentation_handler(
    req: web::Json<VerifyPresentationRequest>,
) -> impl Responder {
    // Verify the presentation
    match verifiable_credentials::presentation::exchange::verify_response(
        &req.original_request,
        &req.presentation_response,
    ) {
        Ok(is_valid) => HttpResponse::Ok().json(serde_json::json!({
            "is_valid": is_valid
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists
    dotenv().ok();
    
    // Set up logging
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // Load configuration
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading configuration: {}. Using defaults.", e);
            ServerConfig::default()
        }
    };
    
    println!("Starting server at http://{}:{}", config.host, config.port);

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
                    .service(
                        web::scope("/presentations")
                            .route("/create", web::post().to(create_presentation_handler))
                            .route("/request", web::post().to(create_presentation_request_handler))
                            .route("/verify", web::post().to(verify_presentation_handler))
                    )
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}