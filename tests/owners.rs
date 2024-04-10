use fuels::prelude::*;
use fuels::types::Identity;

use crate::utils::setup::{
    call_parameters_add_owner, call_parameters_remove_owner, deploy_multisig, get_multisig_caller, get_wallets, wallets_to_identities
};

#[tokio::test]
async fn add_owner_works_with_threshold_of_1() {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let new_owner = Identity::Address(Address::from(wallets[2].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_add_owner(new_owner.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    // Execute the tx because the threshold is 1
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    assert!(response.is_ok());

    // Check if owner was added
    let new_owner_was_added = deployer
        .contract
        .methods()
        .is_owner(new_owner)
        .call()
        .await
        .unwrap()
        .value;

    assert!(new_owner_was_added);
}

#[tokio::test]
async fn add_owner_works_with_threshold_of_3() {
    let wallets = get_wallets(4).await;
    let owners_list = wallets_to_identities(wallets[0..3].to_vec());
    let init_threshold = 3;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let new_owner = Identity::Address(Address::from(wallets[3].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_add_owner(new_owner.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    let proposed_tx_id = response.value;

    // Approve the tx from 2 more wallets
    for i in 1..3 {
        let wallet = &wallets[i];

        let caller = get_multisig_caller(&contract_id, wallet.clone());
        let _ = caller
            .contract
            .methods()
            .approve_tx(proposed_tx_id)
            .call()
            .await
            .unwrap();
    }

    // Execute the tx after the threshold is reached
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    assert!(response.is_ok());

    // Check if owner was added
    let new_owner_was_added = deployer
        .contract
        .methods()
        .is_owner(new_owner)
        .call()
        .await
        .unwrap()
        .value;

    assert!(new_owner_was_added);
}

#[tokio::test]
async fn add_owner_fails_with_already_owner() {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let new_owner = Identity::Address(Address::from(wallets[1].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_add_owner(new_owner.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    // Execute the tx because the threshold is 1
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "AlreadyOwner");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }

    // Check that the owner was not added
    let new_owners = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(new_owners, owners_before);
}

#[tokio::test]
async fn add_owner_fails_with_max_owners_reached() {
    let wallets = get_wallets(11).await;
    let owners_list = wallets_to_identities(wallets[0..10].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let new_owner = Identity::Address(Address::from(wallets[10].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_add_owner(new_owner.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    // Execute the tx because the threshold is 1
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "MaxOwnersReached");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }

    // Check that the owner was not added
    let new_owners = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(new_owners, owners_before);
}

#[tokio::test]
async fn remove_owner_works_with_threshold_of_1() {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let owner_to_remove = Identity::Address(Address::from(wallets[1].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_remove_owner(owner_to_remove.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    // Execute the tx because the threshold is 1
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    assert!(response.is_ok());

    // Check if owner was removed
    let new_owner_was_removed = !deployer
        .contract
        .methods()
        .is_owner(owner_to_remove)
        .call()
        .await
        .unwrap()
        .value;

    assert!(new_owner_was_removed);
}

#[tokio::test]
async fn remove_owner_works_with_threshold_of_3() {
    let wallets = get_wallets(4).await;
    let owners_list = wallets_to_identities(wallets[0..4].to_vec());
    let init_threshold = 3;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let owner_to_remove = Identity::Address(Address::from(wallets[3].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_remove_owner(owner_to_remove.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    let proposed_tx_id = response.value;

    // Approve the tx from 2 more wallets
    for i in 1..3 {
        let wallet = &wallets[i];

        let caller = get_multisig_caller(&contract_id, wallet.clone());
        let _ = caller
            .contract
            .methods()
            .approve_tx(proposed_tx_id)
            .call()
            .await
            .unwrap();
    }

    // Execute the tx after the threshold is reached
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    assert!(response.is_ok());

    // Check if owner was removed
    let new_owner_was_removed = !deployer
        .contract
        .methods()
        .is_owner(owner_to_remove)
        .call()
        .await
        .unwrap()
        .value;

    assert!(new_owner_was_removed);
}

#[tokio::test]
async fn remove_owner_fails_with_not_owner() {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let owner_to_remove = Identity::Address(Address::from(wallets[2].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_remove_owner(owner_to_remove.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    // Execute the tx because the threshold is 1
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "NotOwner");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }

    // Check that the owner was not removed
    let new_owners = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(new_owners, owners_before);
}

#[tokio::test]
async fn remove_owner_fails_with_owners_cannot_be_empty() {
    let wallets = get_wallets(1).await;
    let owners_list = wallets_to_identities(wallets[0..1].to_vec());
    let init_threshold = 1;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let owner_to_remove = Identity::Address(Address::from(wallets[0].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_remove_owner(owner_to_remove.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    // Execute the tx because the threshold is 1
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(contract_id)
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "OwnersCannotBeEmpty");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }

    // Check that the owner was not removed
    let new_owners = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(new_owners, owners_before);
}

#[tokio::test]
async fn remove_owner_fails_with_threshold_can_not_be_greater_than_owners() {
    let wallets = get_wallets(2).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let init_threshold = 2;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 1
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let owners_before = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    // Check owners pre-call
    assert_eq!(owners_before, owners_list);

    let owner_to_remove = Identity::Address(Address::from(wallets[0].address()));

    // Get call parameters
    let transaction_parameters = call_parameters_remove_owner(owner_to_remove.clone());

    // Propose the tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    let proposed_tx_id = response.value;

    // Approve the tx from 1 more wallet
    let wallet = &wallets[1];
    let caller = get_multisig_caller(&contract_id, wallet.clone());
    let _ = caller
        .contract
        .methods()
        .approve_tx(proposed_tx_id)
        .call()
        .await
        .unwrap();
    
    // Execute the tx
    let response = deployer
        .contract
        .methods()
        .execute_tx(proposed_tx_id)
        .append_contract(contract_id)
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    match response.err().unwrap() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "ThresholdCannotBeGreaterThanOwners");
        }
        _ => {
            unreachable!("Error should be RevertTransactionError");
        }
    }

    // Check that the owner was not removed
    let new_owners = deployer
        .contract
        .methods()
        .get_owners()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(new_owners, owners_before);
}
