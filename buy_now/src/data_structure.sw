library data_structure;

pub struct ListNft {
    owner: Identity,
    price: u64,
} 

pub struct OfferNft {
    offerer: Option<Identity>,
    price: u64,
}

pub struct UserStruct {
    user: Identity,
    no_of_nft_listed: u64,
}

pub struct NFTListed {
    owner: Identity,
    nft_contract: ContractId,
    token_id: u64,
    price: u64,
}

pub struct NFTDeListed {
    owner: Identity,
    nft_contract: ContractId,
    token_id: u64,
}

pub struct ChangeListPrice {
    owner: Identity,
    nft_contract: ContractId,
    token_id: u64,
    old_price: u64,
    new_price: u64,
}

pub struct NFTBought {
    buyer: Identity,
    seller: Identity,
    nft_contract: ContractId,
    token_id: u64,
    price: u64,
}
