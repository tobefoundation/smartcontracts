#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod certificate_storage_contract {
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
        traits::{PackedLayout, SpreadLayout},
    };
    
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
    pub struct Certificate {
        pub issuer: AccountId,
        pub subject: AccountId,
        pub public_key: Vec<u8>,
        pub digital_signature: Vec<u8>,
        pub metadata: Vec<u8>,
    }
    
    #[ink(storage)]
    pub struct CertificateStorage {
        certificates: StorageHashMap<Hash, Certificate>,
        total_certificates: Lazy<u64>,
    }
    
    impl CertificateStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                certificates: StorageHashMap::new(),
                total_certificates: Lazy::new(0),
            }
        }
        
        #[ink(message)]
        pub fn store_certificate(&mut self, certificate_hash: Hash, certificate: Certificate) -> bool {
            if self.certificates.insert(certificate_hash, certificate).is_none() {
                *self.total_certificates += 1;
                true
            } else {
                false
            }
        }
        
        #[ink(message)]
        pub fn get_certificate(&self, certificate_hash: Hash) -> Option<Certificate> {
            self.certificates.get(&certificate_hash).cloned()
        }
        
        #[ink(message)]
        pub fn total_certificates(&self) -> u64 {
            *self.total_certificates
        }
    }
}
