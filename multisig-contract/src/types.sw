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
    tx_id: TxId,
    to: Identity,
    valid_until: u64,
    tx_parameters: InternalTransactionParameters,
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
    calldata: Bytes,
    /// The amount of gas to forward.
    forwarded_gas: u64,
    /// The function selector for the call.
    function_selector: Bytes,
    /// Whether the function being called takes a single value-type argument.
    single_value_type_arg: bool,
    /// Parameters for a transfer.
    transfer_params: TransferParams,
}

/// Parameters for calling a contract.
pub struct InternalContractCallParams {
    /// The amount of gas to forward.
    forwarded_gas: u64,
    /// Whether the function being called takes a single value-type argument.
    single_value_type_arg: bool,
    /// Parameters for a transfer.
    transfer_params: TransferParams,
}

/// Parameters for a transfer.
pub struct TransferParams {
    /// The asset to transfer.
    asset_id: AssetId,
    /// The amount to transfer.
    value: Option<u64>,
}

/// The full data of a transaction.
pub struct TransactionData {
    tx_id: TxId,
    to: Identity,
    valid_until: u64,
    tx_parameters: TransactionParameters,
    approvals_count: u8,
    rejections_count: u8
}
