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
///
/// Uses the plain (non-namespaced) variant convention restored per
/// `QUARANTINE.md` ("namespaced storage keys reverted to plain `StorageKey`
/// variants"). Associated data is kept minimal and copy-cheap because a key
/// is constructed on every storage access.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    /// Contract administrator address.
    Admin,
    /// Address authorized to relay signed transactions.
    RelaySigner,
    /// Address nominated to become the next administrator (two-step transfer).
    PendingAdmin,
    /// Monotonic transaction counter.
    TxCounter,
    /// Pause flag gating state-changing entry points.
    Paused,
    /// Default per-transaction ceiling.
    DefaultCeiling,
    /// Per-transaction record keyed by id.
    Transaction(u64),
    /// Status-index slot keyed by status and transaction id.
    StatusIndexSlot(u32, u64),
    /// Status-index count keyed by status.
    StatusIndexCount(u32),
    /// Idempotency key (temporary storage only).
    IdempotencyKey(u64),
    /// Relay approval record keyed by transaction id.
    RelayApproval(u64),
    /// Param-registry entry keyed by parameter id.
    ParamRegistryEntry(u32),
    /// Bond record keyed by address.
    BondRecord(u64),
    /// Treasury balance.
    TreasuryBalance,
    /// Treasury configuration.
    TreasuryConfig,
    /// Upgrade history record keyed by sequence number.
    UpgradeHistory(u32),
    /// Pending upgrade awaiting execution.
    PendingUpgrade,
    /// Schema-compatibility range (min, max) accepted by `upgrade()`.
    SchemaCompatRange,
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
    /// Caller authenticated via `require_auth()` but is not authorized for
    /// this operation (distinct from a missing `require_auth()` call, which
    /// the SDK raises on its own).
    NotAuthorized = 4,
}
