//! Cross-crate error code range convention.
//!
//! OpenZeppelin's Stellar contract suite assigns each module a reserved
//! block of integer error codes by convention rather than letting every
//! `#[contracterror]` enum start at 1. Sotto follows the same pattern so a
//! numeric error code alone (as seen in a failed-transaction result, with no
//! access to source) identifies which contract raised it.
//!
//! | Crate                    | Range     |
//! |---------------------------|-----------|
//! | invoice-registry           | 1-99      |
//! | settlement-orchestrator     | 100-199   |
//! | netting-attestation         | 200-299   |
//! | policy-adapter               | 300-399   |
//!
//! Each crate's own `errors.rs` documents its range in a comment on the
//! `#[contracterror]` enum; this file is the single place that table is
//! collected and cross-checked.

use soroban_sdk::Env;

pub const INVOICE_REGISTRY_ERROR_RANGE: (u32, u32) = (1, 99);
pub const SETTLEMENT_ORCHESTRATOR_ERROR_RANGE: (u32, u32) = (100, 199);
pub const NETTING_ATTESTATION_ERROR_RANGE: (u32, u32) = (200, 299);
pub const POLICY_ADAPTER_ERROR_RANGE: (u32, u32) = (300, 399);

// ISSUE: [contracts] Error-code range audit and documentation
// CONTEXT: The range table above is asserted by hand right now. As each
//          crate's #[contracterror] enum grows, nothing currently stops a
//          new variant from drifting outside its documented range or
//          colliding with another crate's range — and a collision would be a
//          silent, hard-to-debug problem: two different failures reported
//          under the same numeric code to an off-chain indexer.
// SCOPE:   assert_error_code_ranges_disjoint() — a compile-time or test-time
//          check that every variant of InvoiceError, SettlementError,
//          NettingError, and PolicyError falls inside its crate's documented
//          range in this file, and that no two ranges overlap
// ACCEPTANCE:
//   - the check enumerates every variant's discriminant (not just the
//     min/max of each enum) and verifies range membership
//   - the four ranges above are verified pairwise disjoint
//   - the check runs in CI (see segment 7's ci.yml) so a future out-of-range
//     variant fails a build instead of shipping
//   - this file's range table and each crate's errors.rs doc comment are
//     verified to agree, not just this file's constants against themselves
// TESTS: all-in-range passes, an intentionally out-of-range variant fails,
//        an intentionally overlapping range pair fails
// OUT OF SCOPE: choosing new ranges for future crates — this issue only
//               audits and enforces the four ranges already documented
// DIFFICULTY: easy
// LABELS: contracts, tooling, ci
pub fn assert_error_code_ranges_disjoint(_env: &Env) {
    todo!()
}
