use super::format::VerifiableCredential;
use super::keys;

// Needs work: This function should sign the credential using the issuer's private key
// pub fn sign_credential(credential: &mut VerifiableCredential) -> Result<(), String> {
//     // Generate a proof (this is a placeholder, in a real implementation you would use cryptographic signing)
//     let proof = Proof {
//         type_: "Ed25519Signature2018".to_string(),
//         created: Utc::now().to_rfc3339(),
//         verification_method: credential.issuer.clone(),
//         proof_purpose: "assertionMethod".to_string(),
//         proof_value: "placeholder_signature".to_string(), // Placeholder for the actual signature
//     };
    
//     credential.proof = Some(proof);
//     Ok(())
// }
pub fn verify_credential(credential: &VerifiableCredential) -> Result<bool, String> {
    // Check if the credential has a proof
    if let Some(proof) = &credential.proof {
        // Check expiration date
        if let Some(expiration_date) = &credential.expiration_date {
            let expiration = chrono::DateTime::parse_from_rfc3339(expiration_date)
                .map_err(|e| format!("Invalid expiration date: {}", e))?;
            
            let now = chrono::Utc::now();
            if expiration < now {
                return Ok(false);
            }
        }
        
        // Create a copy without the proof for signature verification
        let mut credential_for_verification = credential.clone();
        credential_for_verification.proof = None;
        
        // Canonicalize
        let canonical = serde_json::to_string(&credential_for_verification)
            .map_err(|e| format!("Serialization error: {}", e))?;
        
        // For demo, we use the same keypair for verification
        // In a real implementation, you'd resolve the DID to get the public key
        let keypair = keys::get_demo_keypair();
        let public_key = &keypair.public;
        
        // Verify the signature
        keys::verify_signature(canonical.as_bytes(), &proof.proof_value, public_key)
    } else {
        Err("Credential has no proof".to_string())
    }
}