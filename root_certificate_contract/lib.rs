#![cfg_attr(not(feature = "std"), no_std)]

#[derive(Default, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct RootCertificate {
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[ink::contract]
mod root_certificate_contract {
    use super::*;

    #[ink(storage)]
    pub struct RootCertificateManagement {
        root_certificate: RootCertificate,
        owner: AccountId,
    }

    impl RootCertificateManagement {
        #[ink(constructor)]
        pub fn new(root_certificate: RootCertificate) -> Self {
            Self {
                root_certificate,
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn update_root_certificate(&mut self, new_root_certificate: RootCertificate) {
            self.ensure_owner();
            self.root_certificate = new_root_certificate;
        }

        #[ink(message)]
        pub fn get_root_certificate(&self) -> RootCertificate {
            self.root_certificate.clone()
        }

        fn ensure_owner(&self) {
            assert_eq!(self.owner, self.env().caller());
        }
    }
}