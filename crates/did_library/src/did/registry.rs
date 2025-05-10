pub mod did_storage;

use did_storage::DIDRegistry;
use std::sync::Once;
use std::sync::OnceLock; // Use your local DIDRegistry implementation

static REGISTRY: OnceLock<DIDRegistry> = OnceLock::new();
static INIT: Once = Once::new();

/// Initialize the global DID registry
pub fn init_registry(storage_path: Option<String>) {
    INIT.call_once(|| {
        let _ = REGISTRY.set(DIDRegistry::new(storage_path));
    });
}

/// Get a reference to the global DID registry
pub fn get_registry() -> &'static DIDRegistry {
    REGISTRY.get_or_init(|| DIDRegistry::new(None))
}
