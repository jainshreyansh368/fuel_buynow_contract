use fuels::{prelude::*, tx::ContractId};
use std::str::FromStr;

// Load abi from json
abigen!(MyContract, "./out/debug/buy_now-abi.json");
abigen!(MyNFTContract, "./../NFT/NFT/out/debug/NFT-abi.json");

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

#[tokio::test]
async fn list_nft_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances_and_wallet().await;
    // Now you have an instance of your contract you can use to test each function
    let wallet_addr_ident = Identity::Address(wallet.address().into());

    nft_instance.methods().constructor(true, 
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        100
    ).call().await.unwrap();

    let supply_total = nft_instance.methods().total_supply().call().await.unwrap();

    nft_instance.methods().mint(
        1,
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        SizedAsciiString::<35>::new("exampleooiiuuyyttrreegghhddkkllssmm".to_string()).unwrap(), 
        SizedAsciiString::<59>::new("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string()).unwrap(),
        [   Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            )
        ],
    ).call().await.unwrap();
    println!("nft contract id ::  {:?}", nft_id);
    println!("buynow contract id ::  {:?}", buy_now_id);

    nft_instance.methods().approve(Identity::ContractId(buy_now_id), 0).call().await.unwrap();
    // ).call().await.unwrap();

    buy_now_instance.methods().list_nft(
            nft_id.clone(),
            0,
            10,
    ).set_contracts(&[nft_id.into()])
    .call().await.unwrap();

    let nft_owner = nft_instance.methods().owner_of(supply_total.value).call().await.unwrap();

    assert_eq!(Identity::ContractId(buy_now_id), nft_owner.value);
}


#[tokio::test]
async fn buy_nft_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances_and_wallet().await;
    // Now you have an instance of your contract you can use to test each function
    let wallet_addr_ident = Identity::Address(wallet.address().into());

    nft_instance.methods().constructor(true, 
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        100
    ).call().await.unwrap();

    let supply_total = nft_instance.methods().total_supply().call().await.unwrap();

    nft_instance.methods().mint(
        1,
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        SizedAsciiString::<35>::new("exampleooiiuuyyttrreegghhddkkllssmm".to_string()).unwrap(), 
        SizedAsciiString::<59>::new("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string()).unwrap(),
        [   Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            )
        ],
    ).call().await.unwrap();
    println!("nft contract id ::  {:?}", nft_id);
    println!("buynow contract id ::  {:?}", buy_now_id);

    nft_instance.methods().approve(Identity::ContractId(buy_now_id), 0).call().await.unwrap();
    // ).call().await.unwrap();

    buy_now_instance.methods().list_nft(
            nft_id.clone(),
            0,
            10,
    ).set_contracts(&[nft_id.into()])
    .call().await.unwrap();

    let nft_owner = nft_instance.methods().owner_of(supply_total.value).call().await.unwrap();

    assert_eq!(Identity::ContractId(buy_now_id), nft_owner.value);

    buy_now_instance.methods().buy_nft(
        nft_id.clone(), 
        0
    ).set_contracts(&[nft_id.into()])
    .call_params(CallParameters::new(Some(10), None, None))
    .call().await.unwrap();

    let test_admin = Identity::Address(
        Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
            .expect("failed to create Address from string"),
    );

    let nft_owner = nft_instance.methods().owner_of(supply_total.value).call().await.unwrap();
    
    assert_eq!(test_admin, nft_owner.value);

}


#[tokio::test]
async fn delist_nft_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances_and_wallet().await;
    // Now you have an instance of your contract you can use to test each function
    let wallet_addr_ident = Identity::Address(wallet.address().into());

    nft_instance.methods().constructor(true, 
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        100
    ).call().await.unwrap();

    let supply_total = nft_instance.methods().total_supply().call().await.unwrap();

    nft_instance.methods().mint(
        1,
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        SizedAsciiString::<35>::new("exampleooiiuuyyttrreegghhddkkllssmm".to_string()).unwrap(), 
        SizedAsciiString::<59>::new("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string()).unwrap(),
        [   Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            )
        ],
    ).call().await.unwrap();
    println!("nft contract id ::  {:?}", nft_id);
    println!("buynow contract id ::  {:?}", buy_now_id);

    nft_instance.methods().approve(Identity::ContractId(buy_now_id), 0).call().await.unwrap();
    // ).call().await.unwrap();

    buy_now_instance.methods().list_nft(
            nft_id.clone(),
            0,
            10,
    ).set_contracts(&[nft_id.into()])
    .call().await.unwrap();

    buy_now_instance.methods().delist_nft(
        nft_id.clone(),
        0,
    ).set_contracts(&[nft_id.into()])
    .call().await.unwrap();
    let nft_owner = nft_instance.methods().owner_of(supply_total.value).call().await.unwrap();

    let test_admin = Identity::Address(
        Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
            .expect("failed to create Address from string"),
    );
    assert_eq!(test_admin, nft_owner.value);
}

