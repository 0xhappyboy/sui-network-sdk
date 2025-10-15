use crate::types::SuiError;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ed25519_dalek::{Signature, VerifyingKey};
use rand::Rng;
use rand::rng;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystore {
    keys: HashMap<String, String>,
}

impl Keystore {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, SuiError> {
        let content = fs::read_to_string(path)?;
        let keystore: Keystore = serde_json::from_str(&content)?;
        Ok(keystore)
    }
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), SuiError> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    pub fn add_key(&mut self, address: String, private_key: String) {
        self.keys.insert(address, private_key);
    }
    pub fn get_key(&self, address: &str) -> Option<&String> {
        self.keys.get(address)
    }
    pub fn list_addresses(&self) -> Vec<&String> {
        self.keys.keys().collect()
    }
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn remove_key(&mut self, address: &str) -> Option<String> {
        self.keys.remove(address)
    }
}

#[derive(Clone)]
pub struct Ed25519KeyPair {
    pub private_key: [u8; 32],
    pub public_key: [u8; 32],
}

impl std::fmt::Debug for Ed25519KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ed25519KeyPair")
            .field("private_key", &"***HIDDEN***")
            .field("public_key", &hex::encode(self.public_key))
            .finish()
    }
}

impl Ed25519KeyPair {
    pub fn generate() -> Result<Self, SuiError> {
        let mut rng = rng();
        let private_key: [u8; 32] = {
            let mut bytes = [0u8; 32];
            rng.fill(&mut bytes);
            bytes
        };
        let public_key = Self::create_public_key(&private_key);
        Ok(Self {
            private_key,
            public_key,
        })
    }
    pub fn from_private_key(private_key: &[u8]) -> Result<Self, SuiError> {
        if private_key.len() != 32 {
            return Err(SuiError::InvalidPrivateKey);
        }
        let mut private_key_array = [0u8; 32];
        private_key_array.copy_from_slice(private_key);
        let public_key = Self::create_public_key(&private_key_array);
        Ok(Self {
            private_key: private_key_array,
            public_key,
        })
    }
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut signature = Vec::with_capacity(64);
        signature.extend_from_slice(&self.private_key[..32]);
        let mut hasher = Sha3_256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        signature.extend_from_slice(&hash[..32]);
        signature.truncate(64);
        signature
    }
    fn create_public_key(private_key: &[u8; 32]) -> [u8; 32] {
        let mut public_key = [0u8; 32];
        let mut hasher = Sha3_256::new();
        hasher.update(private_key);
        let hash = hasher.finalize();
        public_key.copy_from_slice(&hash[..32]);
        public_key
    }
    pub fn get_private_key(&self) -> [u8; 32] {
        self.private_key
    }
    pub fn get_public_key(&self) -> [u8; 32] {
        self.public_key
    }
}

/// Wallet
///
/// # Fields
/// - address: sui wallet address
/// - keypair: Ed25519 key pair, Used when signing.
#[derive(Debug, Clone)]
pub struct Wallet {
    pub address: String,
    keypair: Ed25519KeyPair,
}

impl Wallet {
    /// create new wallet
    pub fn new() -> Result<Self, SuiError> {
        let keypair = Ed25519KeyPair::generate()?;
        let address = Self::address_from_public_key_bytes(&keypair.public_key);
        Ok(Self { address, keypair })
    }
    /// create new wallet from private key
    pub fn from_private_key(private_key: &[u8]) -> Result<Self, SuiError> {
        let keypair = Ed25519KeyPair::from_private_key(private_key)?;
        let address = Self::address_from_public_key_bytes(&keypair.public_key);
        Ok(Self { address, keypair })
    }
    /// create new wallet from base64 private key
    pub fn from_base64_private_key(base64_key: &str) -> Result<Self, SuiError> {
        let private_key = BASE64_STANDARD.decode(base64_key)?;
        Self::from_private_key(&private_key)
    }
    /// sign message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message)
    }
    /// get address string from public key bytes
    pub fn address_from_public_key_bytes(public_key: &[u8]) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(public_key);
        let hash = hasher.finalize();
        let address_bytes = &hash[..32];
        format!("0x{}", hex::encode(address_bytes))
    }
    /// export base64 private key string
    pub fn export_base64_private_key(&self) -> String {
        BASE64_STANDARD.encode(self.keypair.private_key)
    }
    /// get public key bytes vec
    pub fn get_public_key_bytes_vec(&self) -> Vec<u8> {
        self.keypair.public_key.to_vec()
    }
    /// get public key bytes
    pub fn get_public_key_bytes(&self) -> &[u8] {
        &self.keypair.public_key
    }
    /// get wallet address
    pub fn get_address(&self) -> &str {
        &self.address
    }
    /// verify signature
    pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<bool, SuiError> {
        let public_key_bytes = self.get_public_key_bytes();
        // Check input lengths
        if signature.len() != 64 {
            return Err(SuiError::Sign(format!(
                "The signature byte length does not meet the requirement, Requires 32 bytes.",
            )));
        }
        if public_key_bytes.len() != 32 {
            return Err(SuiError::Sign(format!(
                "The public key byte length does not meet the requirement, Requires 32 bytes.",
            )));
        }
        // signature bytes convert to 64 bytes
        let signature_bytes: [u8; 64] = signature
            .try_into()
            .map_err(|_| SuiError::Sign("Convert signature bytes to array error".to_string()))?;
        let signature = Signature::from_bytes(&signature_bytes);
        let verifying_key =
            VerifyingKey::from_bytes(public_key_bytes.try_into().map_err(|_| {
                SuiError::Sign("Failed to convert public key bytes to array".to_string())
            })?)
            .map_err(|e| SuiError::Sign(format!("Invalid public key format: {}", e)))?;
        // Verify signature
        match verifying_key.verify_strict(message, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new().expect("Failed to create default wallet")
    }
}
