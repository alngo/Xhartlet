use async_trait::async_trait;
use xhartlet_application::common::abstract_cryptography::{Cryptography as AbstractCryptography, CryptographyError as Error};

pub struct Cryptography;

#[async_trait(?Send)]
impl AbstractCryptography for Cryptography {
    async fn hash(&self, password: &str) -> Result<String, Error> {
        Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap())
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, Error> {
        Ok(bcrypt::verify(password, hash).unwrap())
    }
}
