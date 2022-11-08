library interface;

dep data_structure;

use data_structure::{
    ListNft,
    OfferNft,
};

//list nft 
//delist nft 
//buy nft 
//change_list_price
abi NftMarketplace {
    // Returns the current admin for the contract.
    // 
    // # Reverts
    // 
    // * When the contract does not have an admin.
    #[storage(read)]
    fn admin() -> Identity;

    // Changes the contract's admin.
    // 
    // This new admin will have access to minting if `access_control` is set to true and be able
    // to change the contract's admin to a new admin.
    // 
    // # Arguments
    // 
    // * `admin` - The user which is to be set as the new admin.
    // 
    // # Reverts
    // 
    // * When the sender is not the `admin` in storage.
    #[storage(read, write)]
    fn set_admin(admin: Identity);

    #[storage(read, write)]
    fn list_nft(id: ContractId, token_id: u64, price: u64);
}
