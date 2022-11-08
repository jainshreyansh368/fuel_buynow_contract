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
    NFTChangeOfferEvent,
};
use interface::NftMarketplace;
use external_interface::externalAbi;

use std::{
    chain::auth::msg_sender,
    context::call_frames::contract_id,
    logging::log,
    storage::StorageMap,
    token::transfer,
};

storage {
    // Stores the user that is permitted to be handle the admin Operations of the contract.
    // Only the `admin` is allowed to change the `admin` of the contract.
    admin: Option<Identity> = Option::Some(Identity::Address(~Address::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e))),
    // // Total Number of NFTS Listed on Platform
    // no_of_nft_listed: u64 = 0,
    // //No of NFts listed on platform by a single user
    // // Map(user(Identity) => no_of_nft_listed)
    // no_of_nft_listed_by_user: StorageMap<Option<Identity>, u64> = StorageMap{},
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

        // log(AdminChangedEvent {
        //     mew_admin: admin.unwrap(),
        // });
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

        // log(NFTListedEvent {
        //     owner: msg_sender().unwrap(),
        //     nft_contract: id,
        //     token_id: token_id,
        //     price: price,
        // });
    }
}
