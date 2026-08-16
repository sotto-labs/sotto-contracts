use soroban_sdk::{Address, Env};

use crate::policy_adapter::{errors::PolicyError, types::PolicyHook};

/// A PolicyHook that accepts every party. Lets a settlement-path test
/// exercise its happy case without wiring a real compliance policy engine
/// or allow-list.
pub struct AlwaysAllowPolicyHook;

impl PolicyHook for AlwaysAllowPolicyHook {
    fn check(_env: &Env, _party: &Address) -> Result<(), PolicyError> {
        Ok(())
    }
}

/// A PolicyHook that rejects every party with PolicyError::Blocked. Lets a
/// settlement-path test exercise rejection handling without a real
/// compliance policy engine or block-list.
pub struct AlwaysBlockPolicyHook;

impl PolicyHook for AlwaysBlockPolicyHook {
    fn check(_env: &Env, _party: &Address) -> Result<(), PolicyError> {
        Err(PolicyError::Blocked)
    }
}
