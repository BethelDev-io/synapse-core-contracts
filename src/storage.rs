use soroban_sdk::{contracttype, Address, Env};

use crate::errors::ContractError;

/// Storage keys used by the relay contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Address of the currently active relay signer.
    RelaySigner,
    /// Address of the contract administrator.
    Admin,
    /// Boolean flag indicating whether the contract is paused.
    Paused,
}

/// Persist the active relay signer address in instance storage.
pub fn set_relay_signer(env: &Env, signer: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::RelaySigner, signer);
}

/// Read the active relay signer address from instance storage.
///
/// Returns a clean [`ContractError::NotInitialized`] when the contract has not
/// been initialized yet, rather than panicking on a missing storage key.
pub fn get_relay_signer(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::RelaySigner)
        .ok_or(ContractError::NotInitialized)
}

/// Persist the paused flag in instance storage.
pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}

/// Read the paused flag from instance storage.
///
/// Defaults to `false` when the flag has never been written, so a freshly
/// initialized contract is treated as unpaused.
pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

/// Shared circuit-breaker guard.
///
/// Rejects with [`ContractError::ContractPaused`] when the contract is paused.
/// Intended to be called first by every state-mutating entry point so that
/// read-only queries remain available while paused.
pub fn require_not_paused(env: &Env) -> Result<(), ContractError> {
    if is_paused(env) {
        return Err(ContractError::ContractPaused);
    }
    Ok(())
}
