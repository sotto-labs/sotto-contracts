use soroban_sdk::{contractevent, Address};

use super::types::InvoiceId;

/// Published by store_invoice() once an invoice commitment is recorded.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceRegistered {
    #[topic]
    pub invoice_id: InvoiceId,
    pub buyer: Address,
    pub supplier: Address,
}

/// Published by approve_invoice() on a successful draft → approved
/// transition.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceApproved {
    #[topic]
    pub invoice_id: InvoiceId,
}

/// Published by void_invoice() on a successful transition to void.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceVoided {
    #[topic]
    pub invoice_id: InvoiceId,
}
