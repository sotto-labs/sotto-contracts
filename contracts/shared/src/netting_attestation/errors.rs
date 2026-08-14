use soroban_sdk::contracterror;

/// netting-attestation error codes. Reserved range: 200-299.
/// See `crate::error_codes` for the full cross-crate range table.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NettingError {
    /// construct_netting_commitment() called with an empty invoice_ids Vec.
    EmptySet = 200,
    /// record_netting()'s membership_proof did not verify against the
    /// supplied netting_commitment.
    InvalidProof = 201,
    /// record_netting() called for a settlement_id that already has a
    /// recorded attestation.
    AlreadyRecorded = 202,
    /// A settlement_id was queried (netting_commitment_by_settlement(),
    /// verify_netting_attestation()) with no recorded attestation.
    NotFound = 203,
}
