#![no_std]

//! Core entry points for the contract.
//!
//! This module restores the admin-gated `upgrade()` entry point documented in
//! `DECISIONS.md` §5 and `README.md` ("In-Place Contract Upgradability"). The
//! schema-version guard is enforced *before* the WASM is ever touched so a
//! stale or malicious `expected_schema_version` can never swap the code.

use soroban_sdk::{contract, contractimpl, symbol_short, BytesN, Env, Symbol};

use crate::errors::ContractError;
use crate::events::EventContractUpgraded;
use crate::types::StorageKey;

/// Live schema version of the contract's storage layout.
///
/// Kept as a plain `const` (not derived from WASM metadata) so the upgrade
/// guard is cheap on every admin upgrade attempt.
pub const SCHEMA_VERSION: u32 = 1;

/// Topic used for the `EventContractUpgraded` event.
const UPGRADE_TOPIC: Symbol = symbol_short!("upgrade");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Returns the live schema version of the contract's storage layout.
    ///
    /// This is a cheap query backed by [`SCHEMA_VERSION`]; it is not derived
    /// from WASM metadata.
    pub fn schema_version(_env: Env) -> u32 {
        SCHEMA_VERSION
    }

    /// Upgrades the contract WASM to `new_wasm_hash`.
    ///
    /// Admin-gated via `require_auth()`. The caller's `expected_schema_version`
    /// must match the live [`schema_version`](Self::schema_version); otherwise
    /// the call is rejected with [`ContractError::SchemaVersionMismatch`]
    /// *before* `env.deployer().update_current_contract_wasm()` is invoked.
    ///
    /// On success, `EventContractUpgraded` ("upgrade" topic) is emitted.
    /// Persistent and instance storage survive the upgrade; only temporary
    /// storage (idempotency keys) may be evicted.
    pub fn upgrade(
        env: Env,
        new_wasm_hash: BytesN<32>,
        expected_schema_version: u32,
    ) -> Result<(), ContractError> {
        // Cheapest-first ordering: authenticate the admin before any other work.
        let admin: soroban_sdk::Address = env
            .storage()
            .instance()
            .get(&StorageKey::Admin)
            .ok_or(ContractError::Unauthorized)?;
        admin.require_auth();

        // Schema guard: reject mismatches before touching WASM.
        if expected_schema_version != SCHEMA_VERSION {
            return Err(ContractError::SchemaVersionMismatch);
        }

        // Only now is it safe to swap the contract code.
        env.deployer()
            .update_current_contract_wasm(new_wasm_hash.clone());

        env.events().publish(
            (UPGRADE_TOPIC,),
            EventContractUpgraded {
                new_wasm_hash,
                schema_version: SCHEMA_VERSION,
            },
        );

        Ok(())
    }
}
