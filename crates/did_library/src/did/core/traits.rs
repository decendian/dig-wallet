use ssi::dids::DID;
use crate::did::core::did_document::{DIDDocument, VerificationMethod};

pub trait DIDMethod {
    fn create_did() -> DIDDocument;
    fn resolve_did(did: &str) -> Result<DIDDocument, &'static str>;
    fn update_did(did: &str, verification_method: Option<VerificationMethod>) -> String;
}
