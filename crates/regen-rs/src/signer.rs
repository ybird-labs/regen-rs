use std::str::FromStr;

use crate::error::{RegenError, Result};
use bip32::{DerivationPath, XPrv};
use bip39::Mnemonic;
use cosmrs::crypto::PublicKey;
use cosmrs::crypto::secp256k1::Signature;
use cosmrs::{AccountId, crypto::secp256k1::SigningKey};

pub struct Signer {
    private_key: SigningKey,
    public_key: PublicKey,
    address: String,
    chain_id: String,
}

impl std::fmt::Debug for Signer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signer")
            .field("address", &self.address)
            .field("chain_id", &self.chain_id)
            .field("private_key", &"[REDACTED]")
            .finish()
    }
}

impl Signer {
    pub fn from_mnemonic(
        mnemonic_str: &str,
        derivation_path: &str,
        chain_id: &str,
    ) -> Result<Self> {
        let path = derivation_path;
        let bip32_path = DerivationPath::from_str(path)
            .map_err(|e| RegenError::InvalidDerivationPath(e.to_string()))?;
        let mnemonic = Mnemonic::parse(mnemonic_str).map_err(RegenError::Mnemonic)?;
        let seed = mnemonic.to_seed("");

        let derived_key =
            XPrv::derive_from_path(seed, &bip32_path).map_err(RegenError::KeyDerivation)?;
        let private_key = SigningKey::from_slice(&derived_key.private_key().to_bytes())
            .map_err(RegenError::Cosmos)?;
        let public_key = private_key.public_key();
        let account_id = public_key.account_id(chain_id)?;
        let address = account_id.to_string();

        Ok(Self {
            private_key,
            public_key,
            address,
            chain_id: chain_id.to_string(),
        })
    }

