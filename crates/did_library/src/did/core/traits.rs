use ssi::dids::DID;
use ssi::dids::registration::DIDDocumentOperation;
use crate::did::core::did_document::{DIDCreationOptions, DIDDocument, VerificationMethod};
use crate::did::core::key_utils::KeyType;

pub trait DIDMethod {
    fn  create_did(&self, options: DIDCreationOptions)  -> DIDDocument;
    fn resolve_did(&self, did: &str) -> Result<DIDDocument, &'static str>;
    fn update_did(&self, did: &str, option: DIDCreationOptions) -> Result<DIDDocument, &'static str>;
}

