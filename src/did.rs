use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use quantumfuse_sdk::{
    crypto::{Hash, KeyPair, AESGCM},
    error::DIDError,
    storage::Storage,
    verification::VerificationMethod,
    consensus::QuantumBridge,
    ai::FraudDetector,
    hardware::{HSM, FIDO2Authenticator},
    explorer::DIDTrackerAPI,
};
use pqcrypto::sign::dilithium2::{self, PublicKey, SecretKey, Signature};
use pqcrypto::kem::kyber512::{encapsulate, decapsulate, generate_keypair as kyber_generate};

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
    pub risk_score: f64, // AI-powered fraud risk score
}

impl QuantumDID {
    pub fn new(public_key: &PublicKey, fraud_detector: &FraudDetector) -> Result<Self, DIDError> {
        let id = Self::generate_id(public_key)?;
        let controller = id.clone();
        let now = Utc::now();

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

        did.sign(public_key)?;
        did.analyze_fraud_risk(fraud_detector)?;

        Ok(did)
    }

    pub fn resolve(&self, did_tracker: &DIDTrackerAPI) -> Result<DIDDocument, DIDError> {
        let metadata = DIDMetadata {
            created: self.created,
            updated: self.updated,
            deactivated: false,
            version_id: "1.0".to_string(),
            next_update: None,
            quantum_secure: true,
            risk_score: did_tracker.get_did_risk_score(&self.id)?,
        };

        Ok(DIDDocument {
            did: self.clone(),
            metadata,
        })
    }

    pub fn add_service(&mut self, service: Service) -> Result<(), DIDError> {
        if !Self::validate_service_endpoint(&service.endpoint) {
            return Err(DIDError::InvalidServiceEndpoint);
        }

        if self.services.iter().any(|s| s.id == service.id) {
            return Err(DIDError::DuplicateServiceId);
        }

        self.services.push(service);
        self.updated = Utc::now();
        Ok(())
    }

    pub fn verify(&self, hsm: &HSM) -> Result<bool, DIDError> {
        let proof = self.proof.as_ref().ok_or(DIDError::MissingProof)?;
        
        let method = self.verification_methods
            .iter()
            .find(|m| m.id == proof.verification_method)
            .ok_or(DIDError::InvalidVerificationMethod)?;

        let public_key = base58::decode(&method.public_key_multibase)
            .map_err(|_| DIDError::InvalidPublicKey)?;

        let message = self.create_signing_input()?;

        hsm.verify_signature(&message, &proof.signature, &public_key)
    }

    pub fn authenticate_with_biometrics(&self, fido2: &FIDO2Authenticator) -> Result<bool, DIDError> {
        fido2.authenticate()
    }

    pub fn analyze_fraud_risk(&self, fraud_detector: &FraudDetector) -> Result<(), DIDError> {
        let risk_score = fraud_detector.analyze_did_activity(&self.id)?;
        if risk_score > 0.9 {
            return Err(DIDError::HighFraudRisk);
        }
        Ok(())
    }

    // Private helper methods
    fn generate_id(public_key: &PublicKey) -> Result<String, DIDError> {
        let hash = blake3::hash(&public_key);
        Ok(format!("did:qf:{}", hex::encode(&hash.as_bytes()[..20])))
    }

    fn sign(&mut self, public_key: &PublicKey) -> Result<(), DIDError> {
        let message = self.create_signing_input()?;
        let signature = vec![]; // Securely sign with HSM

        self.proof = Some(DIDProof {
            type_: "DilithiumSignature2023".to_string(),
            created: Utc::now(),
            verification_method: format!("{}#quantum-key-1", self.id),
            signature,
        });

        Ok(())
    }

    fn create_signing_input(&self) -> Result<Vec<u8>, DIDError> {
        let mut did_without_proof = self.clone();
        did_without_proof.proof = None;
        
        serde_json::to_vec(&did_without_proof)
            .map_err(|_| DIDError::SerializationError)
    }
}
