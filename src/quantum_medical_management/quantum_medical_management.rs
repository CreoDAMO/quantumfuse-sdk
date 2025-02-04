use std::collections::HashMap;
use pqcrypto::kem::kyber512::{encrypt, decrypt, PublicKey, SecretKey};
use pqcrypto::sign::dilithium2::{sign, verify};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use quantumfuse_sdk::{
    error::MedicalError,
    blockchain::QuantumLedger,
    did::DIDRegistry,
    ai::MedicalAIEngine,
    iot::IoTHealthMonitor,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedicalRecord {
    patient_id: String,
    encrypted_data: Vec<u8>,
    digital_signature: Vec<u8>,
    timestamp: i64,
}

pub struct QuantumMedicalContract {
    contract_id: String,
    hospital_id: String,
    authorized_providers: HashMap<String, PublicKey>,
    patient_records: HashMap<String, MedicalRecord>,
    quantum_ledger: QuantumLedger,
    ai_engine: MedicalAIEngine,
    did_registry: DIDRegistry,
}

impl QuantumMedicalContract {
    pub fn new(contract_id: &str, hospital_id: &str, ledger: QuantumLedger, did_registry: DIDRegistry) -> Self {
        QuantumMedicalContract {
            contract_id: contract_id.to_string(),
            hospital_id: hospital_id.to_string(),
            authorized_providers: HashMap::new(),
            patient_records: HashMap::new(),
            quantum_ledger: ledger,
            ai_engine: MedicalAIEngine::new(),
            did_registry,
        }
    }

    pub fn authorize_provider(&mut self, provider_id: &str, public_key: PublicKey) -> Result<(), MedicalError> {
        if self.authorized_providers.contains_key(provider_id) {
            return Err(MedicalError::DuplicateProvider);
        }

        // Verify provider identity via DID
        if !self.did_registry.verify_identity(provider_id)? {
            return Err(MedicalError::InvalidDID);
        }

        self.authorized_providers.insert(provider_id.to_string(), public_key);
        Ok(())
    }

    pub fn add_medical_record(
        &mut self,
        patient_id: &str,
        data: &str,
        private_key: &SecretKey,
    ) -> Result<String, MedicalError> {
        if !self.authorized_providers.contains_key(patient_id) {
            return Err(MedicalError::UnauthorizedProvider);
        }

        // Encrypt medical data using Kyber512
        let public_key = self.authorized_providers.get(patient_id).unwrap();
        let encrypted_data = encrypt(data.as_bytes(), public_key);

        // Digitally sign data with Dilithium2
        let signature = sign(encrypted_data.clone(), private_key)?;

        let record_id = Uuid::new_v4().to_string();

        let record = MedicalRecord {
            patient_id: patient_id.to_string(),
            encrypted_data,
            digital_signature: signature,
            timestamp: Utc::now().timestamp(),
        };
        self.patient_records.insert(record_id.clone(), record.clone());

        // Record in blockchain ledger
        self.quantum_ledger.record_event(&record_id, &record)?;

        // AI-Powered Analysis for disease prediction
        self.ai_engine.analyze_medical_data(&record);

        Ok(record_id)
    }

    pub fn retrieve_medical_record(
        &self,
        provider_id: &str,
        record_id: &str,
        private_key: &SecretKey,
    ) -> Result<String, MedicalError> {
        let record = self.patient_records.get(record_id).ok_or(MedicalError::RecordNotFound)?;

        if record.patient_id != provider_id {
            return Err(MedicalError::UnauthorizedAccess);
        }

        // Decrypt medical data
        let decrypted_data = decrypt(&record.encrypted_data, private_key)?;
        let data_string = String::from_utf8(decrypted_data).map_err(|_| MedicalError::DecryptionFailed)?;

        // Verify signature
        let public_key = self.authorized_providers.get(provider_id).ok_or(MedicalError::InvalidPublicKey)?;
        if !verify(&record.encrypted_data, &record.digital_signature, public_key)? {
            return Err(MedicalError::InvalidSignature);
        }

        Ok(data_string)
    }

    pub fn integrate_iot_health_data(
        &mut self,
        patient_id: &str,
        health_data: &str,
        private_key: &SecretKey,
    ) -> Result<String, MedicalError> {
        if !self.authorized_providers.contains_key(patient_id) {
            return Err(MedicalError::UnauthorizedProvider);
        }

        // Validate IoT Data
        let verified_data = IoTHealthMonitor::validate(health_data)?;

        // Store as medical record
        self.add_medical_record(patient_id, &verified_data, private_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqcrypto::kem::kyber512::keypair as kyber_keypair;
    use pqcrypto::sign::dilithium2::keypair as dilithium_keypair;

    #[test]
    fn test_medical_contract() {
        let (kyber_pub, kyber_priv) = kyber_keypair();
        let (dilithium_pub, dilithium_priv) = dilithium_keypair();
        let ledger = QuantumLedger::new();
        let did_registry = DIDRegistry::new();

        let mut contract = QuantumMedicalContract::new("hospital_123", "hospital_main", ledger, did_registry);

        assert!(contract.authorize_provider("doctor_1", kyber_pub).is_ok());

        let record_id = contract.add_medical_record("patient_1", "Blood Pressure: 120/80", &dilithium_priv).unwrap();
        assert!(!record_id.is_empty());

        let decrypted_data = contract.retrieve_medical_record("doctor_1", &record_id, &kyber_priv).unwrap();
        assert_eq!(decrypted_data, "Blood Pressure: 120/80");
    }

    #[test]
    fn test_iot_health_data_integration() {
        let (kyber_pub, kyber_priv) = kyber_keypair();
        let (dilithium_pub, dilithium_priv) = dilithium_keypair();
        let ledger = QuantumLedger::new();
        let did_registry = DIDRegistry::new();

        let mut contract = QuantumMedicalContract::new("hospital_456", "hospital_main", ledger, did_registry);
        contract.authorize_provider("doctor_2", kyber_pub).unwrap();

        let record_id = contract.integrate_iot_health_data("patient_2", "Heart Rate: 75 bpm", &dilithium_priv).unwrap();
        let decrypted_data = contract.retrieve_medical_record("doctor_2", &record_id, &kyber_priv).unwrap();
        
        assert_eq!(decrypted_data, "Heart Rate: 75 bpm");
    }
}
