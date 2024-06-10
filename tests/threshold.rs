use fuels::prelude::*;
use fuels::types::Identity;

use crate::utils::setup::{
    call_parameters_change_threshold, deploy_multisig, get_multisig_caller, get_wallets,
    wallets_to_identities,
};
use crate::utils::validate_error;

#[tokio::test]
async fn given_a_multisig_with_two_owners_a_threshold_of_one_when_propose_to_increment_it_should_be_changed_to_the_new_value(
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

    let threshold_before = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    // Check threshold pre-call
    assert_eq!(threshold_before, init_threshold);

    let new_threshold = 2;

    // Get call parameters
    let transaction_parameters = call_parameters_change_threshold(new_threshold);

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

    // Check threshold post-call
    let threshold_after = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(threshold_after, new_threshold);
}

#[tokio::test]
async fn given_a_multisig_with_4_owners_and_a_threshold_of_3_when_trying_to_set_threshold_to_4_it_shoudl_be_changed_as_proposed(
) {
    let wallets = get_wallets(4).await;
    let owners_list = wallets_to_identities(wallets[0..4].to_vec());
    let init_threshold = 3;

    // Deploy the multisig contract
    let (contract_id, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor with threshold 3/4
    let _ = deployer
        .contract
        .methods()
        .constructor(init_threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    let threshold_before = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    // Check threshold pre-call
    assert_eq!(threshold_before, init_threshold);

    let new_threshold = 4;

    // Get call parameters
    let transaction_parameters = call_parameters_change_threshold(new_threshold);

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

    // Check threshold post-call
    let threshold_after = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(threshold_after, new_threshold);
}

#[tokio::test]
async fn given_a_multisig_when_trying_to_set_threshold_to_zero_it_should_fail_and_throw_error() {
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

    let threshold_before = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    // Check threshold pre-call
    assert_eq!(threshold_before, init_threshold);

    let new_threshold = 0;

    // Get call parameters
    let transaction_parameters = call_parameters_change_threshold(new_threshold);

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
    validate_error(response, "ThresholdCannotBeZero");

    // Check threshold post-call
    let threshold_after = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(threshold_after, init_threshold);
}

#[tokio::test]
async fn given_a_multisig_of_2_owners_and_threshold_of_1_when_trying_to_set_threshold_to_3_it_should_fail_and_throw_error(
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

    let threshold_before = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    // Check threshold pre-call
    assert_eq!(threshold_before, init_threshold);

    let new_threshold = 3;

    // Get call parameters
    let transaction_parameters = call_parameters_change_threshold(new_threshold);

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
    validate_error(response, "ThresholdCannotBeGreaterThanOwners");

    // Check threshold post-call
    let threshold_after = deployer
        .contract
        .methods()
        .get_threshold()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(threshold_after, init_threshold);
}
