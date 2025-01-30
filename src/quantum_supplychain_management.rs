use std::collections::HashMap;
use pqcrypto::kem::kyber512::{encrypt, decrypt, PublicKey, SecretKey};
use pqcrypto::sign::dilithium2::{sign, verify};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use quantumfuse_sdk::{
    error::SupplyChainError,
    blockchain::QuantumLedger,
    did::DIDRegistry,
    ai::FraudDetectionEngine,
    iot::IoTDataVerifier,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupplyChainEvent {
    participant: String,
    encrypted_data: Vec<u8>,
    digital_signature: Vec<u8>,
    timestamp: i64,
}

pub struct QuantumSupplyChainContract {
    contract_id: String,
    creator: String,
    authorized_participants: HashMap<String, PublicKey>,
    supply_chain_events: HashMap<String, SupplyChainEvent>,
    quantum_ledger: QuantumLedger,
    fraud_detection: FraudDetectionEngine,
    did_registry: DIDRegistry,
}

impl QuantumSupplyChainContract {
    pub fn new(contract_id: &str, creator: &str, ledger: QuantumLedger, did_registry: DIDRegistry) -> Self {
        QuantumSupplyChainContract {
            contract_id: contract_id.to_string(),
            creator: creator.to_string(),
            authorized_participants: HashMap::new(),
            supply_chain_events: HashMap::new(),
            quantum_ledger: ledger,
            fraud_detection: FraudDetectionEngine::new(),
            did_registry,
        }
    }

    pub fn authorize_participant(&mut self, participant: &str, public_key: PublicKey) -> Result<(), SupplyChainError> {
        if self.authorized_participants.contains_key(participant) {
            return Err(SupplyChainError::DuplicateParticipant);
        }
        
        // Verify DID before adding participant
        if !self.did_registry.verify_identity(participant)? {
            return Err(SupplyChainError::InvalidDID);
        }

        self.authorized_participants.insert(participant.to_string(), public_key);
        Ok(())
    }

    pub fn register_event(
        &mut self,
        participant: &str,
        data: &str,
        private_key: &SecretKey,
    ) -> Result<String, SupplyChainError> {
        if !self.authorized_participants.contains_key(participant) {
            return Err(SupplyChainError::UnauthorizedParticipant);
        }

        // Encrypt data using Kyber512
        let public_key = self.authorized_participants.get(participant).unwrap();
        let encrypted_data = encrypt(data.as_bytes(), public_key);

        // Generate digital signature using Dilithium2
        let signature = sign(encrypted_data.clone(), private_key)?;

        // Generate event ID
        let event_id = Uuid::new_v4().to_string();

        // Store event
        let event = SupplyChainEvent {
            participant: participant.to_string(),
            encrypted_data,
            digital_signature: signature,
            timestamp: Utc::now().timestamp(),
        };
        self.supply_chain_events.insert(event_id.clone(), event.clone());

        // Record event on blockchain ledger
        self.quantum_ledger.record_event(&event_id, &event)?;

        // Run AI-based fraud detection
        self.fraud_detection.analyze_event(&event);

        Ok(event_id)
    }

    pub fn retrieve_event(
        &self,
        participant: &str,
        event_id: &str,
        private_key: &SecretKey,
    ) -> Result<String, SupplyChainError> {
        let event = self.supply_chain_events.get(event_id).ok_or(SupplyChainError::EventNotFound)?;

        if event.participant != participant {
            return Err(SupplyChainError::UnauthorizedAccess);
        }

        // Decrypt data using Kyber512
        let decrypted_data = decrypt(&event.encrypted_data, private_key)?;
        let data_string = String::from_utf8(decrypted_data).map_err(|_| SupplyChainError::DecryptionFailed)?;

        // Verify digital signature using Dilithium2
        let public_key = self.authorized_participants.get(participant).ok_or(SupplyChainError::InvalidPublicKey)?;
        if !verify(&event.encrypted_data, &event.digital_signature, public_key)? {
            return Err(SupplyChainError::InvalidSignature);
        }

        Ok(data_string)
    }

    pub fn integrate_iot_sensor_data(
        &mut self,
        participant: &str,
        sensor_data: &str,
        private_key: &SecretKey,
    ) -> Result<String, SupplyChainError> {
        if !self.authorized_participants.contains_key(participant) {
            return Err(SupplyChainError::UnauthorizedParticipant);
        }

        // Verify IoT Data
        let verified_data = IoTDataVerifier::validate(sensor_data)?;

        // Encrypt and store event
        self.register_event(participant, &verified_data, private_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqcrypto::kem::kyber512::keypair as kyber_keypair;
    use pqcrypto::sign::dilithium2::keypair as dilithium_keypair;

    #[test]
    fn test_supply_chain_contract() {
        let (kyber_pub, kyber_priv) = kyber_keypair();
        let (dilithium_pub, dilithium_priv) = dilithium_keypair();
        let ledger = QuantumLedger::new();
        let did_registry = DIDRegistry::new();

        let mut contract = QuantumSupplyChainContract::new("contract_123", "creator", ledger, did_registry);

        assert!(contract.authorize_participant("supplier_1", kyber_pub).is_ok());

        let event_id = contract.register_event("supplier_1", "Product Shipped", &dilithium_priv).unwrap();
        assert!(!event_id.is_empty());

        let decrypted_data = contract.retrieve_event("supplier_1", &event_id, &kyber_priv).unwrap();
        assert_eq!(decrypted_data, "Product Shipped");
    }

    #[test]
    fn test_iot_data_integration() {
        let (kyber_pub, kyber_priv) = kyber_keypair();
        let (dilithium_pub, dilithium_priv) = dilithium_keypair();
        let ledger = QuantumLedger::new();
        let did_registry = DIDRegistry::new();

        let mut contract = QuantumSupplyChainContract::new("contract_456", "creator", ledger, did_registry);
        contract.authorize_participant("manufacturer", kyber_pub).unwrap();

        let event_id = contract.integrate_iot_sensor_data("manufacturer", "Temp: 5°C", &dilithium_priv).unwrap();
        let decrypted_data = contract.retrieve_event("manufacturer", &event_id, &kyber_priv).unwrap();
        
        assert_eq!(decrypted_data, "Temp: 5°C");
    }
}
