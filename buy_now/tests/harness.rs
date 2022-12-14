use fuels::{prelude::*, tx::ContractId};
use std::str::FromStr;

// Load abi from json
abigen!(MyContract, "out/debug/buy_now-abi.json");
abigen!(MyNFTContract, "../NFT/NFT/out/debug/NFT-abi.json");

async fn get_contract_instances_and_wallet() -> (
    MyContract,
    ContractId,
    MyNFTContract,
    ContractId,
    WalletUnlocked,
) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let buy_now_contract_id = Contract::deploy(
        "./out/debug/buy_now.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/buy_now-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let nft_contract_id = Contract::deploy(
        "../NFT/NFT/out/debug/NFT.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "../NFT/NFT/out/debug/NFT-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let buy_now_instance = MyContract::new(buy_now_contract_id.to_string(), wallet.clone());
    let nft_instance = MyNFTContract::new(nft_contract_id.to_string(), wallet.clone());

    (
        buy_now_instance,
        buy_now_contract_id.into(),
        nft_instance,
        nft_contract_id.into(),
        wallet,
    )
}

// get admin buy now contract test
#[tokio::test]
async fn get_admin_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances_and_wallet().await;

    let test_admin = Identity::Address(
        Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
            .expect("failed to create Address from string"),
    );

    let admin_call_response = buy_now_instance.methods().admin().call().await.unwrap();

    assert_eq!(test_admin, admin_call_response.value);
}

// set admin buy now contract test
#[tokio::test]
async fn set_admin_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances_and_wallet().await;
    // Now you have an instance of your contract you can use to test each function
    let wallet_addr_ident = Identity::Address(wallet.address().into());

    buy_now_instance.methods().set_admin(
        Identity::Address(
            Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .expect("failed to create Address from string"),
        )
    ).call().await.unwrap();

    let test_admin = Identity::Address(
        Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
            .expect("failed to create Address from string"),
    );

    let admin_call_response = buy_now_instance.methods().admin().call().await.unwrap();

    assert_eq!(test_admin, admin_call_response.value);
    
}
