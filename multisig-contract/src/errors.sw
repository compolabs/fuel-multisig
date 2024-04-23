library;

// Errors
pub enum MultisigError {
    /// Maximum number of owners reached
    MaxOwnersReached: (),
    /// The account is already an owner
    AlreadyOwner: (),
    /// The account is not an owner
    NotOwner: (),
    /// Duplicate owner
    DuplicatedOwner: (),
    /// Owners cannot be empty
    OwnersCannotBeEmpty: (),
    /// Threshold cannot be zero
    ThresholdCannotBeZero: (),
    /// Threshold cannot be greater than the number of owners
    ThresholdCannotBeGreaterThanOwners: (),
    /// The threshold has not been reached
    ThresholdNotReached: (),
    /// The transaction is still valid
    TransactionStillValid: (),
    /// The account has already voted
    AlreadyVoted: (),
    /// Transaction ID does not exist
    InvalidTxId: (),
    /// The transaction can only be executed by the multisig contract itself
    Unauthorized: (),
    /// The multisig contract is not initialized
    NotInitialized: (),
    /// The multisig contract is already initialized
    AlreadyInitialized: (),
    /// The multisig does not have enough funds
    InsufficientAssetAmount: (),
    /// Only contracts can be called
    CanOnlyCallContracts: (),
    /// Transfer requires a value to be sent
    TransferRequiresAValue: (),
    /// The transaction has expired
    TransactionExpired: (),
    /// Maximum number of transactions reached
    MaxTransactionsReached: (),
}
