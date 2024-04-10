use fuels::macros::abigen;

abigen!(
    Contract(
        name = "Multisig",
        abi = "../multisig-contract/out/debug/fuel-multisig-abi.json"
    ),
    Contract(
        name = "Counter",
        abi = "./utils/test-contracts/counter/out/debug/counter-abi.json"
    )
);
