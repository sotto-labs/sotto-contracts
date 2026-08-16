#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use sotto_shared::policy_adapter::types::PolicyHook;
use sotto_shared::testutils::policy_mocks::{AlwaysAllowPolicyHook, AlwaysBlockPolicyHook};

use crate::{PolicyAdapter, PolicyAdapterClient};

#[test]
fn contract_registers_and_client_constructs() {
    let env = Env::default();
    let contract_id = env.register(PolicyAdapter, ());
    let _client = PolicyAdapterClient::new(&env, &contract_id);
}

#[test]
fn mock_policy_hooks_behave_as_configured() {
    let env = Env::default();
    let party = Address::generate(&env);

    assert!(AlwaysAllowPolicyHook::check(&env, &party).is_ok());
    assert!(AlwaysBlockPolicyHook::check(&env, &party).is_err());
}
