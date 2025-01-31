use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{create_random_proof, prepare_verifying_key, verify_proof, ProvingKey, VerifierKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKVote {
    pub voter_id: String,
    pub proposal_id: String,
    pub proof: Vec<u8>,
}

pub struct ZKPSystem {
    pub proving_key: ProvingKey<Bls12_381>,
    pub verifier_key: VerifierKey<Bls12_381>,
}

impl ZKPSystem {
    pub fn new() -> Self {
        let rng = &mut thread_rng();
        let circuit = DummyCircuit::<Fr> {};
        let (proving_key, verifier_key) = generate_random_parameters::<Bls12_381, _, _>(circuit, rng).unwrap();
        Self { proving_key, verifier_key }
    }

    pub fn generate_proof(&self, voter_id: &str, proposal_id: &str) -> Vec<u8> {
        let circuit = DummyCircuit::<Fr> {};
        let proof = create_random_proof(circuit, &self.proving_key, &mut thread_rng()).unwrap();
        let mut proof_bytes = vec![];
        proof.serialize(&mut proof_bytes).unwrap();
        proof_bytes
    }

    pub fn verify_vote(&self, zk_vote: &ZKVote) -> bool {
        let proof = Proof::deserialize(&zk_vote.proof[..]).unwrap();
        let prepared_vk = prepare_verifying_key(&self.verifier_key);
        verify_proof(&prepared_vk, &proof, &[]).unwrap()
    }
}