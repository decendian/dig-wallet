// src/did/registry/did_storage.rs

use crate::did::core::did_document::DIDDocument;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

/// A storage system for DID documents
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DIDRegistry {
    #[serde(skip)]
    storage: Arc<Mutex<HashMap<String, DIDDocument>>>,
    storage_path: Option<String>,
}

impl DIDRegistry {
    /// Create a new DID registry with optional persistence path
    pub fn new(storage_path: Option<String>) -> Self {
        let registry = DIDRegistry {
            storage: Arc::new(Mutex::new(HashMap::new())),
            storage_path,
        };
        
        // Load from disk if path is provided
        if let Some(path) = &registry.storage_path {
            // Don't unwrap here - just log errors
            if let Err(e) = registry.load_from_disk(path) {
                eprintln!("Warning: Failed to load registry from disk: {}", e);
            }
        }
        
        registry
    }
    
    /// Store a DID document in the registry
    pub fn store(&self, did_document: DIDDocument) -> Result<(), &'static str> {
        // Clone the ID before locking to minimize lock time
        let did_id = did_document.id.clone();
        
        // Scope the lock to ensure it's released before file I/O
        {
            let mut storage = self.storage.lock().map_err(|_| "Failed to acquire lock on storage")?;
            storage.insert(did_id, did_document);
        }
        
        // Persist to disk if path is provided (outside the lock scope)
        if let Some(path) = &self.storage_path {
            self.save_to_disk(path)?;
        }
        
        Ok(())
    }
    
    /// Retrieve a DID document from the registry
    pub fn get(&self, did: &str) -> Result<Option<DIDDocument>, &'static str> {
        let storage = self.storage.lock().map_err(|_| "Failed to acquire lock on storage")?;
        Ok(storage.get(did).cloned())
    }
    
    /// List all DIDs in the registry
    pub fn list_dids(&self) -> Result<Vec<String>, &'static str> {
        let storage = self.storage.lock().map_err(|_| "Failed to acquire lock on storage")?;
        Ok(storage.keys().cloned().collect())
    }
    
    /// Save registry to disk
    fn save_to_disk(&self, path: &str) -> Result<(), &'static str> {
        // Add debug prints
        println!("Attempting to save registry to disk at path: {}", path);
        
        // Clone the data while holding the lock, to minimize lock time
        let serializable: HashMap<String, DIDDocument> = {
            let storage = self.storage.lock().map_err(|_| {
                println!("Failed to acquire lock on storage");
                "Failed to acquire lock on storage"
            })?;
            println!("Number of DIDs to save: {}", storage.len());
            storage.clone()
        };
        
        // Ensure directory exists
        if let Some(parent) = Path::new(path).parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => println!("Directory created/verified: {:?}", parent),
                Err(e) => {
                    println!("Failed to create directory: {:?}: {}", parent, e);
                    return Err("Failed to create directory");
                }
            }
        }
        
        // Serialize and save - done outside the lock
        let json = match serde_json::to_string(&serializable) {
            Ok(j) => {
                println!("Successfully serialized registry data, {} bytes", j.len());
                j
            },
            Err(e) => {
                println!("Failed to serialize registry: {}", e);
                return Err("Failed to serialize registry");
            }
        };
        
        match fs::write(path, &json) {
            Ok(_) => println!("Successfully wrote registry to disk at: {}", path),
            Err(e) => {
                println!("Failed to write registry to disk: {}", e);
                return Err("Failed to write registry to disk");
            }
        }
        
        Ok(())
    }
    
    /// Load registry from disk
    fn load_from_disk(&self, path: &str) -> Result<(), &'static str> {
        if !Path::new(path).exists() {
            return Ok(());  // No file yet, start with empty registry
        }
        // Read file outside of lock
        let json = fs::read_to_string(path).map_err(|_| "Failed to read registry from disk")?;
        let loaded: HashMap<String, DIDDocument> = serde_json::from_str(&json)
            .map_err(|_| "Failed to deserialize registry")?;
        // Update storage with minimal lock time
        let mut storage = self.storage.lock().map_err(|_| "Failed to acquire lock on storage")?;
        *storage = loaded;
        
        Ok(())
    }
}