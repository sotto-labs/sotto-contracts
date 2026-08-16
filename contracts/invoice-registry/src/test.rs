#![cfg(test)]

use soroban_sdk::Env;

use crate::{InvoiceRegistry, InvoiceRegistryClient};

#[test]
fn contract_registers_and_client_constructs() {
    let env = Env::default();
    let contract_id = env.register(InvoiceRegistry, ());
    let _client = InvoiceRegistryClient::new(&env, &contract_id);
}
