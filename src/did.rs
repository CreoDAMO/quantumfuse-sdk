use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use quantumfuse_sdk::{
    crypto::{Hash, KeyPair},
    error::DIDError,
    storage::Storage,
    verification::VerificationMethod
};
use pqcrypto::{
    sign::dilithium2::{self, PublicKey, SecretKey, Signature},
    prelude::*
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDID {
    pub id: String,
    pub controller: String,
    pub verification_methods: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub assertion_method: Vec<String>,
    pub key_agreement: Vec<String>,
    pub capability_invocation: Vec<String>,
    pub capability_delegation: Vec<String>,
    pub services: Vec<Service>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    proof: Option<DIDProof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub type_: String,
    pub endpoint: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDProof {
    pub type_: String,
    pub created: DateTime<Utc>,
    pub verification_method: String,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    pub did: QuantumDID,
    pub metadata: DIDMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDMetadata {
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub deactivated: bool,
    pub version_id: String,
    pub next_update: Option<DateTime<Utc>>,
    pub quantum_secure: bool,
}

impl QuantumDID {
    pub fn new(public_key: &PublicKey) -> Result<Self, DIDError> {
        let id = Self::generate_id(public_key)?;
        let controller = id.clone();
        let now = Utc::now();

        // Create verification method for the quantum-resistant key
        let verification_method = VerificationMethod {
            id: format!("{}#quantum-key-1", id),
            type_: "DilithiumVerificationKey2023".to_string(),
            controller: controller.clone(),
            public_key_multibase: base58::encode(public_key),
        };

        let mut did = Self {
            id,
            controller,
            verification_methods: vec![verification_method.clone()],
            authentication: vec![verification_method.id.clone()],
            assertion_method: vec![],
            key_agreement: vec![],
            capability_invocation: vec![],
            capability_delegation: vec![],
            services: vec![],
            created: now,
            updated: now,
            proof: None,
        };

        // Sign the DID with the quantum-resistant key
        did.sign(public_key)?;

        Ok(did)
    }

    pub fn resolve(&self) -> Result<DIDDocument, DIDError> {
        let metadata = DIDMetadata {
            created: self.created,
            updated: self.updated,
            deactivated: false,
            version_id: "1.0".to_string(),
            next_update: None,
            quantum_secure: true,
        };

        Ok(DIDDocument {
            did: self.clone(),
            metadata,
        })
    }

    pub fn add_service(&mut self, service: Service) -> Result<(), DIDError> {
        // Validate service endpoint
        if !Self::validate_service_endpoint(&service.endpoint) {
            return Err(DIDError::InvalidServiceEndpoint);
        }

        // Ensure service ID is unique
        if self.services.iter().any(|s| s.id == service.id) {
            return Err(DIDError::DuplicateServiceId);
        }

        self.services.push(service);
        self.updated = Utc::now();
        Ok(())
    }

    pub fn add_verification_method(&mut self, method: VerificationMethod) -> Result<(), DIDError> {
        // Validate verification method
        if !Self::validate_verification_method(&method) {
            return Err(DIDError::InvalidVerificationMethod);
        }

        self.verification_methods.push(method);
        self.updated = Utc::now();
        Ok(())
    }

    pub fn rotate_key(&mut self, new_public_key: &PublicKey) -> Result<(), DIDError> {
        let new_verification_method = VerificationMethod {
            id: format!("{}#quantum-key-{}", self.id, self.verification_methods.len() + 1),
            type_: "DilithiumVerificationKey2023".to_string(),
            controller: self.controller.clone(),
            public_key_multibase: base58::encode(new_public_key),
        };

        self.verification_methods.push(new_verification_method.clone());
        self.authentication.push(new_verification_method.id);
        self.updated = Utc::now();

        // Sign with new key
        self.sign(new_public_key)?;

        Ok(())
    }

    pub fn verify(&self) -> Result<bool, DIDError> {
        let proof = self.proof.as_ref().ok_or(DIDError::MissingProof)?;
        
        // Find verification method
        let method = self.verification_methods
            .iter()
            .find(|m| m.id == proof.verification_method)
            .ok_or(DIDError::InvalidVerificationMethod)?;

        // Decode public key
        let public_key = base58::decode(&method.public_key_multibase)
            .map_err(|_| DIDError::InvalidPublicKey)?;

        // Create message to verify
        let message = self.create_signing_input()?;

        // Verify signature
        dilithium2::verify(
            &message,
            &proof.signature,
            &public_key,
        ).map_err(|_| DIDError::InvalidSignature)?;

        Ok(true)
    }

    pub fn deactivate(&mut self) -> Result<(), DIDError> {
        // Create deactivation proof
        let deactivation_proof = DIDProof {
            type_: "Deactivation".to_string(),
            created: Utc::now(),
            verification_method: self.authentication[0].clone(),
            signature: vec![], // Should be signed by controller
        };

        self.proof = Some(deactivation_proof);
        self.updated = Utc::now();
        Ok(())
    }

    // Private helper methods
    fn generate_id(public_key: &PublicKey) -> Result<String, DIDError> {
        let hash = blake3::hash(&public_key);
        Ok(format!("did:qf:{}", hex::encode(&hash.as_bytes()[..20])))
    }

    fn sign(&mut self, public_key: &PublicKey) -> Result<(), DIDError> {
        let message = self.create_signing_input()?;
        
        // Note: In production, this would use the actual secret key
        // This is just for demonstration
        let signature = vec![]; // Should be actual signature

        self.proof = Some(DIDProof {
            type_: "DilithiumSignature2023".to_string(),
            created: Utc::now(),
            verification_method: format!("{}#quantum-key-1", self.id),
            signature,
        });

        Ok(())
    }

    fn create_signing_input(&self) -> Result<Vec<u8>, DIDError> {
        // Create canonical form of DID document without proof
        let mut did_without_proof = self.clone();
        did_without_proof.proof = None;
        
        serde_json::to_vec(&did_without_proof)
            .map_err(|_| DIDError::SerializationError)
    }

    fn validate_service_endpoint(endpoint: &str) -> bool {
        // Implement endpoint validation
        url::Url::parse(endpoint).is_ok()
    }

    fn validate_verification_method(method: &VerificationMethod) -> bool {
        // Implement verification method validation
        !method.id.is_empty() && 
        !method.type_.is_empty() && 
        !method.controller.is_empty() && 
        !method.public_key_multibase.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_creation() {
        let (public_key, _) = dilithium2::keypair();
        let did = QuantumDID::new(&public_key).unwrap();
        assert!(did.id.starts_with("did:qf:"));
        assert_eq!(did.verification_methods.len(), 1);
        assert_eq!(did.authentication.len(), 1);
    }

    #[test]
    fn test_did_resolution() {
        let (public_key, _) = dilithium2::keypair();
        let did = QuantumDID::new(&public_key).unwrap();
        let doc = did.resolve().unwrap();
        assert_eq!(did.id, doc.did.id);
        assert!(doc.metadata.quantum_secure);
    }

    #[test]
    fn test_service_addition() {
        let (public_key, _) = dilithium2::keypair();
        let mut did = QuantumDID::new(&public_key).unwrap();
        
        let service = Service {
            id: "test-service".to_string(),
            type_: "QuantumEndpoint".to_string(),
            endpoint: "https://quantum.example.com".to_string(),
            properties: HashMap::new(),
        };

        assert!(did.add_service(service).is_ok());
        assert_eq!(did.services.len(), 1);
    }

    #[test]
    fn test_key_rotation() {
        let (public_key1, _) = dilithium2::keypair();
        let (public_key2, _) = dilithium2::keypair();
        let mut did = QuantumDID::new(&public_key1).unwrap();
        
        assert!(did.rotate_key(&public_key2).is_ok());
        assert_eq!(did.verification_methods.len(), 2);
        assert_eq!(did.authentication.len(), 2);
    }
}
