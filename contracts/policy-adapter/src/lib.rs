//! Policy adapter.
//!
//! Plugs the settlement path into the Confidential Token compliance policy
//! engine, so allow-list / block-list identity registries govern who may
//! settle.
//!
//! Deliberately empty in segment 1: its entrypoint stubs (policy hook
//! interface, allow-list adapter, block-list check — issue budget items
//! 16-18) are scoped to segment 2, alongside this crate's `types.rs` and
//! `errors.rs`, once the shared error taxonomy those stubs' ACCEPTANCE
//! criteria depend on exists. Wiring this crate into the workspace now keeps
//! the four-contract folder tree correct from segment 1 onward.

#![no_std]

use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct PolicyAdapter;

#[contractimpl]
impl PolicyAdapter {}
