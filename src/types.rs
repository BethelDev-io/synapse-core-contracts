//! Shared types, storage keys, and error codes for the contract.
//!
//! See `DECISIONS.md` §5 for the upgrade entry point that consumes
//! [`ContractError::SchemaVersionMismatch`] and [`StorageKey`].

use soroban_sdk::contracttype;

/// Live on-chain schema version.
///
/// Bumped whenever the persistent/instance storage layout changes in a way
/// that requires migration. `upgrade()` compares the caller-supplied
/// `expected_schema_version` against this constant *before* touching WASM so
/// a stale or malicious caller can never swap in incompatible code.
pub const SCHEMA_VERSION: u32 = 1;

/// Storage key namespace for the contract's persistent/instance entries.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    /// Contract administrator address.
    Admin,
    /// Monotonic transaction counter.
    TxCounter,
    /// Per-transaction record keyed by id.
    Transaction(u64),
    /// Idempotency key (temporary storage only).
    IdempotencyKey(u64),
}

/// Error codes returned by the contract.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    /// Caller is not the contract administrator.
    Unauthorized = 1,
    /// `expected_schema_version` did not match the live schema version.
    SchemaVersionMismatch = 2,
    /// Requested transaction record does not exist.
    NotFound = 3,
}
