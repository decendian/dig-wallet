use crate::did::core::did_document::DIDCreationOptions;
use crate::did::core::traits::DIDMethod;
use crate::DIDDocument;

pub struct Web;

impl Web {
	pub fn new() -> Self {
		Self {}
	}
}

impl DIDMethod for Web {
	fn create_did(&self, options: DIDCreationOptions) -> DIDDocument {
		todo!()
		}
	

	fn resolve_did(&self, did: &str) -> Result<DIDDocument, &'static str> {
		todo!()
	}

    fn update_did(
        &self,
        did: &str,
        option: DIDCreationOptions,
    ) -> Result<DIDDocument, &'static str> {
        todo!()
    }
	
	fn invalidate_did(&self, did: &str) -> Result<DIDDocument, &'static str> {
		todo!()
	}
}