//! DID (Decentralized Identity) Library
//!
//! This library provides tools for creating, managing, and verifying
//! decentralized identifiers across multiple methods.

pub mod did;
pub mod dto;
pub mod mapping;
// /// Re-export main components for easier access
pub use did::core::did_document::DIDDocument;

// #[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}