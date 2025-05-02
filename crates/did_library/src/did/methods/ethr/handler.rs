use crate::did::core::did_document::DIDCreationOptions;
use crate::did::core::traits::DIDMethod;
use crate::DIDDocument;

pub struct EthrHandler;

impl EthrHandler {
	pub fn new() -> Self {
		Self {}
	}
}

impl DIDMethod for EthrHandler {
	fn create_did(&self, options: DIDCreationOptions) -> DIDDocument {
		todo!()
	}

	fn resolve_did(&self, did: &str) -> Result<DIDDocument, &'static str> {
		todo!()
	}

	fn update_did(&self, did: &str, option: DIDCreationOptions) -> Result<DIDDocument, &'static str> {
		todo!()
	}
	
}