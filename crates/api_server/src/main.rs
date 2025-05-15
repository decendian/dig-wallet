use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use config::{Config, ConfigError, Environment, File};
use did_library::did::core::{did_document::DIDCreationOptions, traits::DIDMethod};
use did_library::did::methods::key::handler::KeyDID;
use did_library::DIDDocument;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use verifiable_credentials::{self, CredentialRequest, CredentialSubject};

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
    config_builder =
        config_builder.add_source(Environment::with_prefix("API_SERVER").separator("_"));

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

#[derive(Deserialize)]
//TODO: Implement this
struct CreateDIDRequest {
    
}

#[derive(Serialize, Deserialize)]
pub struct CreateDIDResponse {
    document: DIDDocument,
}

/// Handler for creating a new DID
/// TODO: Make so that it can handle/manage input from a user
async fn create_did_handler(req: web::Json<CreateDIDRequest>) -> impl Responder {

    // Path to the registry file
    let registry_path = env::var("DID_REGISTRY_PATH").unwrap();

    // Check if the registry file exists and has content
    if let Ok(metadata) = std::fs::metadata(registry_path) {
        if metadata.len() > 0 {
            // Registry exists and has content
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "DID registry already exists. No new DID will be created."
            }));
        }
    }

    // If we get here, either the file doesn't exist or is empty
    // Create a new KeyDID instance
    let did_method = KeyDID::new();

    // Set up DID creation options
    let options = DIDCreationOptions {
        key_type: None,
        verification_method: None,
        authentication: None,
        assertion_method: None,
        key_agreement: None,
        capability_invocation: None,
        capability_delegation: None,
        service: None,
    };

    did_library::did::registry::init_registry(Some(env::var("DID_REGISTRY_PATH").unwrap()));
    let document = did_method.create_did(options);

    // Return the DID document
    HttpResponse::Ok().json(document)

}

async fn issue_credential_handler(req: web::Json<IssueCredentialRequest>) -> impl Responder {
    // Extract the name from the subject data
    let subject_data = req.subject.clone();

    let name = match subject_data.get("name") {
        Some(name) => match name.as_str() {
            Some(s) => s.to_string(),
            None => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Subject name must be a string"
                }))
            }
        },
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Subject must include a name"
            }))
        }
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
        name,                             // We already extracted and validated this above
        attributes: subject_data.clone(), // Store all subject data for flexibility
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
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("Error loading configuration: {}. Using defaults.", e);
        ServerConfig::default()
    });

    println!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        // TODO: Look into loggers later
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/credentials")
                            .route("/issue", web::post().to(issue_credential_handler)),
                    )
                    .service(
                        web::scope("/did").route("/create", web::post().to(create_did_handler)),
                    ),
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
