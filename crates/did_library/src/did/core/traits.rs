use crate::did::core::did_document::DIDDocument;

pub trait DIDMethod {
    fn creat_did(&self) -> DIDDocument;
    fn resolve_did(&self) -> String;
    fn update_did(&self) -> bool;
}