#[tokio::test]
async fn make_offer_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances_and_wallet().await;

    nft_instance
        .methods()
        .constructor(
            true,
            Identity::Address(
                Address::from_str(
                    "0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db",
                )
                .expect("failed to create Address from string"),
            ),
            100,
        )
        .call()
        .await
        .unwrap();

    nft_instance
        .methods()
        .mint(
            1,
            Identity::Address(
                Address::from_str(
                    "0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db",
                )
                .expect("failed to create Address from string"),
            ),
            SizedAsciiString::<35>::new("exampleooiiuuyyttrreegghhddkkllssmm".to_string()).unwrap(),
            SizedAsciiString::<59>::new(
                "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string(),
            )
            .unwrap(),
            [
                Identity::Address(
                    Address::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .expect("failed to create Address from string"),
                ),
                Identity::Address(
                    Address::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .expect("failed to create Address from string"),
                ),
                Identity::Address(
                    Address::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .expect("failed to create Address from string"),
                ),
                Identity::Address(
                    Address::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .expect("failed to create Address from string"),
                ),
                Identity::Address(
                    Address::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .expect("failed to create Address from string"),
                ),
            ],
        )
        .call()
        .await
        .unwrap();

        nft_instance.methods().approve(Identity::ContractId(buy_now_id), 0).call().await.unwrap();
        // ).call().await.unwrap();
    
        buy_now_instance.methods().list_nft(
                nft_id.clone(),
                0,
                10,
        ).set_contracts(&[nft_id.into()])
        .call().await.unwrap();

        buy_now_instance.methods().make_offer(
            nft_id.clone(),
            0,
            10,
        ).set_contracts(&[nft_id.into()])
        .call_params(CallParameters::new(Some(10), None, None))
        .call().await.unwrap();
}

#[tokio::test]
async fn accept_offer_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
    get_contract_instances_and_wallet().await;

nft_instance
    .methods()
    .constructor(
        true,
        Identity::Address(
            Address::from_str(
                "0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db",
            )
            .expect("failed to create Address from string"),
        ),
        100,
    )
    .call()
    .await
    .unwrap();

nft_instance
    .methods()
    .mint(
        1,
        Identity::Address(
            Address::from_str(
                "0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db",
            )
            .expect("failed to create Address from string"),
        ),
        SizedAsciiString::<35>::new("exampleooiiuuyyttrreegghhddkkllssmm".to_string()).unwrap(),
        SizedAsciiString::<59>::new(
            "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string(),
        )
        .unwrap(),
        [
            Identity::Address(
                Address::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .expect("failed to create Address from string"),
            ),
            Identity::Address(
                Address::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .expect("failed to create Address from string"),
            ),
            Identity::Address(
                Address::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .expect("failed to create Address from string"),
            ),
            Identity::Address(
                Address::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .expect("failed to create Address from string"),
            ),
            Identity::Address(
                Address::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .expect("failed to create Address from string"),
            ),
        ],
    )
    .call()
    .await
    .unwrap();

    nft_instance.methods().approve(Identity::ContractId(buy_now_id), 0).call().await.unwrap();
    // ).call().await.unwrap();

    buy_now_instance.methods().list_nft(
            nft_id.clone(),
            0,
            10,
    ).set_contracts(&[nft_id.into()])
    .call().await.unwrap();

    buy_now_instance.methods().make_offer(
        nft_id.clone(),
        0,
        10,
    ).set_contracts(&[nft_id.into()])
    .call_params(CallParameters::new(Some(10), None, None))
    .call().await.unwrap();

    buy_now_instance.methods().accept_offer(
        nft_id.clone(),
        0,
        10,
    ).set_contracts(&[nft_id.into()])
    .call_params(CallParameters::new(Some(10), None, None))
    .call().await.unwrap();

}


#[tokio::test]
async fn change_nft_price_test() {
    let (buy_now_instance, buy_now_id, nft_instance, nft_id, wallet) =
        get_contract_instances_and_wallet().await;
    // Now you have an instance of your contract you can use to test each function
    let wallet_addr_ident = Identity::Address(wallet.address().into());

    nft_instance.methods().constructor(true, 
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        100
    ).call().await.unwrap();

    let supply_total = nft_instance.methods().total_supply().call().await.unwrap();

    nft_instance.methods().mint(
        1,
        Identity::Address(
            Address::from_str("0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db")
                .expect("failed to create Address from string"),
        ),
        SizedAsciiString::<35>::new("exampleooiiuuyyttrreegghhddkkllssmm".to_string()).unwrap(), 
        SizedAsciiString::<59>::new("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string()).unwrap(),
        [   Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            ),        
            Identity::Address(
                Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .expect("failed to create Address from string"),
            )
        ],
    ).call().await.unwrap();
    println!("nft contract id ::  {:?}", nft_id);
    println!("buynow contract id ::  {:?}", buy_now_id);

    nft_instance.methods().approve(Identity::ContractId(buy_now_id), 0).call().await.unwrap();
    // ).call().await.unwrap();

    buy_now_instance.methods().list_nft(
            nft_id.clone(),
            0,
            10,
    ).set_contracts(&[nft_id.into()])
    .call().await.unwrap();

    let nft_owner = nft_instance.methods().owner_of(supply_total.value).call().await.unwrap();

    assert_eq!(Identity::ContractId(buy_now_id), nft_owner.value);

    buy_now_instance.methods().change_nft_price(
        nft_id.clone(),
        0, 
        13
    ).call().await.unwrap();

    let new_price = buy_now_instance.methods().nft_price(
        nft_id.clone(),
        0
    ).call().await.unwrap();

    assert_eq!(13, new_price.value);
}