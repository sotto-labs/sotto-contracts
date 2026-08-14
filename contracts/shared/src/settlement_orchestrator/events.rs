use soroban_sdk::{contractevent, Address, Vec};

use super::types::SettlementId;
use crate::invoice_registry::types::InvoiceId;

/// Published by bind_settlement() once a settlement_id is associated with
/// its invoice_ids.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementBound {
    #[topic]
    pub settlement_id: SettlementId,
    pub invoice_ids: Vec<InvoiceId>,
}

/// Published by the settlement-executing entrypoints (full and partial)
/// after the underlying Confidential Token transfer succeeds. See
/// `settlement-orchestrator::emit_settlement_executed()`.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementExecuted {
    #[topic]
    pub settlement_id: SettlementId,
    pub buyer: Address,
    pub supplier: Address,
}
