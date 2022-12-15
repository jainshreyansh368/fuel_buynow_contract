contract;

dep data_structure;
dep errors;
dep events;
dep interface;
dep external_interface;

use data_structure::{
    ListNft,
    OfferNft,
};
use errors::{AccessError, InputError};
use events::{
    AdminChangedEvent,
    NFTBoughtEvent,
    NFTDeListedEvent,
    NFTListedEvent,
    NFTPriceChangeEvent,
    NFTOfferEvent,
    NFTOfferAcceptEvent,
    NFTChangeOfferEvent,
};
use interface::NftMarketplace;
use external_interface::externalAbi;

use std::{
    chain::auth::msg_sender,
    context::{call_frames::contract_id, msg_amount},
    logging::log,
    storage::StorageMap,
    token::transfer,
    constants::BASE_ASSET_ID,
};

storage {
    // Stores the user that is permitted to be handle the admin Operations of the contract.
    // Only the `admin` is allowed to change the `admin` of the contract.
    admin: Option<Identity> = Option::Some(Identity::Address(~Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db))),
    // // Total Number of NFTS Listed on Platform
    // no_of_nft_listed: u64 = 0,
    // //No of NFts listed on platform by a single user
    // // Map(user(Identity) => no_of_nft_listed)
    // no_of_nft_listed_by_user: StorageMap<Option<Identity>, u64> = StorageMap{},
    platform_fee_account: Identity = Identity::Address(~Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db)),
    protocol_fee: u64 = 0,
    nft_listed: StorageMap<(Option<ContractId>, u64), bool> = StorageMap {},
    list_nft: StorageMap<(Option<ContractId>, u64), ListNft> = StorageMap {},

    offer_nft: StorageMap<(Option<ContractId>, u64), OfferNft> = StorageMap {},

}

impl NftMarketplace for Contract {
    #[storage(read)]
    fn admin() -> Identity {
        let admin = storage.admin;
        require(admin.is_some(), InputError::AdminDoesNotExist);
        admin.unwrap()
    }

    #[storage(read, write)]
    fn set_admin(admin: Identity) {
        // Ensure that the sender is the admin
        let admin = Option::Some(admin);
        let current_admin = storage.admin;
        require(current_admin.is_some() && msg_sender().unwrap() == current_admin.unwrap(), AccessError::SenderCannotSetAccessControl);
        storage.admin = admin;

        log(AdminChangedEvent {
            mew_admin: admin.unwrap(),
        });
    }

    #[storage(read, write)]
    fn list_nft(id: ContractId, token_id: u64, price: u64) {
        require(price != 0, InputError::PriceCantBeZero);
        require(!storage.nft_listed.get((Option::Some(id), token_id)), AccessError::NFTAlreadyListed);

        let nft_contract: b256 = id.into();
        let this_contract = Identity::ContractId(contract_id());

        // todo ContractNotInInputs error
        let x = abi(externalAbi, nft_contract);
        let owner = x.owner_of(token_id);
        require(owner == msg_sender().unwrap(), AccessError::SenderNotOwner);
        x.transfer_from(msg_sender().unwrap(), this_contract, token_id);
        storage.nft_listed.insert((Option::Some(id), token_id), true);

        let nft = ListNft {
            owner: msg_sender().unwrap(),
            price: price,
        };
        storage.list_nft.insert((Option::Some(id), token_id), nft);

        log(NFTListedEvent {
            owner: msg_sender().unwrap(),
            nft_contract: id,
            token_id: token_id,
            price: price,
        });
    }

    #[storage(read, write)]
    fn delist_nft(id: ContractId, token_id: u64) {
        require(storage.nft_listed.get((Option::Some(id), token_id)), AccessError::NFTNotListed);
        let nft_data = storage.list_nft.get((Option::Some(id), token_id));
        require(nft_data.owner == msg_sender().unwrap(), AccessError::SenderNotOwner);

        let nft_contract: b256 = id.into();
        let this_contract = Identity::ContractId(contract_id());

        // todo ContractNotInInputs error
        let x = abi(externalAbi, nft_contract);

        x.transfer_from(this_contract, msg_sender().unwrap(), token_id);
        
        storage.nft_listed.insert((Option::Some(id), token_id), false);


        // TODO: if we have `nft_listed` field in the contract we don't need to update/write in the contract
        // let nft = ListNft{
        //     owner: Option::None(),
        //     price: price,
        // };
        // storage.list_nft.insert((Option::Some(id), token_id), nft);
        
        log(NFTDeListedEvent {
            owner: msg_sender().unwrap(),
            nft_contract: id,
            token_id: token_id,
        });
    }


