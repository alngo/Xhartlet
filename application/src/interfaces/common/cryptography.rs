use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait Cryptography {
    async fn hash(&self, password: &str) -> Result<String, CryptographyError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, CryptographyError>;
}

#[derive(Debug)]
pub struct CryptographyError;
