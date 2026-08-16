//! Shared types, errors, and events for the four `sotto-contracts` crates.
//!
//! Each module below mirrors one contract crate (`contracts/<name>`) and
//! carries that contract's `types.rs` / `errors.rs` / `events.rs`. See each
//! contract crate's `src/lib.rs` for the entrypoint stubs these definitions
//! belong to.

#![no_std]

pub mod error_codes;
pub mod invoice_registry;
pub mod netting_attestation;
pub mod policy_adapter;
pub mod settlement_orchestrator;
pub mod testutils;

mod test;
