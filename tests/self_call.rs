use fuels::prelude::*;

use crate::utils::setup::{deploy_multisig, get_wallets, wallets_to_identities};

#[tokio::test]
async fn given_a_multisig_and_a_not_owner_account_when_try_to_change_threshold_from_it_then_should_throw_unauthorized(
) {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    // Try to call change_threshold with an external account
    let response = deployer.contract.methods().change_threshold(2).call().await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "Unauthorized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_and_a_not_owner_account_when_try_to_add_owner_from_it_then_should_throw_unauthorized(
) {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    // Try to call add_owner with an external account
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
            assert_eq!(reason, "Unauthorized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}

#[tokio::test]
async fn given_a_multisig_and_a_not_owner_account_when_try_to_remove_owner_from_it_then_should_throw_unauthorized(
) {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    // Try to call remove_owner with an external account
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
            assert_eq!(reason, "Unauthorized");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }
}
