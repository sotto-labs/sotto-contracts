#![cfg(test)]

use soroban_sdk::Env;

use crate::{NettingAttestation, NettingAttestationClient};

#[test]
fn contract_registers_and_client_constructs() {
    let env = Env::default();
    let contract_id = env.register(NettingAttestation, ());
    let _client = NettingAttestationClient::new(&env, &contract_id);
}
