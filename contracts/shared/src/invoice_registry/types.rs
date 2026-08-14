use soroban_sdk::{contracttype, Address, BytesN, Env};

/// The 32-byte identifier a supplier or buyer assigns to an invoice at
/// `store_invoice()` time. Opaque to this contract — Sotto never interprets
/// its structure, only checks it for uniqueness.
pub type InvoiceId = BytesN<32>;

/// draft → approved → settled → void. Settled and void are terminal; see the
/// ISSUE block on `extend_invoice_ttl()` below and on
/// `invoice-registry`'s `invoice_status()` for the transition rules a
/// contributor must enforce.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum InvoiceStatus {
    Draft = 0,
    Approved = 1,
    Settled = 2,
    Void = 3,
}

/// Persistent storage keys for the invoice registry.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// A single invoice record, keyed by its InvoiceId.
    Invoice(InvoiceId),
    /// The ordered list of invoice_ids a party is buyer or supplier on, for
    /// `list_invoices_by_party()`'s pagination.
    PartyIndex(Address),
}

/// Ledger entries younger than this many ledgers-to-live are bumped back up
/// to `INVOICE_LIFETIME_LEDGERS` on every write. ~30 days at Stellar's ~5s
/// ledger close time. # VERIFY: not measured against mainnet close times;
/// confirm before relying on this constant for a real TTL budget.
pub const INVOICE_BUMP_THRESHOLD_LEDGERS: u32 = 518_400;

/// Target TTL an invoice entry is extended to on every bump. Kept equal to
/// the bump threshold so a single missed write cycle does not archive a
/// still-open invoice.
pub const INVOICE_LIFETIME_LEDGERS: u32 = 518_400;

// ISSUE: [contracts] Implement TTL and rent-extension strategy for registry entries
// CONTEXT: Soroban archives persistent entries that fall below their TTL
//          threshold. An invoice can sit in Draft or Approved for a long
//          negotiation cycle — if nobody calls a mutating entrypoint before it
//          archives, the registry loses the invoice and the buyer/supplier
//          lose their on-chain record that it ever existed. Project doc §6
//          calls this out as an unresolved open question, not a solved one.
// SCOPE:   extend_invoice_ttl() — the ledger-TTL bump every mutating
//          invoice-registry entrypoint (store_invoice, approve_invoice,
//          void_invoice) must call after writing, plus a strategy for
//          extending a Draft/Approved invoice that nobody has touched
//          recently (a keeper-callable bump, or amortizing the cost into
//          approve_invoice — the tradeoff is this issue's design decision)
// ACCEPTANCE:
//   - after any write, the entry's TTL is at least INVOICE_LIFETIME_LEDGERS
//   - a Settled or Void invoice is not bumped indefinitely — document and
//     enforce a maximum retention window instead of bumping forever
//   - the strategy for un-touched Draft/Approved invoices is implemented, not
//     just documented as a gap
//   - bump cost is accounted for: document the expected instruction/fee cost
//     per bump so a treasurer can reason about registry upkeep cost
// TESTS: bump on write, archival-avoidance over N simulated ledger closes,
//        terminal-status entries stop being bumped past the retention window
// OUT OF SCOPE: settlement or netting contract TTL strategy (separate issues
//               if those contracts need one)
// DIFFICULTY: hard
// LABELS: contracts, soroban, storage
pub fn extend_invoice_ttl(_env: &Env, _key: &DataKey) {
    todo!()
}
