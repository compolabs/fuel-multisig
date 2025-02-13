use fuels::prelude::*;
use fuels::types::Identity;

use crate::utils::setup::{
    call_parameters_add_owner, call_parameters_remove_owner, deploy_multisig, get_multisig_caller,
    get_wallets, wallets_to_identities,
};
use crate::utils::validate_error;

#[tokio::test]
async fn given_a_multisig_with_one_owner_and_threshold_one_when_proposing_a_tx_it_should_be_executed_with_no_errors(
) {
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
        .with_contract_ids(&[contract_id.into()])
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
async fn given_a_multisig_with_threshold_three_when_proposing_a_add_owner_and_is_approved_by_two_it_should_be_executed_with_no_errors(
) {
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
        .with_contract_ids(&[contract_id.into()])
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
async fn given_a_multisig_with_some_owners_when_trying_to_add_an_exisiting_owner_it_should_fail_throwing_already_owner_error(
) {
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
        .with_contract_ids(&[contract_id.into()])
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    validate_error(response, "AlreadyOwner");

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
async fn given_a_multisig_with_max_owners_reached_when_trying_to_add_an_owner_it_should_throw_max_owners_error(
) {
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
        .with_contract_ids(&[contract_id.into()])
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    validate_error(response, "MaxOwnersReached");

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
async fn given_a_multisig_with_threshold_1_and_3_owners_when_trying_to_remove_one_of_them_it_should_be_removed_successfully(
) {
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
        .with_contract_ids(&[contract_id.into()])
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
async fn given_a_multisig_with_threshold_3_and_4_owners_when_trying_to_remove_one_of_them_it_should_be_removed_successfully(
) {
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
        .with_contract_ids(&[contract_id.into()])
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
async fn given_a_multisig_with_threshold_one_and_3_owners_when_trying_to_remove_one_that_is_not_owner_it_should_throw_not_owner_error_and_revert(
) {
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
        .with_contract_ids(&[contract_id.into()])
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    validate_error(response, "NotOwner");

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
async fn given_a_multisig_with_a_single_owner_when_trying_to_remove_that_owner_it_should_throw_non_empty_error_and_revert(
) {
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
        .with_contract_ids(&[contract_id.into()])
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    validate_error(response, "OwnersCannotBeEmpty");

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
async fn given_a_multisig_with_two_owners_and_a_threshold_of_two_when_trying_to_remove_an_owner_it_should_fail_with_verbose_error_and_revert(
) {
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
        .with_contract_ids(&[contract_id.into()])
        .call()
        .await;

    // Check if the tx reverted
    assert!(response.is_err());

    // Check the error
    validate_error(response, "ThresholdCannotBeGreaterThanOwners");

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
