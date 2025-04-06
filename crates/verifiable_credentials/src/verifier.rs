use super::format::VerifiableCredential;


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
    // In a real implementation, you would:
    // 1. Extract the proof from the credential
    // 2. Remove the proof from the credential
    // 3. Serialize the credential without the proof
    // 4. Hash the serialized data
    // 5. Verify the signature using the issuer's public key
    
    // This is a simplified example
    if let Some(proof) = &credential.proof {
        // Verify the proof is valid
        // In a real implementation, you would verify the signature
        
        // Check if the credential has expired
        if let Some(expiration_date) = &credential.expiration_date {
            let expiration = chrono::DateTime::parse_from_rfc3339(expiration_date)
                .map_err(|e| format!("Invalid expiration date: {}", e))?;
            
            let now = chrono::Utc::now();
            if expiration < now {
                return Ok(false);
            }
        }
        
        // All checks passed
        Ok(true)
    } else {
        Err("Credential has no proof".to_string())
    }
}