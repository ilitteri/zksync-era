use core::fmt;

use circuit_sequencer_api_1_5_0::proof::FinalProof;
use serde::{Deserialize, Serialize};
use zksync_object_store::{serialize_using_bincode, Bucket, StoredObject};
use zksync_types::{protocol_version::ProtocolSemanticVersion, L1BatchNumber};

/// The only type of proof utilized by the core subsystem: a "final" proof that can be sent
/// to the L1 contract.
#[derive(Clone, Serialize, Deserialize)]
pub struct L1BatchProofForL1 {
    pub aggregation_result_coords: [[u8; 32]; 4],
    pub scheduler_proof: FinalProof,
    pub protocol_version: ProtocolSemanticVersion,
}

impl fmt::Debug for L1BatchProofForL1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("L1BatchProofForL1")
            .field("aggregation_result_coords", &self.aggregation_result_coords)
            .finish_non_exhaustive()
    }
}

impl StoredObject for L1BatchProofForL1 {
    const BUCKET: Bucket = Bucket::ProofsFri;
    type Key<'a> = (L1BatchNumber, ProtocolSemanticVersion);

    fn encode_key(key: Self::Key<'_>) -> String {
        let (l1_batch_number, protocol_version) = key;
        let semver_suffix = protocol_version.to_string().replace('.', "_");
        format!("l1_batch_proof_{l1_batch_number}_{semver_suffix}.bin")
    }

    serialize_using_bincode!();
}
