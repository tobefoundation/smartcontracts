#![cfg_attr(not(feature = "std"), no_std)]


#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
pub enum CertificateOwner {
    None,
    Account(AccountId),
    Contract(ContractId),
}

impl CertificateOwner {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    pub fn is_account(&self) -> bool {
        match self {
            Self::Account(_) => true,
            _ => false,
        }
    }

    pub fn is_contract(&self) -> bool {
        match self {
            Self::Contract(_) => true,
            _ => false,
        }
    }

    pub fn as_account(&self) -> Option<AccountId> {
        match self {
            Self::Account(account_id) => Some(*account_id),
            _ => None,
        }
    }

    pub fn as_contract(&self) -> Option<ContractId> {
        match self {
            Self::Contract(contract_id) => Some(*contract_id),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
pub struct Certificate {
    owner: CertificateOwner,
    issuer: AccountId,
    subject: AccountId,
    expiration_date: u64,
    signature: Vec<u8>,
}

impl Certificate {
    pub fn new(
        owner: CertificateOwner,
        issuer: AccountId,
        subject: AccountId,
        expiration_date: u64,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            owner,
            issuer,
            subject,
            expiration_date,
            signature,
        }
    }

    pub fn transfer(&mut self, new_owner: CertificateOwner) {
        assert!(!new_owner.is_none(), "Invalid new owner");

        self.owner = new_owner;
    }
}

#[ink::contract]
pub mod certificate_ownership_contract {
    use super::*;

    #[ink(storage)]
    pub struct CertificateOwnershipContract {
        certificates: ink_storage::collections::HashMap<Hash, Certificate>,
    }

    impl CertificateOwnershipContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                certificates: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn transfer_certificate(&mut self, certificate_hash: Hash, new_owner: CertificateOwner) {
            let mut certificate = self.get_certificate(&certificate_hash).unwrap();
            assert!(
                certificate.owner.as_account() == Some(env.caller()),
                "Only the current owner can transfer the certificate"
            );

            certificate.transfer(new_owner);
        }

        #[ink(message)]
        pub fn get_certificate_owner(&self, certificate_hash: Hash) -> CertificateOwner {
            let certificate = self.get_certificate(&certificate_hash).unwrap();

            certificate.owner
        }

        #[ink(message)]
        pub fn validate_certificate_owner(&self, certificate_hash: Hash, expected_owner: CertificateOwner) -> bool {
            let certificate = self.get_certificate(&certificate_hash).unwrap();

            certificate.owner == expected_owner
        }

        fn get_certificate(&self, certificate_hash: &Hash) -> Option<Certificate> {
            self.certificates.get(certificate_hash).cloned()
        }
    }
