use soroban_sdk::{contractevent, Address};

use super::errors::PolicyError;

/// Published when a PolicyHook or check_block_list() rejects a party on the
/// settle path, so compliance tooling can index rejections without parsing
/// failed-transaction logs.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyCheckRejected {
    #[topic]
    pub party: Address,
    pub reason: PolicyError,
}
