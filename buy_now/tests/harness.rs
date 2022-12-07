use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(MyContract, "out/debug/nft_marketplace-abi.json");
abigen!(MyNFTContract, "../NFT/NFT/out/debug/NFT-abi.json");

async fn get_contract_instances() -> (
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
        "./out/debug/nft_marketplace.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/nft_marketplace-storage_slots.json".to_string(),
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

#[tokio::test]
async fn can_get_contract_id() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances().await;
    // Now you have an instance of your contract you can use to test each function
    let wallet_addr_ident = Identity::Address(wallet.address().into());
    nft_instance
        .methods()
        .constructor(true, wallet_addr_ident, 1)
        .call()
        .await
        .unwrap();
    let admin = nft_instance.methods().admin().call().await.unwrap();
    // println!("{:?}", admin);

    let buy_now_admin = buy_now_instance.methods().admin().call().await.unwrap();
    println!("{:?}", buy_now_admin);
}
