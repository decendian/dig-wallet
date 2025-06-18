use crate::did::core::did_document::DIDCreationOptions;
use crate::did::core::traits::DIDMethod;
use crate::DIDDocument;

pub struct Web;

impl Web {
}

impl DIDMethod for Web {
	fn create_did(_options: DIDCreationOptions) -> DIDDocument {
		todo!()
		}
	

	fn resolve_did(_did: &str) -> Result<DIDDocument, &'static str> {
		todo!()
	}

    fn update_did(
        _did: &str,
        _option: DIDCreationOptions,
    ) -> Result<DIDDocument, &'static str> {
        todo!()
    }
	
	fn invalidate_did(_did: &str) -> Result<DIDDocument, &'static str> {
		todo!()
	}
}