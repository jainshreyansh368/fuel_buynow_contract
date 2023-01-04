library user_bid_return_data;

dep token_asset;
dep nft_asset;
dep auction;

use token_asset::TokenAsset;
use nft_asset::NFTAsset;
use auction::Auction;

pub struct UserBidReturnData {
    bid_amount: u64,
    auction_state: Auction,
}