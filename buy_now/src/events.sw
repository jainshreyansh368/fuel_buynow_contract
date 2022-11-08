library events;

pub struct AdminChangedEvent {
    // The user which is now the admin of this contract.
    // If there is no longer an admin then the `Option` will be `None`.
    mew_admin: Identity,
}

pub struct NFTListedEvent {
    owner: Identity,
    nft_contract: ContractId,
    token_id: u64,
    price: u64,
}

pub struct NFTDeListedEvent {
    owner: Identity,
    nft_contract: ContractId,
    token_id: u64,
}

pub struct NFTPriceChangeEvent {
    owner: Identity,
    nft_contract: ContractId,
    token_id: u64,
    old_price: u64,
    new_price: u64,
}

pub struct NFTBoughtEvent {
    buyer: Identity,
    seller: Identity,
    nft_contract: ContractId,
    token_id: u64,
    price: u64,
}

pub struct NFTOfferEvent {
    offerer: Identity,
    nft_contract: ContractId,
    token_id: u64,
    price: u64,
}

pub struct NFTOfferAcceptEvent {
    offerer: Identity,
    owner: Identity,
    nft_contract: ContractId,
    token_id: u64,
    price: u64,
}

pub struct NFTChangeOfferEvent {
    offerer: Identity,
    nft_contract: ContractId,
    token_id: u64,
    new_price: u64,
    old_price: u64,
}
