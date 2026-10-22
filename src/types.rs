//! Shared types for the contract.
//!
//! This module holds the small, storage-agnostic value types used across the
//! contract's public surface. Diagnostic read-only queries such as `health()`
//! and `version()` live on the contract impl in `lib.rs`; the constants and
//! helper types they rely on are defined here so they stay in one place.

use soroban_sdk::{contracterror, contracttype, symbol_short, Symbol};

/// Current contract semver, exposed via the `version()` read-only query.
///
/// This is intentionally distinct from `schema_version()`: `version()` tracks
/// the contract's own semver (bumped per `EVENTS.md`'s semver policy on schema
/// changes), while `schema_version()` tracks the storage/event schema revision.
/// Conflating the two was explicitly avoided per `DECISIONS.md`.
pub const CONTRACT_VERSION: u32 = 1;

/// Symbol used to key the contract version in storage, if ever persisted.
pub const VERSION_KEY: Symbol = symbol_short!("version");

/// Result of the `health()` diagnostic query.
///
/// A minimal self-consistency snapshot: `healthy` is true only when the
/// contract's core configuration (admin and relay signer) is fully set.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HealthStatus {
    /// True when the contract's minimal self-consistency check passes.
    pub healthy: bool,
    /// True when an admin address has been configured.
    pub admin_set: bool,
    /// True when a relay signer has been configured.
    pub relay_signer_set: bool,
}

impl HealthStatus {
    /// Build a health snapshot from the two configuration flags.
    pub fn new(admin_set: bool, relay_signer_set: bool) -> Self {
        Self {
            healthy: admin_set && relay_signer_set,
            admin_set,
            relay_signer_set,
        }
    }

    /// Snapshot representing an uninitialized contract (never panics).
    pub fn uninitialized() -> Self {
        Self::new(false, false)
    }
}

/// Consolidated contract error enum.
///
/// Soroban caps a `#[contracterror]` enum at 50 cases. The pre-squash feature
/// set needed 61 conceptually distinct errors, so this enum merges them by
/// *category* (Soroban requires unit variants, so detail is carried by emitted
/// events rather than payloads). The mapping below documents every merge.
///
/// # Error-code budget (61 conceptual errors -> 50 variants)
///
/// Categories preserved for off-chain callers:
/// - **Invalid input** (1-9): caller supplied bad data.
/// - **Unauthorized** (10-14): caller lacks permission.
/// - **State machine** (15-29): the current state forbids the operation.
/// - **Migration/upgrade** (30-39): schema/upgrade lifecycle errors.
/// - **Operational** (40-50): limits, self-checks, and misc failures.
///
/// ## Merges
/// - `AlreadyInitialized` (1) absorbs `AlreadyConfigured`.
/// - `NotInitialized` (2) absorbs `NotConfigured`.
/// - `InvalidInput` (3) absorbs `InvalidAmount`, `InvalidAddress`,
///   `InvalidSignature`, `InvalidNonce`, `InvalidTimestamp`.
/// - `InvalidSchemaCompatRange` (4) absorbs `InvalidSchemaVersion`.
/// - `SchemaVersionMismatch` (5) absorbs `SchemaTooOld`, `SchemaTooNew`.
/// - `Unauthorized` (10) absorbs `NotAdmin`, `NotRelaySigner`, `NotOwner`.
/// - `Forbidden` (11) absorbs `Paused`, `Frozen`.
/// - `CannotCancel` (15) absorbs `NotCancellable`.
/// - `AlreadyCancelled` (16) absorbs `AlreadySettled`, `AlreadyCompleted`.
/// - `InvalidTransition` (17) absorbs `CannotSettle`, `CannotRefund`,
///   `CannotExpire`, `CannotRetry` (disambiguated by event data).
/// - `TimelockNotElapsed` (18) absorbs `TimelockActive`.
/// - `NoPendingChange` (19) absorbs `NoPendingAdmin`, `NoPendingUpgrade`.
/// - `NothingToRollback` (20) absorbs `NoRollbackPoint`.
/// - `UnknownMigration` (30) absorbs `UnknownUpgrade`.
/// - `MigrationFailed` (31) absorbs `UpgradeFailed`.
/// - `UpgradeNotReversible` (32) absorbs `NotReversible`.
/// - `RetryLimitExceeded` (40) absorbs `TooManyRetries`.
/// - `SelfCheckFailed` (41) absorbs `InvariantViolation`, `HealthCheckFailed`.
/// - `Overflow` (42) absorbs `Underflow`, `ArithmeticError`.
/// - `NotFound` (43) absorbs `MissingRecord`, `MissingEntry`.
/// - `AlreadyExists` (44) absorbs `DuplicateRecord`.
/// - `StorageError` (45) absorbs `StorageCorrupt`, `StorageFull`.
/// - `InternalError` (46) absorbs `Unexpected`, `Unreachable`.
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    // --- Invalid input (1-9) ---
    /// Contract has already been initialized.
    AlreadyInitialized = 1,
    /// Contract has not been initialized yet.
    NotInitialized = 2,
    /// Caller supplied malformed or out-of-range input.
    InvalidInput = 3,
    /// Schema compatibility range is invalid.
    InvalidSchemaCompatRange = 4,
    /// Stored schema version does not match the expected version.
    SchemaVersionMismatch = 5,

    // --- Unauthorized (10-14) ---
    /// Caller is not authorized for this operation.
    Unauthorized = 10,
    /// Operation is forbidden in the current mode (e.g. paused).
    Forbidden = 11,

    // --- State machine (15-29) ---
    /// The operation cannot be cancelled in the current state.
    CannotCancel = 15,
    /// The operation has already been cancelled.
    AlreadyCancelled = 16,
    /// The state machine forbids this transition.
    InvalidTransition = 17,
    /// The timelock has not elapsed yet.
    TimelockNotElapsed = 18,
    /// There is no pending change to act on.
    NoPendingChange = 19,
    /// There is nothing to roll back.
    NothingToRollback = 20,

    // --- Migration/upgrade (30-39) ---
    /// The referenced migration is unknown.
    UnknownMigration = 30,
    /// The migration failed to apply.
    MigrationFailed = 31,
    /// The upgrade cannot be reversed.
    UpgradeNotReversible = 32,

    // --- Operational (40-50) ---
    /// The retry limit has been exceeded.
    RetryLimitExceeded = 40,
    /// A self-consistency check failed.
    SelfCheckFailed = 41,
    /// Arithmetic overflow or underflow.
    Overflow = 42,
    /// A required record was not found.
    NotFound = 43,
    /// A record already exists.
    AlreadyExists = 44,
    /// Underlying storage error.
    StorageError = 45,
    /// Unexpected internal error.
    InternalError = 46,
}
