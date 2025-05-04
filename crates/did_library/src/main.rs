use did_library::did::core::did_document::DIDCreationOptions;
use did_library::did::core::traits::DIDMethod;
use did_library::did::methods::key::handler::KeyDID;

fn main() {

  let options = DIDCreationOptions {
    key_type: None,
    verification_method: None,
    authentication: None,
    assertion_method: None,
    key_agreement:None,
    capability_invocation: None,
    capability_delegation: None,
    service: None
  };
  

  print!("{:?}", KeyDID.create_did(options));

}
