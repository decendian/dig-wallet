// tests/keys_tests.rs
use verifiable_credentials::keys::*;

#[cfg(test)]
mod keys_tests {
    use super::*;

    #[test]
    fn test_generate_random_keypair() {
        let keypair1 = generate_random_keypair();
        let keypair2 = generate_random_keypair();
        
        // Different keypairs should have different public keys
        assert_ne!(keypair1.public.to_bytes(), keypair2.public.to_bytes());
    }

    #[test]
    fn test_demo_keypair_consistency() {
        let keypair1 = get_demo_keypair();
        let keypair2 = get_demo_keypair();
        
        // Demo keypair should be consistent across calls
        assert_eq!(keypair1.public.to_bytes(), keypair2.public.to_bytes());
        assert_eq!(keypair1.secret.to_bytes(), keypair2.secret.to_bytes());
    }

    #[test]
    fn test_sign_and_verify_data() {
        let keypair = get_demo_keypair();
        let test_data = b"Hello, World!";
        
        let signature = sign_data(test_data, &keypair);
        let is_valid = verify_signature(test_data, &signature, &keypair.public).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_verify_invalid_signature() {
        let keypair = get_demo_keypair();
        let test_data = b"Hello, World!";
        let invalid_signature = "invalid_base64_signature";
        
        let result = verify_signature(test_data, invalid_signature, &keypair.public);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_wrong_data() {
        let keypair = get_demo_keypair();
        let original_data = b"Hello, World!";
        let tampered_data = b"Hello, Hacker!";
        
        let signature = sign_data(original_data, &keypair);
        let is_valid = verify_signature(tampered_data, &signature, &keypair.public).unwrap();
        
        assert!(!is_valid);
    }

    #[test]
    fn test_sign_empty_data() {
        let keypair = get_demo_keypair();
        let empty_data = b"";
        
        let signature = sign_data(empty_data, &keypair);
        let is_valid = verify_signature(empty_data, &signature, &keypair.public).unwrap();
        
        assert!(is_valid);
    }
}