    #[storage(read, write)]
    fn make_offer(id: ContractId, token_id: u64, price: u64) {
        require(price != 0, InputError::PriceCantBeZero);
        require(storage.nft_listed.get((Option::Some(id), token_id)), AccessError::NFTNotListed);

        let data = storage.offer_nft.get((Option::Some(id), token_id));

        require(msg_amount() > data.price, InputError::LessPriceThanPreviousOffer);

        // transfer previous user amount back to user's account
        if data.offerer.is_some() {
            transfer(data.price, ~ContractId::from(FUEL), data.offerer.unwrap());
        }
        
        let nft_contract: b256 = id.into();

        let nft = OfferNft{
            offerer: Option::Some(msg_sender().unwrap()),
            price: price,
        };
        storage.offer_nft.insert((Option::Some(id), token_id), nft);
       
        log(NFTOfferEvent {
            offerer: msg_sender().unwrap(),
            nft_contract: id,
            token_id: token_id,
            price: price,
        });
    }

    #[storage(read, write)]
    fn buy_nft(id: ContractId, token_id: u64) {
        require(storage.nft_listed.get((Option::Some(id), token_id)), AccessError::NFTNotListed);

        let nft_contract: b256 = id.into();
        let this_contract = Identity::ContractId(contract_id());

        let nft_data = storage.list_nft.get((Option::Some(id), token_id));

        let protocol_amount = (nft_data.price * storage.protocol_fee) / 100;
        let user_amount = nft_data.price - protocol_amount;

        let nft_listed_data = storage.list_nft.get((Option::Some(id), token_id));
        let seller = nft_listed_data.owner;
        // require(seller != msg_sender().unwrap(), AccessError::BuyerSameAsSeller);
        require(msg_amount() == nft_listed_data.price, InputError::LessPriceThanPreviousOffer);

        // protocol fee
        // transfer(protocol_amount, ~ContractId::from(FUEL), storage.platform_fee_account);

        // user amount
        // transfer(user_amount , ~ContractId::from(FUEL), seller);

        // todo ContractNotInInputs error
        let x = abi(externalAbi, nft_contract);
        x.transfer_from(this_contract, msg_sender().unwrap(), token_id);
        storage.nft_listed.insert((Option::Some(id), token_id), false);

        // TODO: if we have `nft_listed` field in the contract we don't need to update/write in the contract
        // let nft = ListNft{
        //     owner: Option::None(),
        //     price: price,
        // };
        // storage.list_nft.insert((Option::Some(id), token_id), nft);
        log(NFTBoughtEvent {
            buyer: msg_sender().unwrap(),
            seller: nft_data.owner,
            nft_contract: id,
            token_id: token_id,
            price: nft_data.price,
        });
    }

    #[storage(read, write)]
    fn accept_offer(id: ContractId, token_id: u64, price: u64) {
        require(storage.nft_listed.get((Option::Some(id), token_id)), AccessError::NFTNotListed);
        
        let nft_contract: b256 = id.into();
        let data = storage.offer_nft.get((Option::Some(id), token_id));
        require(data.offerer.is_some(), InputError::OffererNotExists);
        require(data.offerer.unwrap() == msg_sender().unwrap(), AccessError::SenderDidNotMakeOffer);

        // close the offer
        let nft = OfferNft{
            offerer: Option::Some(msg_sender().unwrap()),
            price: 0,
        }; 
        
        let protocol_amount = (nft.price * storage.protocol_fee) / 100;
        let user_amount = nft.price - protocol_amount;
        let this_contract = Identity::ContractId(contract_id());

        storage.offer_nft.insert((Option::Some(id), token_id), nft);


        // protocol fee
        // transfer(protocol_amount, BASE_ASSET_ID, this_contract);

        // user amount
        // transfer(user_amount, BASE_ASSET_ID, msg_sender().unwrap());

        let x = abi(externalAbi, nft_contract);
        x.transfer_from(this_contract, msg_sender().unwrap(), token_id);
        storage.nft_listed.insert((Option::Some(id), token_id), false);

        log(NFTOfferAcceptEvent {
            offerer: msg_sender().unwrap(),
            owner: Identity::ContractId(id),
            nft_contract: id,
            token_id: token_id,
            price: price,
        });
    }

    #[storage(read, write)]
    fn change_nft_price(id: ContractId, token_id: u64, new_price: u64) {
        require(storage.nft_listed.get((Option::Some(id), token_id)), AccessError::NFTNotListed);
        let nft_data = storage.list_nft.get((Option::Some(id), token_id));
        require(nft_data.owner == msg_sender().unwrap(), AccessError::SenderNotOwner);

        let nft = ListNft {
            owner: msg_sender().unwrap(),
            price: new_price,
        };
        storage.list_nft.insert((Option::Some(id), token_id), nft);

        log(NFTPriceChangeEvent {
            owner: msg_sender().unwrap(),
            nft_contract: id,
            token_id: token_id,
            old_price: nft_data.price,
            new_price: new_price,
        });
    }

    #[storage(read)]
    fn nft_price(id: ContractId, token_id: u64) -> u64 {
        let nft_data = storage.list_nft.get((Option::Some(id), token_id));
        nft_data.price
    }
}
