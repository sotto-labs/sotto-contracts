use soroban_sdk::contractevent;

use super::types::NettingCommitment;
use crate::settlement_orchestrator::types::SettlementId;

/// Published by record_netting() once a membership proof verifies against
/// the supplied netting_commitment.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NettingRecorded {
    #[topic]
    pub settlement_id: SettlementId,
    pub netting_commitment: NettingCommitment,
}
