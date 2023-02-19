#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod CertificateIssuance {
    use ink_env::hash::{CryptoHash, Sha3_256};
    use ink_prelude::vec::Vec;
    use ink::ink_prelude::borrow::ToOwned;   

    #[derive(Debug)]
    pub struct Certificate {
        pub subject: String,
        pub public_key: String,
        pub expiration_date: String,
        pub issuer: String,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct CertificateIssuanceContract {
        certificates: StorageHashMap<Vec<u8>, Certificate>,
    }

    impl CertificateIssuanceContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                certificates: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn issue_certificate(
            &mut self,
            certificate_subject: String,
            certificate_public_key: String,
            certificate_expiration_date: String,
            certificate_issuer: String,
            verification_code: String,
            sender_address: AccountId,
            sender_signature: Vec<u8>,
            timestamp: u64,
        ) -> Result<(), String> {
            // Verify the digital signature of the sender
            // let is_valid_signature = self.env().verify_signature(
            //     &sender_address.into(),
            //     &sender_signature,
            //     &self.env().account_id().as_ref(),
            // );
            // if !is_valid_signature {
            //     return Err("Invalid sender_signature".to_owned());
            // }

            // // Verify the domain ownership or entity identity using the verification code
            // let is_valid_verification = self.verify_verification_code(&certificate_subject, &verification_code);
            // if !is_valid_verification {
            //     return Err("Invalid verification_code".to_owned());
            // }

            // Generate the certificate
            let certificate = Certificate {
                subject: certificate_subject,
                public_key: certificate_public_key,
                expiration_date: certificate_expiration_date,
                issuer: certificate_issuer,
            };

            // Store the certificate on the blockchain
            let certificate_hash = self.compute_certificate_hash(&certificate);
            self.certificates.insert(certificate_hash, certificate);

            Ok(())
        }

        #[ink(message)]
        pub fn verify_certificate(
            &self,
            certificate_hash: Vec<u8>,
            certificate_issuer: String,
        ) -> Result<bool, String> {
            use ink::ink_prelude::borrow::ToOwned;
            // Retrieve the certificate from the storage
            let certificate_option = self.certificates.get(&certificate_hash);
            if let Some(certificate) = certificate_option {
                // Check if the certificate is issued by the specified issuer
                if certificate.issuer == certificate_issuer {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            } else {
                return Err("Certificate not found".to_owned());
            }
        }

        fn verify_verification_code(&self, certificate_subject: &str, verification_code: &str) -> bool {
            // TODO: Implement domain ownership or entity identity verification using the verification code
            true
        }

        fn compute_certificate_hash(&self, certificate: &Certificate) -> Vec<u8> {
//            let encoded_certificate = certificate.encode();
            let mut hasher = Sha3_256::default();
            hasher.write(&certificate);
            hasher.finish().to_vec()
        }
    }
}


// #[ink::test]
// fn issue_certificate() {
//     // Arrange
//     let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
//     let mut contract = CertificateIssuance::new();

//     let issuer = accounts.alice;
//     let subject = accounts.bob;
//     let expiration = 100u32;

//     // Act
//     let result = contract.issue_certificate(issuer, subject, expiration);

//     // Assert
//     assert_eq!(result, true);
// }

// #[ink::test]
// fn get_certificate_expiration() {
//     // Arrange
//     let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
//     let mut contract = CertificateIssuance::new();

//     let issuer = accounts.alice;
//     let subject = accounts.bob;
//     let expiration = 100u32;

//     // Issue certificate
//     contract.issue_certificate(issuer, subject, expiration);

//     // Act
//     let result = contract.get_certificate_expiration(subject);

//     // Assert
//     assert_eq!(result, Some(expiration));
// }

// #[ink::test]
// fn get_certificate_issuer() {
//     // Arrange
//     let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
//     let mut contract = CertificateIssuance::new();

//     let issuer = accounts.alice;
//     let subject = accounts.bob;
//     let expiration = 100u32;

//     // Issue certificate
//     contract.issue_certificate(issuer, subject, expiration);

//     // Act
//     let result = contract.get_certificate_issuer(subject);

//     // Assert
//     assert_eq!(result, Some(issuer));
// }

// #[ink::test]
// fn revoke_certificate() {
//     // Arrange
//     let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
//     let mut contract = CertificateIssuance::new();

//     let issuer = accounts.alice;
//     let subject = accounts.bob;
//     let expiration = 100u32;

//     // Issue certificate
//     contract.issue_certificate(issuer, subject, expiration);

//     // Act
//     let result = contract.revoke_certificate(subject);

//     // Assert
//     assert_eq!(result, true);
// }