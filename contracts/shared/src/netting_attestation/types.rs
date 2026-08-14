use soroban_sdk::{contracttype, BytesN};

/// A commitment produced by construct_netting_commitment() over a netting
/// set's invoice_ids and salt. Opaque outside this contract and sotto-core's
/// mirrored off-chain computation.
pub type NettingCommitment = BytesN<32>;

/// A stored netting attestation: the binding between a settlement_id and the
/// netting_commitment record_netting() verified for it.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NettingAttestationRecord {
    pub settlement_id: crate::settlement_orchestrator::types::SettlementId,
    pub netting_commitment: NettingCommitment,
}
