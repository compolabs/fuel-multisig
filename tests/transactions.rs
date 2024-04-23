use fuels::types::Identity;
use fuels::prelude::*;

use crate::utils::constants::DEFAULT_TRANSFER_AMOUNT;
use crate::utils::setup::{
    base_asset_contract_id, call_parameters, deploy_counter, deploy_multisig, get_wallets,
    transfer_parameters, wallets_to_identities,
};

#[tokio::test]
async fn propose_call_tx() {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let threshold = 1;

    // Deploy the counter contract
    let (counter_contract_id, counter_deployer) = deploy_counter(&wallets[0]).await.unwrap();

    // Deploy the multisig contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the multisig constructor
    let _ = deployer
        .contract
        .methods()
        .constructor(threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    // Transfer some funds to the contract
    deployer
        .wallet
        .force_transfer_to_contract(
            deployer.contract.contract_id(),
            DEFAULT_TRANSFER_AMOUNT,
            BASE_ASSET_ID,
            TxPolicies::default(),
        )
        .await
        .unwrap();

    // Check counter pre-call
    let initial_counter_value = counter_deployer
        .contract
        .methods()
        .get_counter()
        .call()
        .await
        .unwrap()
        .value;

    // Get call parameters
    let transaction_parameters = call_parameters();

    // Propose a call tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(
            Identity::ContractId(counter_contract_id.clone().into()),
            3600,
            transaction_parameters.clone(),
        )
        .call()
        .await
        .unwrap();

    // Execute the call tx because the threshold is 1
    let response = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_contract(counter_contract_id)
        .call()
        .await;

    assert!(response.is_ok());

    // Check counter post-call
    let final_counter_value = counter_deployer
        .contract
        .methods()
        .get_counter()
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(initial_counter_value, 0);
    assert_eq!(final_counter_value, initial_counter_value + 5);
}

#[tokio::test]
async fn propose_transfer_tx() {
    let wallets = get_wallets(3).await;
    let owners_list = wallets_to_identities(wallets[0..2].to_vec());
    let threshold = 1;

    // Deploy the contract
    let (_, deployer) = deploy_multisig(&wallets[0]).await.unwrap();

    // Call the constructor
    let _ = deployer
        .contract
        .methods()
        .constructor(threshold, owners_list.clone())
        .call()
        .await
        .unwrap();

    // Transfer some funds to the contract
    deployer
        .wallet
        .force_transfer_to_contract(
            deployer.contract.contract_id(),
            DEFAULT_TRANSFER_AMOUNT,
            BASE_ASSET_ID,
            TxPolicies::default(),
        )
        .await
        .unwrap();

    // Check balances pre-transfer
    let initial_contract_balance = deployer
        .wallet
        .provider()
        .unwrap()
        .get_contract_asset_balance(deployer.contract.contract_id(), base_asset_contract_id())
        .await
        .unwrap();

    // Get transfer parameters
    let (receiver_wallet, receiver, transaction_parameters) = transfer_parameters();

    let initial_receiver_balance = deployer
        .wallet
        .provider()
        .unwrap()
        .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
        .await
        .unwrap();

    // Propose a transfer tx
    let response = deployer
        .contract
        .methods()
        .propose_tx(receiver, 3600, transaction_parameters.clone())
        .call()
        .await
        .unwrap();

    // Execute the transfer tx because the threshold is 1
    let _ = deployer
        .contract
        .methods()
        .execute_tx(response.value)
        .append_variable_outputs(1)
        .call()
        .await;

    // check balances post-transfer
    let final_contract_balance = deployer
        .wallet
        .provider()
        .unwrap()
        .get_contract_asset_balance(deployer.contract.contract_id(), base_asset_contract_id())
        .await
        .unwrap();
    let final_receiver_balance = deployer
        .wallet
        .provider()
        .unwrap()
        .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
        .await
        .unwrap();

    assert_eq!(initial_contract_balance, DEFAULT_TRANSFER_AMOUNT);
    assert_eq!(initial_receiver_balance, 0);

    assert_eq!(final_contract_balance, 0);
    assert_eq!(final_receiver_balance, DEFAULT_TRANSFER_AMOUNT);

    assert!(final_contract_balance < initial_contract_balance);
    assert!(final_receiver_balance > initial_receiver_balance);
}
