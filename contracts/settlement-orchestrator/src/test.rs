#![cfg(test)]

use soroban_sdk::{Address, BytesN, Env, Error};

use crate::ct_adapter::ConfidentialTokenClient;
use crate::{SettlementOrchestrator, SettlementOrchestratorClient};

/// A ConfidentialTokenClient stand-in for tests that don't need a real
/// Confidential Token deployment. Deliberately panics rather than faking a
/// successful transfer — a mock that fakes success would hide the fact
/// that no settlement-orchestrator entrypoint using it is implemented yet.
/// Extend this once a test actually needs transfer/balance behavior.
struct MockConfidentialTokenClient;

impl ConfidentialTokenClient for MockConfidentialTokenClient {
    fn confidential_transfer(
        _env: &Env,
        _from: &Address,
        _to: &Address,
        _proof: &BytesN<32>,
    ) -> Result<(), Error> {
        unimplemented!("MockConfidentialTokenClient: extend when a test needs transfer behavior")
    }

    fn balance_commitment(_env: &Env, _holder: &Address) -> BytesN<32> {
        unimplemented!("MockConfidentialTokenClient: extend when a test needs balance behavior")
    }
}

#[test]
fn contract_registers_and_client_constructs() {
    let env = Env::default();
    let contract_id = env.register(SettlementOrchestrator, ());
    let _client = SettlementOrchestratorClient::new(&env, &contract_id);
}

#[test]
fn mock_confidential_token_client_satisfies_the_adapter_trait() {
    // Compile-time check that the mock satisfies ct_adapter's boundary — no
    // runtime behavior to assert until the real ct_adapter stubs (item 8)
    // are implemented.
    fn assert_impl<T: ConfidentialTokenClient>() {}
    assert_impl::<MockConfidentialTokenClient>();
}
