#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod certificate_revocation_contract {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    use scale::{Decode, Encode};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, PackedLayout, SpreadLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            ink_storage::traits::StorageLayout,
            ink_storage::traits::StorageDebugInfo,
        )
    )]
    pub struct Certificate {
        pub issuer: AccountId,
        pub subject: String,
        pub public_key: Vec<u8>,
        pub expiration_date: u64,
        pub hash: [u8; 32],
        pub is_revoked: bool,
    }

    #[ink(storage)]
    pub struct CertificateRevocationContract {
        issuer: AccountId,
        certificates: StorageHashMap<[u8; 32], Certificate>,
        authorized_parties: StorageHashMap<AccountId, ()>,
    }

    impl CertificateRevocationContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                issuer: Self::env().caller(),
                certificates: Default::default(),
                authorized_parties: Default::default(),
            }
        }

        #[ink(message)]
        pub fn revoke_certificate(&mut self, certificate_hash: [u8; 32]) -> bool {
            let sender = self.env().caller();
            let certificate = self.certificates.get_mut(&certificate_hash);

            if let Some(certificate) = certificate {
                if certificate.issuer != sender && !self.authorized_parties.contains_key(&sender) {
                    return false;
                }

                certificate.is_revoked = true;
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn is_certificate_revoked(&self, certificate_hash: [u8; 32]) -> bool {
            let certificate = self.certificates.get(&certificate_hash);

            match certificate {
                Some(certificate) => certificate.is_revoked,
                None => true, // Certificate is considered revoked if it doesn't exist
            }
        }

        #[ink(message)]
        pub fn add_authorized_party(&mut self, authorized_party: AccountId) -> bool {
            let sender = self.env().caller();

            if sender != self.issuer {
                return false;
            }

            self.authorized_parties.insert(authorized_party, ());
            true
        }

        #[ink(message)]
        pub fn remove_authorized_party(&mut self, authorized_party: AccountId) -> bool {
            let sender = self.env().caller();

            if sender != self.issuer {
                return false;
            }

            self.authorized_parties.take(&authorized_party);
            true
        }

        #[ink(message)]
        pub fn transfer_issuer(&mut self, new_issuer: AccountId) -> bool {
            let sender = self.env().caller();

            if sender != self.issuer {
                return false;
            }

            self.issuer = new_issuer;
            true
        }
    }
}
