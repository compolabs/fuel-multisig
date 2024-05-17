use fuels::{prelude::*, types::U256};

use crate::utils::setup::{
    deploy_multisig, get_wallets, transfer_parameters, wallets_to_identities,
};

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_change_threshold_then_it_will_throw_not_initialized(
) {
    let wallets = get_wallets(3).await;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Try to call change_threshold before initialization
    let response = deployer.contract.methods().change_threshold(2).call().await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_add_owner_call_then_it_will_throw_not_initialized(
) {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Try to call add_owner before initialization
    let response = deployer
        .contract
        .methods()
        .add_owner(owners_list[0].clone())
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_remove_owner_call_then_it_will_throw_not_initialized(
) {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Try to call remove_owner before initialization
    let response = deployer
        .contract
        .methods()
        .remove_owner(owners_list[0].clone())
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_propose_tx_then_it_will_throw_not_initialized(
) {
    let wallets = get_wallets(3).await;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Get call parameters
    let (_, receiver, transaction_parameters) = transfer_parameters();

    // Try to call propose_tx before initialization
    let response = deployer
        .contract
        .methods()
        .propose_tx(receiver.clone(), 3600, transaction_parameters.clone())
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_approve_tx_then_it_will_throw_not_initialized(
) {
    let wallets = get_wallets(3).await;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Try to call approve_tx before initialization
    let response = deployer
        .contract
        .methods()
        .approve_tx(U256::zero())
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_reject_tx_then_it_will_throw_not_initialized()
{
    let wallets = get_wallets(3).await;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Try to call reject_tx before initialization
    let response = deployer
        .contract
        .methods()
        .reject_tx(U256::zero())
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_execute_tx_then_it_will_throw_not_initialized(
) {
    let wallets = get_wallets(3).await;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Try to call execute_tx before initialization
    let response = deployer
        .contract
        .methods()
        .execute_tx(U256::zero())
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_not_initialized_when_try_to_remove_tx_then_it_will_throw_not_initialized()
{
    let wallets = get_wallets(3).await;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Try to call remove_tx before initialization
    let response = deployer
        .contract
        .methods()
        .remove_tx(U256::zero())
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotInitialized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}
