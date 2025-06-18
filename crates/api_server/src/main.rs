mod api;
mod dto;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use config::{Config, ConfigError, Environment, File};
use did_library::did::core::{did_document::DIDCreationOptions, traits::DIDMethod};
use did_library::did::methods::key::handler::KeyDID;
use did_library::did::methods::ethr::handler::EthrHandler;
use did_library::did::methods::web::handler::Web;
use did_library::DIDDocument;
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

#[derive(Serialize)]
struct DIDOperationResponse {
    success: bool,
    message: String,
    document: Option<DIDDocument>,
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
    //TODO:: Investigate why we don't use this field
    issuer_did: String,
    expiration_date: Option<String>,
}

#[derive(Deserialize)]
//TODO: Implement this
struct CreateDIDRequest {
    // Optional method field, defaults to "key" if not provided
    method: Option<String>,
    //TODO:: Investigate why we don't use this field
    _key_type: Option<String>,
    network: Option<String>,
    chain_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDIDResponse {
    document: DIDDocument,
}

/// Handler for creating a new DID
/// TODO: Make so that it can handle/manage input from a user
async fn create_did_handler(req: web::Json<CreateDIDRequest>) -> impl Responder {
    // Path to the registry file

    // Get the method from the request, default to "key" if not specified
    let method = req.method.clone().unwrap_or_else(|| "key".to_string());

    // Set up DID creation options (same for both methods)
    let options = DIDCreationOptions {
        key_type: None,
        verification_method: None,
        authentication: None,
        assertion_method: None,
        key_agreement: None,
        capability_invocation: None,
        capability_delegation: None,
        service: None,
        network:  req.network.clone(),
        chain_id:  req.chain_id.clone(),
    };

    did_library::did::registry::init_registry(Some(env::var("DID_REGISTRY_PATH").unwrap()));

    // Create the document based on the selected method
    let document = match method.as_str() {
        "ethr" => {
            EthrHandler::create_did(options)
        },
        "key" => {
            KeyDID::create_did(options)
        }
        "web" => {
            Web::create_did(options)
        }
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Unsupported DID method: {}", method)
            }));
        }
    };
    // Return the DID document
    HttpResponse::Ok().json(document)
}

/// Handler for invalidating a DID
async fn invalidate_did_handler(path: web::Path<String>) -> impl Responder {
    let did = path.into_inner();
    
    // Determine which DID method to use based on the DID format
    let result: Result<DIDDocument, &str> = if did.starts_with("did:key:") {
        // Create a new KeyDID instance for key DIDs
        KeyDID::invalidate_did(&did)
    } else if did.starts_with("did:ethr:") {
        // Create a new EthrHandler instance for Ethereum DIDs
        EthrHandler::invalidate_did(&did)
    } else {
        // Unsupported DID method
        Err("Unsupported DID method: only did:key: and did:ethr: are supported")
    };
    
    match result {
        Ok(document) => HttpResponse::Ok().json(DIDOperationResponse {
            success: true,
            message: "DID successfully invalidated".to_string(),
            document: Some(document),
        }),
        Err(error) => HttpResponse::BadRequest().json(DIDOperationResponse {
            success: false,
            message: error.to_string(),
            document: None,
        }),
    }
}

async fn issue_credential_handler(req: web::Json<IssueCredentialRequest>) -> impl Responder {
    // Extract the name from the subject data
    let subject_data = req.subject.clone();

    // This is medicore solution, although it cleans up, we are using unwrap (an unsafe function that
    // shouldn't be used in production),
    // TODO:: add generic handlers for dealing with Result<> functions
    let name = subject_data
      .get("name")
      .ok_or_else(|| HttpResponse::BadRequest().body("Missing 'name' field"))
      .and_then(|val| val.as_str()
        .ok_or_else(|| HttpResponse::BadRequest().body("'name' must be a string"))
        .map(|s| s.to_string())).unwrap();

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
        credential_type: req.credential_type.clone(),
        expiration_date: req.expiration_date.clone(),
    };

    // Issue the credential
    match verifiable_credentials::issue_credential(request) {
        Ok(credential) => HttpResponse::Ok().json(credential),
        Err(e) => {
            // Check for specific DID-related errors and return appropriate HTTP status codes
            if e.contains("DID status is") && e.contains("but must be 'active'") {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "DID_DEACTIVATED",
                    "message": "Cannot issue credential: The DID has been deactivated and cannot be used to issue credentials.",
                    "detailed_error": e,
                    "error_type": "invalid_did_status"
                }))
            } else if e.contains("No DID found in registry") {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "NO_DID_FOUND",
                    "message": "No DID found in the registry. Please create a DID first.",
                    "detailed_error": e,
                    "error_type": "missing_did"
                }))
            } else if e.contains("DID document not found in registry") {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "DID_DOCUMENT_NOT_FOUND",
                    "message": "DID document could not be retrieved from the registry.",
                    "detailed_error": e,
                    "error_type": "missing_did_document"
                }))
            } else {
                // For other errors, return 500 Internal Server Error
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "CREDENTIAL_ISSUANCE_FAILED",
                    "message": "Failed to issue credential due to an internal error.",
                    "detailed_error": e,
                    "error_type": "internal_error"
                }))
            }
        }
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

    //TODO: Look into logging
    if let Ok(log_level) = env::var("RUST_LOG") {
        // RUST_LOG is set, use it
        println!("Log Level: {} ", log_level);
        env_logger::init();
    } else {
        // RUST_LOG is not set, use default level
        env_logger::Builder::new()
          .filter_level(log::LevelFilter::Debug)
          .init();
    }


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

        // TODO: Look into loggers
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
                        web::scope("/presentations")
                            .route("/create", web::post().to(create_presentation_handler))
                            .route("/request", web::post().to(create_presentation_request_handler))
                            .route("/verify", web::post().to(verify_presentation_handler))
                    )
                    .service(
                        web::scope("/did")
                            .route("/create", web::post().to(create_did_handler))
                            .route("/{did}/invalidate", web::post().to(invalidate_did_handler))
                    ),
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
