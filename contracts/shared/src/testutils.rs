//! Test-only fixtures shared across the contract crates' test suites.
//!
//! Deliberately does not use soroban-sdk's `testutils` feature
//! (`Address::generate`, `BytesN::random`) so this module compiles under
//! sotto-shared's normal (non-dev) dependency set, with no Cargo.toml
//! feature-wiring change needed in any of the four contract crates. Each
//! crate's own `test.rs` already has that feature available through its
//! existing `soroban-sdk` dev-dependency (from segment 1) and should
//! generate its own sample addresses/IDs directly rather than routing them
//! through here.

pub mod admin;
pub mod policy_mocks;
