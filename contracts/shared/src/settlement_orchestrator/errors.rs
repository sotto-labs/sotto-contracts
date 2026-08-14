use soroban_sdk::contracterror;

/// settlement-orchestrator error codes. Reserved range: 100-199.
/// See `crate::error_codes` for the full cross-crate range table.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SettlementError {
    /// bind_settlement() referenced an invoice_id that is not in
    /// InvoiceStatus::Approved.
    InvoiceNotApproved = 100,
    /// bind_settlement() referenced an invoice_id already bound to a
    /// different, non-void settlement_id.
    AlreadyBound = 101,
    /// execute_partial_settlement() was called with an invoice_id subset
    /// that is empty or not a subset of the settlement's bound invoices.
    InvalidSubset = 102,
    /// A settlement_id was referenced (e.g. by netting-attestation's
    /// record_netting()) that this contract has no binding for.
    NotFound = 103,
}
