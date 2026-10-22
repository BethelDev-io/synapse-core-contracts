//! Soroban contract entry points.
//!
//! This module wires the public `#[contractimpl]` surface to the storage and
//! event helpers. See `THREAT_MODEL.md` §4.7 for the read-only query list.

use soroban_sdk::{contractimpl, Address, Env};

use crate::storage;

/// Current contract semver.
///
/// This is intentionally distinct from `schema_version()`: `version()` tracks
/// the contract release (bumped per `EVENTS.md`'s semver policy), while
/// `schema_version()` tracks the persisted storage/event schema. Conflating the
/// two was explicitly avoided per `DECISIONS.md`'s schema-guard discussion.
pub const CONTRACT_VERSION: u32 = 1;

/// Soroban caps a `#[contracterror]` enum at 50 cases.
///
/// `ContractError` (see `src/types.rs`) is consolidated to fit this ceiling; the
/// full "~61 conceptual errors -> <=50 variants" mapping and every merge
/// decision live in the doc-comment atop the enum and in `DECISIONS.md`'s
/// "Error-code budget" section. This constant is the single source of truth for
/// the cap so the compile-time assertion below and any future tooling agree.
pub const MAX_CONTRACT_ERROR_VARIANTS: usize = 50;

pub struct Contract;

#[contractimpl]
impl Contract {
    /// Initialize the contract with an admin and relay signer.
    pub fn initialize(env: Env, admin: Address, relay_signer: Address) {
        storage::set_admin(&env, &admin);
        storage::set_relay_signer(&env, &relay_signer);
    }

    /// Return the current contract semver.
    ///
    /// Stable across calls within a single deployment and independent from
    /// `schema_version()`. Bumped on schema changes per `EVENTS.md`.
    pub fn version(_env: Env) -> u32 {
        CONTRACT_VERSION
    }

    /// Minimal liveness / self-consistency check.
    ///
    /// Returns `true` only when both the admin and relay signer are set. Safe to
    /// call before `initialize()`: it reports `false` rather than panicking.
    pub fn health(env: Env) -> bool {
        storage::get_admin(&env).is_some() && storage::get_relay_signer(&env).is_some()
    }
}

/// Compile-time guard that `ContractError` stays within Soroban's 50-variant cap.
///
/// `ContractError::__VARIANT_COUNT` is a `const` emitted alongside the enum in
/// `src/types.rs`; if a future PR adds a variant past the ceiling this module
/// fails to compile, so the cap cannot silently regress.
const _: () = assert!(
    crate::types::ContractError::__VARIANT_COUNT <= MAX_CONTRACT_ERROR_VARIANTS,
    "ContractError exceeds Soroban's 50-variant cap; see DECISIONS.md 'Error-code budget'"
);
