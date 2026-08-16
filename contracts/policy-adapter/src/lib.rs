//! Policy adapter.
//!
//! Plugs the settlement path into the Confidential Token compliance policy
//! engine, so allow-list / block-list identity registries govern who may
//! settle.
//!
//! `#[contractimpl]` below is deliberately empty: the policy hook interface,
//! allow-list adapter, and block-list check (issue budget items 16-18) live
//! in `sotto_shared::policy_adapter` instead of this crate's own entrypoint
//! surface — see that crate's `types.rs`. Whether this contract ever grows
//! its own entrypoints on top of those, or stays a pure library boundary
//! other contracts call into directly, is an open call for whoever picks up
//! items 16-18.

#![no_std]

use soroban_sdk::{contract, contractimpl};

mod test;

#[contract]
pub struct PolicyAdapter;

#[contractimpl]
impl PolicyAdapter {}
