#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::policy_adapter::types::PolicyHook;
use crate::testutils::policy_mocks::{AlwaysAllowPolicyHook, AlwaysBlockPolicyHook};

#[test]
fn mock_policy_hooks_behave_as_configured() {
    let env = Env::default();
    let party = Address::generate(&env);

    assert!(AlwaysAllowPolicyHook::check(&env, &party).is_ok());
    assert!(AlwaysBlockPolicyHook::check(&env, &party).is_err());
}

// A shallow check on the range table itself, not the full audit
// error_codes::assert_error_code_ranges_disjoint() owes (item 20): this
// does not check that every enum variant actually falls inside its
// crate's range, only that the four declared ranges don't overlap.
#[test]
fn error_code_ranges_are_ordered_and_non_overlapping() {
    let ranges = [
        crate::error_codes::INVOICE_REGISTRY_ERROR_RANGE,
        crate::error_codes::SETTLEMENT_ORCHESTRATOR_ERROR_RANGE,
        crate::error_codes::NETTING_ATTESTATION_ERROR_RANGE,
        crate::error_codes::POLICY_ADAPTER_ERROR_RANGE,
    ];
    for (low, high) in ranges {
        assert!(low <= high, "range {low}-{high} is inverted");
    }
    for i in 0..ranges.len() {
        for j in (i + 1)..ranges.len() {
            let (a_low, a_high) = ranges[i];
            let (b_low, b_high) = ranges[j];
            assert!(
                a_high < b_low || b_high < a_low,
                "ranges {ranges_i:?} and {ranges_j:?} overlap",
                ranges_i = ranges[i],
                ranges_j = ranges[j],
            );
        }
    }
}
