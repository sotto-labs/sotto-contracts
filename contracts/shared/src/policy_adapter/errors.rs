use soroban_sdk::contracterror;

/// policy-adapter error codes. Reserved range: 300-399.
/// See `crate::error_codes` for the full cross-crate range table.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PolicyError {
    /// The counterparty is present on the block-list; the settle path must
    /// reject before any Confidential Token call is made.
    Blocked = 300,
    /// The counterparty is not present on the required allow-list.
    NotAllowed = 301,
    /// The configured policy hook could not be reached or returned an
    /// error the adapter could not interpret (fail-closed, not fail-open).
    HookUnavailable = 302,
}
