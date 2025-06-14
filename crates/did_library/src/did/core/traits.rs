use crate::did::core::did_document::{DIDCreationOptions, DIDDocument};

pub trait DIDMethod {
    fn create_did(options: DIDCreationOptions) -> DIDDocument;
    fn resolve_did(did: &str) -> Result<DIDDocument, &'static str>;
    fn update_did(
        did: &str,
        option: DIDCreationOptions,
    ) -> Result<DIDDocument, &'static str>;
    fn invalidate_did(did: &str) -> Result<DIDDocument, &'static str>;
}
