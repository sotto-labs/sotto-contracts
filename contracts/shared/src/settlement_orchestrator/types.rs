use soroban_sdk::BytesN;

/// Identifier a caller assigns at bind_settlement() time. Opaque — this
/// contract does not interpret its structure.
pub type SettlementId = BytesN<32>;

/// A proof handle produced off-chain by sotto-core's proof service and
/// passed through to the Confidential Token adapter. Opaque to this
/// contract; see `settlement-orchestrator::ct_adapter`.
pub type SettlementProof = BytesN<32>;
