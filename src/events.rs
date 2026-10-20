//! Event definitions and emission helpers for the contract.
//!
//! Events are emitted with a stable topic so off-chain indexers can filter
//! reliably. The `upgrade` topic is emitted by the admin-gated `upgrade()`
//! entry point after a successful WASM swap (see `DECISIONS.md` §5).

use soroban_sdk::{contracttype, symbol_short, BytesN, Env, Symbol};

/// Topic symbol used for contract-upgrade events.
pub const UPGRADE_TOPIC: Symbol = symbol_short!("upgrade");

/// Payload emitted when the contract WASM is upgraded in place.
///
/// `new_wasm_hash` is the hash of the freshly installed WASM and
/// `schema_version` is the live schema version the upgrade was validated
/// against.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventContractUpgraded {
    pub new_wasm_hash: BytesN<32>,
    pub schema_version: u32,
}

/// Emit the `upgrade` event for a successful in-place WASM swap.
///
/// Called by `upgrade()` only after the schema-version guard has passed and
/// `env.deployer().update_current_contract_wasm()` has been invoked.
pub fn emit_contract_upgraded(env: &Env, new_wasm_hash: BytesN<32>, schema_version: u32) {
    let payload = EventContractUpgraded {
        new_wasm_hash,
        schema_version,
    };
    env.events().publish((UPGRADE_TOPIC,), payload);
}
