# Fuel Multisig

This project provides a fully on chain multisig to manage shared assets and interact with other contracts, requiring multiple signatures to authorize transactions.

## Introduction
A multisig contract requires multiple parties to sign off on transactions before they are executed. This enhances security by distributing control among multiple owners.

## Features
- **Multiple Owners:** Assign multiple addresses or contracts as owners.
- **Configurable Threshold:** Set the number of required approvals for a transaction.
- **Propose and Execute Transactions:** Secure transaction management with propose, approve, reject and execute methods.
- **Transaction Removal:** Remove unapproved transactions after a timeout or if the approval threshold can not met.

## Getting Started
To get started with the multisig contract, clone the repository and build the contract:

```
git clone https://github.com/protofire/fuel-multisig
cd fuel-multisig
forc build --release
```

## Testing
To run the tests, build the contract in debug mode and execute the following command:

```
forc build && cd tests && cargo test
```

## User Flow
Here’s a step-by-step guide on how to use the multisig contract:

1. **Deploy the Multisig Contract**

    Deploy the multisig contract on the Fuel network using your preferred method.

2. **Initialize the Contract**

    After deployment, call the constructor function to initialize the contract. You need to provide a list of owner addresses and set the approval threshold.

3. **Propose a Transaction**

    Any owner can propose a new transaction using the propose_tx method. The proposal includes details such as the recipient address, amount, and any additional data required.

4. **Approve or Reject the Transaction**

    Other owners can approve or reject the proposed transaction. Approval is done by calling the approve_tx method, and rejection is done using the reject_tx method.

5. **Execute the Transaction**

    Once the proposed transaction has received the required number of approvals (threshold), it can be executed by calling the execute_tx method.

6. **Remove an Unapproved Transaction**

    If a transaction is not executed within a certain timeframe, or if it is clear that the threshold will not be met, it can be removed using the remove_tx method.