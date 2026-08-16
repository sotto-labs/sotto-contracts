//! Netting attestation.
//!
//! Records that N invoices were netted down to one transfer, with a
//! commitment to the netting set, so the compression is verifiable without
//! revealing which invoices were in it.

#![no_std]

use soroban_sdk::{contract, contractimpl, BytesN, Env, Vec};
use sotto_shared::invoice_registry::types::InvoiceId;
use sotto_shared::netting_attestation::{errors::NettingError, types::NettingCommitment};
use sotto_shared::settlement_orchestrator::types::SettlementId;

#[contract]
pub struct NettingAttestation;

#[contractimpl]
impl NettingAttestation {
    // ISSUE: [contracts] Implement netting-set commitment construction
    // CONTEXT: A netting set (the group of invoice_ids compressed into one
    //          transfer) must be committed to on-chain without revealing its
    //          members. sotto-core computes the same commitment off-chain when
    //          it builds the netting proposal; this function is the on-chain
    //          mirror both sides must agree on byte-for-byte.
    // SCOPE:   construct_netting_commitment() — deterministic commitment over
    //          (invoice_ids, salt)
    // ACCEPTANCE:
    //   - commitment is order-independent: the same invoice_id set in any
    //     order and the same salt produces the same commitment
    //   - different salts for the same invoice_id set produce different
    //     commitments (salt is not decorative)
    //   - empty invoice_ids is rejected with NettingError::EmptySet
    //   - the hash construction matches whatever sotto-core's proof service
    //     computes off-chain — this is the shared contract between the two
    //     repos and must be documented, not just implemented
    // TESTS: order independence, salt sensitivity, empty-set rejection,
    //        cross-check vector against a fixed off-chain-computed commitment
    // OUT OF SCOPE: the membership proof itself (see record_netting)
    // DIFFICULTY: hard
    // LABELS: contracts, soroban, cryptography
    pub fn construct_netting_commitment(
        _env: Env,
        _invoice_ids: Vec<InvoiceId>,
        _salt: BytesN<32>,
    ) -> Result<NettingCommitment, NettingError> {
        todo!()
    }

    // ISSUE: [contracts] Implement record_netting() with membership proof
    // CONTEXT: Once a netting set is agreed off-chain, the orchestrator needs an
    //          on-chain record binding a settlement_id to that netting set's
    //          commitment, plus a proof that the settlement is actually built
    //          from members of that set — otherwise "netting" is just an
    //          unverifiable off-chain claim glued onto a normal transfer.
    // SCOPE:   record_netting() — bind settlement_id to netting_commitment and
    //          verify membership_proof against it
    // ACCEPTANCE:
    //   - rejects a membership_proof that does not verify against
    //     netting_commitment, with NettingError::InvalidProof
    //   - a settlement_id can only be recorded once; re-recording is rejected
    //     with NettingError::AlreadyRecorded
    //   - the referenced settlement_id must exist in the settlement
    //     orchestrator (cross-contract check), else SettlementError::NotFound
    //   - emits NettingRecorded { settlement_id, netting_commitment }
    // TESTS: valid proof accepted, invalid proof rejected, double-record
    //        rejection, unknown settlement_id rejection
    // OUT OF SCOPE: proof generation (sotto-core), the commitment construction
    //               itself (see construct_netting_commitment)
    // DIFFICULTY: hard
    // LABELS: contracts, soroban, cryptography
    pub fn record_netting(
        _env: Env,
        _settlement_id: SettlementId,
        _netting_commitment: NettingCommitment,
        _membership_proof: BytesN<32>,
    ) -> Result<(), NettingError> {
        todo!()
    }

    // ISSUE: [contracts] Implement verify_netting_attestation()
    // CONTEXT: A counterparty or auditor needs a read-only way to confirm that a
    //          settlement_id's recorded netting attestation is valid, without
    //          re-deriving the proof themselves or trusting a claim off-chain.
    // SCOPE:   verify_netting_attestation() — re-check a stored attestation's
    //          proof against its stored commitment
    // ACCEPTANCE:
    //   - returns true for a settlement_id with a validly recorded attestation
    //   - returns false, not an error, for an unknown settlement_id
    //   - does not mutate any storage — this is a pure read/verify path
    //   - agrees with record_netting()'s own proof check (same verification
    //     logic, not a second implementation that can drift)
    // TESTS: valid attestation returns true, unknown settlement_id returns
    //        false, verification logic shared with record_netting (no drift)
    // OUT OF SCOPE: recording new attestations (see record_netting)
    // DIFFICULTY: medium
    // LABELS: contracts, soroban, cryptography
    pub fn verify_netting_attestation(_env: Env, _settlement_id: SettlementId) -> bool {
        todo!()
    }

    // ISSUE: [contracts] Implement netting-set query by settlement ID
    // CONTEXT: Reconciliation and the auditor console both need to look up which
    //          netting commitment a given settlement_id attested to, without
    //          re-deriving it from event logs.
    // SCOPE:   netting_commitment_by_settlement() — read the stored commitment
    //          for a settlement_id
    // ACCEPTANCE:
    //   - returns the exact commitment passed to record_netting() for that
    //     settlement_id
    //   - unknown settlement_id is rejected with NettingError::NotFound
    // TESTS: happy path lookup, unknown settlement_id rejection
    // OUT OF SCOPE: verification (see verify_netting_attestation)
    // DIFFICULTY: easy
    // LABELS: contracts, soroban, query
    pub fn netting_commitment_by_settlement(
        _env: Env,
        _settlement_id: SettlementId,
    ) -> Result<NettingCommitment, NettingError> {
        todo!()
    }
}
