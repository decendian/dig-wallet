use crate::did::core::did_document::DIDDocument;
use crate::did::core::traits::DIDMethod;

pub struct KeyDID;

impl KeyDID {
    pub fn new() -> Self {
        Self {}
    }
}

impl DIDMethod for KeyDID {

    fn creat_did(&self) -> DIDDocument {
        todo!()
    }

    fn resolve_did(&self) -> String {
        todo!()
    }

    fn update_did(&self) -> bool {
        todo!()
    }
}