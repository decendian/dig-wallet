use crate::did::core::did_document::DIDCreationOptions;
use crate::did::core::traits::DIDMethod;
use crate::DIDDocument;

pub struct Web;

impl Web {
}

impl DIDMethod for Web {
	fn create_did(options: DIDCreationOptions) -> DIDDocument {
		todo!()
		}
	

	fn resolve_did(did: &str) -> Result<DIDDocument, &'static str> {
		todo!()
	}

    fn update_did(
        did: &str,
        option: DIDCreationOptions,
    ) -> Result<DIDDocument, &'static str> {
        todo!()
    }
	
	fn invalidate_did(did: &str) -> Result<DIDDocument, &'static str> {
		todo!()
	}
}