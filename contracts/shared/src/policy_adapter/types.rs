use soroban_sdk::{Address, Env};

use super::errors::PolicyError;

/// The policy-adapter's boundary against Stellar's Confidential Token
/// compliance policy engine. Defined locally rather than importing an
/// OpenZeppelin interface directly, for the same reason as
/// `settlement-orchestrator::ct_adapter::ConfidentialTokenClient`:
/// Confidential Tokens are a Developer Preview under audit and this trait's
/// shape has not been verified against a deployed policy engine instance.
pub trait PolicyHook {
    /// Return Ok(()) if `party` may participate in a settlement under this
    /// hook's policy, or the specific PolicyError otherwise.
    fn check(env: &Env, party: &Address) -> Result<(), PolicyError>;
}

pub struct ConfidentialTokenPolicyHook;

// ISSUE: [contracts] Implement policy hook interface to the CT compliance policy engine
// CONTEXT: The settle path must not move funds for a party the Confidential
//          Token compliance policy engine would reject. This adapter is the
//          concrete PolicyHook bound to that engine's deployed instance, the
//          same way ct_adapter::TestnetConfidentialTokenClient is bound to
//          the Confidential Token contract itself.
// SCOPE:   ConfidentialTokenPolicyHook's PolicyHook impl — check() calls into
//          the compliance policy engine via cross-contract call
// ACCEPTANCE:
//   - check() fails closed: any error reading the policy engine (not just an
//     explicit rejection) returns PolicyError::HookUnavailable, never Ok(())
//   - a party the engine accepts returns Ok(())
//   - a party the engine rejects returns the specific error the engine gave,
//     mapped onto PolicyError, not a generic Blocked
//   - the policy engine's contract address is a constructor/config
//     parameter, never hardcoded
// TESTS: accepted party, rejected party, unreachable engine fails closed
// OUT OF SCOPE: allow-list and block-list adapters below (they are two
//               possible PolicyHook implementations a deployment can choose
//               between, not sub-steps of this one)
// DIFFICULTY: medium
// LABELS: contracts, integration, confidential-tokens
impl PolicyHook for ConfidentialTokenPolicyHook {
    fn check(_env: &Env, _party: &Address) -> Result<(), PolicyError> {
        todo!()
    }
}

pub struct AllowListRegistryAdapter;

// ISSUE: [contracts] Implement allow-list identity registry adapter
// CONTEXT: Some Sotto deployments will want an explicit allow-list — only
//          pre-approved counterparties may settle — rather than delegating
//          entirely to the Confidential Token compliance engine. This is a
//          second, independent PolicyHook implementation a deployment can
//          configure instead of (or in front of) ConfidentialTokenPolicyHook.
// SCOPE:   AllowListRegistryAdapter's PolicyHook impl — check() looks up
//          `party` in an on-chain allow-list registry
// ACCEPTANCE:
//   - a party present in the registry returns Ok(())
//   - a party absent from the registry returns PolicyError::NotAllowed
//   - the registry is stored so an admin-only entrypoint can add/remove
//     entries (that entrypoint is part of this issue, not a follow-up)
//   - registry mutation is restricted to the admin role established by
//     issue budget item 19 (admin role and upgrade authorization pattern)
// TESTS: allowed party, disallowed party, admin add/remove, non-admin
//        mutation rejected
// OUT OF SCOPE: block-list check (see below), the Confidential Token policy
//               engine adapter (see ConfidentialTokenPolicyHook)
// DIFFICULTY: medium
// LABELS: contracts, soroban, compliance
impl PolicyHook for AllowListRegistryAdapter {
    fn check(_env: &Env, _party: &Address) -> Result<(), PolicyError> {
        todo!()
    }
}

// ISSUE: [contracts] Implement block-list check on the settle path
// CONTEXT: Independent of which PolicyHook a deployment configures, Sotto
//          needs a fast, always-on block-list check that every settlement
//          path runs before touching the Confidential Token contract — a
//          cheap first line of defense that does not depend on a
//          cross-contract call to the policy engine succeeding.
// SCOPE:   check_block_list() — the guard settlement-orchestrator's
//          execute_partial_settlement() (and the eventual full-settlement
//          entrypoint) must call before ct_adapter::confidential_transfer()
// ACCEPTANCE:
//   - a party on the block-list is rejected with PolicyError::Blocked before
//     any cross-contract call is made
//   - a party not on the block-list returns Ok(()) without consulting the
//     allow-list or the Confidential Token policy engine — this check is
//     block-list-only by design, composition with the other hooks happens at
//     the call site
//   - block-list mutation is restricted to the admin role (see item 19)
// TESTS: blocked party rejected before any cross-contract call, non-blocked
//        party passes, non-admin mutation rejected
// OUT OF SCOPE: allow-list check, Confidential Token policy engine adapter
// DIFFICULTY: easy
// LABELS: contracts, soroban, compliance
pub fn check_block_list(_env: &Env, _party: &Address) -> Result<(), PolicyError> {
    todo!()
}