    pub fn from_private_key(private_key_bytes: &[u8], chain_id: &str) -> Result<Self> {
        let private_key = SigningKey::from_slice(private_key_bytes)?;
        let public_key = private_key.public_key();
        let account_id = public_key.account_id(chain_id)?;
        let address = account_id.to_string();

        Ok(Self {
            private_key,
            public_key,
            address,
            chain_id: chain_id.to_string(),
        })
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn account_id(&self) -> Result<AccountId> {
        let account_id = self
            .public_key
            .account_id(&self.chain_id)
            .map_err(RegenError::Cosmos)?;
        Ok(account_id)
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn chain_id(&self) -> String {
        self.chain_id.clone()
    }

    pub fn sign(&self, message: &[u8]) -> Result<Signature> {
        let signed_message = self.private_key.sign(message).map_err(RegenError::Cosmos)?;
        Ok(signed_message)
    }
}

pub struct SignerBuilder {
    mnemonic: Option<String>,
    private_key: Option<Vec<u8>>,
    derivation_path: Option<String>,
    chain_id: String,
}

impl SignerBuilder {
    pub fn new(chain_id: &str) -> Self {
        Self {
            mnemonic: None,
            private_key: None,
            derivation_path: None,
            chain_id: chain_id.to_string(),
        }
    }

    pub fn mnemonic(mut self, mnemonic: &str) -> Self {
        self.mnemonic = Some(mnemonic.to_string());
        self
    }

    pub fn private_key(mut self, private_key: &[u8]) -> Self {
        self.private_key = Some(private_key.to_vec());
        self
    }

    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = chain_id.to_string();
        self
    }

    pub fn derivation_path(mut self, derivation_path: &str) -> Self {
        self.derivation_path = Some(derivation_path.to_string());
        self
    }

    pub fn build(self) -> Result<Signer> {
        if self.chain_id.is_empty() {
            return Err(RegenError::InvalidChainId(
                "Chain ID is required".to_string(),
            ));
        }

        let derivation_path = self
            .derivation_path
            .unwrap_or("m/44'/118'/0'/0/0".to_string());

        if let Some(mnemonic) = self.mnemonic {
            Signer::from_mnemonic(&mnemonic, &derivation_path, &self.chain_id)
        } else if let Some(private_key) = self.private_key {
            Signer::from_private_key(&private_key, &self.chain_id)
        } else {
            Err(RegenError::InvalidSignerConfiguration(
                "No mnemonic or private key provided".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    const TEST_CHAIN_ID: &str = "regen";
    const TEST_DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";

    #[test]
    fn test_signer_from_mnemonic() {
        let signer = Signer::from_mnemonic(TEST_MNEMONIC, TEST_DERIVATION_PATH, TEST_CHAIN_ID)
            .expect("Failed to create signer from mnemonic");

        assert_eq!(signer.chain_id(), TEST_CHAIN_ID);
        assert!(!signer.address().is_empty());
        assert!(signer.address().starts_with("regen"));
    }

    #[test]
    fn test_signer_from_private_key() {
        let private_key_bytes = [1u8; 32];

        let signer = Signer::from_private_key(&private_key_bytes, TEST_CHAIN_ID)
            .expect("Failed to create signer from private key");

        assert_eq!(signer.chain_id(), TEST_CHAIN_ID);
        assert!(!signer.address().is_empty());
        assert!(signer.address().starts_with("regen"));
    }

    #[test]
    fn test_signer_deterministic_addresses() {
        let signer1 = Signer::from_mnemonic(TEST_MNEMONIC, TEST_DERIVATION_PATH, TEST_CHAIN_ID)
            .expect("Failed to create first signer");

        let signer2 = Signer::from_mnemonic(TEST_MNEMONIC, TEST_DERIVATION_PATH, TEST_CHAIN_ID)
            .expect("Failed to create second signer");

        assert_eq!(signer1.address(), signer2.address());
        assert_eq!(
            signer1.public_key().to_bytes(),
            signer2.public_key().to_bytes()
        );
    }

    #[test]
    fn test_signer_different_derivation_paths() {
        let path1 = "m/44'/118'/0'/0/0";
        let path2 = "m/44'/118'/0'/0/1";

        let signer1 = Signer::from_mnemonic(TEST_MNEMONIC, path1, TEST_CHAIN_ID)
            .expect("Failed to create signer with path1");

        let signer2 = Signer::from_mnemonic(TEST_MNEMONIC, path2, TEST_CHAIN_ID)
            .expect("Failed to create signer with path2");

        assert_ne!(signer1.address(), signer2.address());
        assert_ne!(
            signer1.public_key().to_bytes(),
            signer2.public_key().to_bytes()
        );
    }

    #[test]
    fn test_signer_signing() {
        let signer = Signer::from_mnemonic(TEST_MNEMONIC, TEST_DERIVATION_PATH, TEST_CHAIN_ID)
            .expect("Failed to create signer");

        let message = b"test message to sign";
        let signature = signer.sign(message).expect("Failed to sign message");

        assert!(!signature.to_bytes().is_empty());
        assert_eq!(signature.to_bytes().len(), 64); // secp256k1 signature length
    }

    #[test]
    fn test_signer_account_id() {
        let signer = Signer::from_mnemonic(TEST_MNEMONIC, TEST_DERIVATION_PATH, TEST_CHAIN_ID)
            .expect("Failed to create signer");

        let account_id = signer.account_id().expect("Failed to get account ID");

        assert_eq!(account_id.to_string(), signer.address());
    }

    #[test]
    fn test_invalid_mnemonic() {
        let invalid_mnemonic = "invalid mnemonic phrase";

        let result = Signer::from_mnemonic(invalid_mnemonic, TEST_DERIVATION_PATH, TEST_CHAIN_ID);
        assert!(result.is_err());

        match result.unwrap_err() {
            RegenError::Mnemonic(_) => {}
            _ => panic!("Expected Mnemonic error"),
        }
    }

    #[test]
    fn test_invalid_derivation_path() {
        let invalid_path = "invalid/path";

        let result = Signer::from_mnemonic(TEST_MNEMONIC, invalid_path, TEST_CHAIN_ID);
        assert!(result.is_err());

        match result.unwrap_err() {
            RegenError::InvalidDerivationPath(_) => {}
            _ => panic!("Expected InvalidDerivationPath error"),
        }
    }

    #[test]
    fn test_invalid_private_key_length() {
        let invalid_key = [1u8; 16]; // Too short

        let result = Signer::from_private_key(&invalid_key, TEST_CHAIN_ID);
        assert!(result.is_err());
    }

    #[test]
    fn test_signer_builder_with_mnemonic() {
        let signer = SignerBuilder::new(TEST_CHAIN_ID)
            .mnemonic(TEST_MNEMONIC)
            .derivation_path(TEST_DERIVATION_PATH)
            .build()
            .expect("Failed to build signer with mnemonic");

        assert_eq!(signer.chain_id(), TEST_CHAIN_ID);
        assert!(!signer.address().is_empty());
    }

    #[test]
    fn test_signer_builder_with_private_key() {
        let private_key_bytes = [1u8; 32];

        let signer = SignerBuilder::new(TEST_CHAIN_ID)
            .private_key(&private_key_bytes)
            .build()
            .expect("Failed to build signer with private key");

        assert_eq!(signer.chain_id(), TEST_CHAIN_ID);
        assert!(!signer.address().is_empty());
    }

    #[test]
    fn test_signer_builder_default_derivation_path() {
        let signer = SignerBuilder::new(TEST_CHAIN_ID)
            .mnemonic(TEST_MNEMONIC)
            .build()
            .expect("Failed to build signer with default path");

        assert!(!signer.address().is_empty());
    }

    #[test]
    fn test_signer_builder_empty_chain_id() {
        let result = SignerBuilder::new("").mnemonic(TEST_MNEMONIC).build();

        assert!(result.is_err());
        match result.unwrap_err() {
            RegenError::InvalidChainId(_) => {}
            _ => panic!("Expected InvalidChainId error"),
        }
    }

    #[test]
    fn test_signer_builder_no_key_material() {
        let result = SignerBuilder::new(TEST_CHAIN_ID).build();

        assert!(result.is_err());
        match result.unwrap_err() {
            RegenError::InvalidSignerConfiguration(_) => {}
            _ => panic!("Expected InvalidSignerConfiguration error"),
        }
    }

    #[test]
    fn test_signer_drop() {
        let signer = Signer::from_mnemonic(TEST_MNEMONIC, TEST_DERIVATION_PATH, TEST_CHAIN_ID)
            .expect("Failed to create signer");

        // Test that signer drops without panicking
        drop(signer);
    }

    #[test]
    fn test_signature_deterministic() {
        let signer = Signer::from_mnemonic(TEST_MNEMONIC, TEST_DERIVATION_PATH, TEST_CHAIN_ID)
            .expect("Failed to create signer");

        let message = b"deterministic test message";

        let signature1 = signer
            .sign(message)
            .expect("Failed to sign message first time");
        let signature2 = signer
            .sign(message)
            .expect("Failed to sign message second time");

        assert_eq!(signature1.to_bytes(), signature2.to_bytes());
    }
}
