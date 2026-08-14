use soroban_sdk::contracterror;

/// invoice-registry error codes. Reserved range: 1-99.
/// See `crate::error_codes` for the full cross-crate range table and the
/// audit that keeps these ranges disjoint.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum InvoiceError {
    /// store_invoice() called with an invoice_id that already exists.
    AlreadyExists = 1,
    /// Any entrypoint referenced an invoice_id with no stored record.
    NotFound = 2,
    /// Caller is not the buyer or supplier on the invoice.
    Unauthorized = 3,
    /// The requested status transition is not legal from the invoice's
    /// current status (see InvoiceStatus in types.rs).
    InvalidTransition = 4,
    /// list_invoices_by_party() called with a limit above the documented
    /// per-page maximum.
    LimitExceeded = 5,
}
