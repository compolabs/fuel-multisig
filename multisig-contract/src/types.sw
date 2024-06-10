library;

use std::storage::storage_bytes::*;
use std::bytes::Bytes;
use std::asset_id::AssetId;

pub type TxId = u256;
pub type Approvals = u8;
pub type Rejections = u8;

pub const MAX_OWNERS: u8 = 10;
pub const MAX_TRANSACTIONS: u8 = 10;

/// The transaction that is being proposed.
pub struct Transaction {
    pub tx_id: TxId,
    pub to: Identity,
    pub valid_until: u64,
    pub tx_parameters: InternalTransactionParameters,
}

/// Determines the type of transaction parameters.
pub enum TransactionParameters {
    Call: ContractCallParams,
    Transfer: TransferParams,
}

pub enum InternalTransactionParameters {
    Call: InternalContractCallParams,
    Transfer: TransferParams,
}

/// Parameters for calling a contract.
pub struct ContractCallParams {
    /// The calldata for the call.
    pub calldata: Bytes,
    /// The amount of gas to forward.
    pub forwarded_gas: u64,
    /// The function selector for the call.
    pub function_selector: Bytes,
    /// Parameters for a transfer.
    pub transfer_params: TransferParams,
}

/// Parameters for calling a contract.
pub struct InternalContractCallParams {
    /// The amount of gas to forward.
    pub forwarded_gas: u64,
    /// Parameters for a transfer.
    pub transfer_params: TransferParams,
}

/// Parameters for a transfer.
pub struct TransferParams {
    /// The asset to transfer.
    pub asset_id: AssetId,
    /// The amount to transfer.
    pub value: Option<u64>,
}

/// The full data of a transaction.
pub struct TransactionData {
    pub tx_id: TxId,
    pub to: Identity,
    pub valid_until: u64,
    pub tx_parameters: TransactionParameters,
    pub approvals_count: u8,
    pub rejections_count: u8
}
