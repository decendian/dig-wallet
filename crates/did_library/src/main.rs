use did_library::did::core::did_document::DIDCreationOptions;
use did_library::did::core::traits::DIDMethod;
use did_library::did::methods::key::handler::KeyDID;
use std::env;

fn main() {
    // Print current working directory
    match env::current_dir() {
        Ok(path) => println!("Current working directory: {:?} + <><>< ", path),
        Err(e) => println!("Failed to get current directory: {}", e),
    }

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

    //TODO: Dynamically load and configure path for registry initialization (from .env file)
    did_library::did::registry::init_registry(Some(
        "did_library/resources/did_registry.json".to_string(),
    ));

    let document = KeyDID.create_did(options);
}
