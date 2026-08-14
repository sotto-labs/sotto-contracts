//! Confidential Token adapter boundary.
//!
//! The settlement orchestrator never talks to the Confidential Token contract
//! directly through a hardcoded client type. It goes through this trait
//! instead, because Confidential Tokens are a Developer Preview under audit —
//! the concrete cross-contract call signature has not been verified against a
//! deployed testnet instance. See VERIFY FLAGS in the segment 1 STOP BLOCK.

use soroban_sdk::{Address, BytesN, Env, Error};

pub trait ConfidentialTokenClient {
    /// Execute a confidential transfer and return the host error, if any.
    /// `proof` is the UltraHonk proof produced off-chain by sotto-core's
    /// proof service; this trait does not generate it.
    fn confidential_transfer(
        env: &Env,
        from: &Address,
        to: &Address,
        proof: &BytesN<32>,
    ) -> Result<(), Error>;

    /// Return the current balance commitment for `holder`, as published by
    /// the Confidential Token contract.
    fn balance_commitment(env: &Env, holder: &Address) -> BytesN<32>;
}

pub struct TestnetConfidentialTokenClient;

// ISSUE: [contracts] Implement Confidential Token adapter trait + testnet client
// CONTEXT: The settlement orchestrator must call into Stellar's Confidential
//          Token contract to move funds, but that contract's interface is a
//          Developer Preview under audit — Sotto must not hardcode a signature
//          it has not read from a deployed instance. TestnetConfidentialTokenClient
//          is the concrete adapter bound to the testnet deployment.
// SCOPE:   TestnetConfidentialTokenClient's ConfidentialTokenClient impl —
//          confidential_transfer() and balance_commitment(), wired to the
//          real testnet contract address via a cross-contract call
// ACCEPTANCE:
//   - confidential_transfer() invokes the deployed testnet Confidential Token
//     contract and propagates its error verbatim, not a re-wrapped one
//   - balance_commitment() reads the wrapper's published commitment without
//     ever decrypting or holding a spending key
//   - the testnet contract address is a constructor/config parameter, never
//     hardcoded in this module
//   - no code path in this impl can custody or sign with a user's key
// TESTS: successful transfer against a testnet fork, propagated failure from
//        the underlying contract, balance_commitment happy path
// OUT OF SCOPE: proof generation (sotto-core), mainnet address (Confidential
//               Tokens are testnet-only; see project doc §6)
// DIFFICULTY: hard
// LABELS: contracts, integration, confidential-tokens
impl ConfidentialTokenClient for TestnetConfidentialTokenClient {
    fn confidential_transfer(
        _env: &Env,
        _from: &Address,
        _to: &Address,
        _proof: &BytesN<32>,
    ) -> Result<(), Error> {
        todo!()
    }

    fn balance_commitment(_env: &Env, _holder: &Address) -> BytesN<32> {
        todo!()
    }
}
