library;

use ::types::*;

/// Event emitted when the constructor is called
pub struct MultisigInitialized{
    pub contract_id: ContractId,
    pub threshold: u8,
    pub owners: Vec<Identity>
}

/// Event emitted when the threshold is changed
pub struct ThresholdChanged{
    pub new_threshold: u8
}

/// Event emitted when an owner is added
pub struct OwnerAdded{
    pub owner: Identity
}

/// Event emitted when an owner is removed
pub struct OwnerRemoved{
    pub owner: Identity
}

/// Event emitted when a transaction is proposed
pub struct TransactionProposed{
    pub tx_id: TxId,
    pub to: Identity,
    pub transaction_parameters: TransactionParameters,
}

/// Event emitted when a transaction is executed
pub struct TransactionExecuted{
    pub tx_id: TxId
}

/// Event emitted when a transaction is cancelled
pub struct TransactionCancelled{
    pub tx_id: TxId
}

/// Event emitted when a transaction is removed
pub struct TransactionRemoved{
    pub tx_id: TxId
}

/// Event emitted when a transaction is approved
pub struct TransactionApproved{
    pub tx_id: TxId,
    pub owner: Identity
}

/// Event emitted when a transaction is rejected
pub struct TransactionRejected{
    pub tx_id: TxId,
    pub owner: Identity
}