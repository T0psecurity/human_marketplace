use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sg_controllers::Hooks;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SudoParams {
    pub trading_fee_percent: u32,
    pub ask_expiry: (u64, u64),
    pub bid_expiry: (u64, u64),
    pub operators: Vec<Addr>,
}

pub const SUDO_PARAMS: Item<SudoParams> = Item::new("sudo_params");

pub type TokenId = u32;

/// Represents an ask on the marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ask {
    pub collection: Addr,
    pub token_id: TokenId,
    pub seller: Addr,
    pub price: Uint128,
    pub funds_recipient: Option<Addr>,
    pub expires: Timestamp,
    pub active: bool,
}

/// Primary key for asks: (collection, token_id)
pub type AskKey = (Addr, TokenId);
/// Convenience ask key constructor
pub fn ask_key(collection: Addr, token_id: TokenId) -> AskKey {
    (collection, token_id)
}

/// Defines indices for accessing Asks
pub struct AskIndicies<'a> {
    pub collection: MultiIndex<'a, Addr, Ask, AskKey>,
    pub collection_price: MultiIndex<'a, (Addr, u128), Ask, AskKey>,
    pub seller: MultiIndex<'a, Addr, Ask, AskKey>,
}

impl<'a> IndexList<Ask> for AskIndicies<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Ask>> + '_> {
        let v: Vec<&dyn Index<Ask>> = vec![&self.collection, &self.collection_price, &self.seller];
        Box::new(v.into_iter())
    }
}

pub fn asks<'a>() -> IndexedMap<'a, AskKey, Ask, AskIndicies<'a>> {
    let indexes = AskIndicies {
        collection: MultiIndex::new(|d: &Ask| d.collection.clone(), "asks", "asks__collection"),
        collection_price: MultiIndex::new(
            |d: &Ask| (d.collection.clone(), d.price.u128()),
            "asks",
            "asks__collection_price",
        ),
        seller: MultiIndex::new(|d: &Ask| d.seller.clone(), "asks", "asks__seller"),
    };
    IndexedMap::new("asks", indexes)
}

/// Represents a bid (offer) on the marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bid {
    pub collection: Addr,
    pub token_id: TokenId,
    pub bidder: Addr,
    pub price: Uint128,
    pub expires: Timestamp,
}

/// Primary key for bids: (collection, token_id, bidder)
pub type BidKey = (Addr, TokenId, Addr);
/// Convenience bid key constructor
pub fn bid_key(collection: Addr, token_id: TokenId, bidder: Addr) -> BidKey {
    (collection, token_id, bidder)
}

/// Defines incides for accessing bids
pub struct BidIndicies<'a> {
    pub collection: MultiIndex<'a, Addr, Bid, BidKey>,
    pub collection_token_id: MultiIndex<'a, (Addr, TokenId), Bid, BidKey>,
    pub collection_price: MultiIndex<'a, (Addr, u128), Bid, BidKey>,
    pub bidder: MultiIndex<'a, Addr, Bid, BidKey>,
}

impl<'a> IndexList<Bid> for BidIndicies<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Bid>> + '_> {
        let v: Vec<&dyn Index<Bid>> = vec![
            &self.collection,
            &self.collection_token_id,
            &self.collection_price,
            &self.bidder,
        ];
        Box::new(v.into_iter())
    }
}

pub fn bids<'a>() -> IndexedMap<'a, BidKey, Bid, BidIndicies<'a>> {
    let indexes = BidIndicies {
        collection: MultiIndex::new(|d: &Bid| d.collection.clone(), "bids", "bids__collection"),
        collection_token_id: MultiIndex::new(
            |d: &Bid| (d.collection.clone(), d.token_id),
            "bids",
            "bids__collection_token_id",
        ),
        collection_price: MultiIndex::new(
            |d: &Bid| (d.collection.clone(), d.price.u128()),
            "bids",
            "bids__collection_price",
        ),
        bidder: MultiIndex::new(|d: &Bid| d.bidder.clone(), "bids", "bids__bidder"),
    };
    IndexedMap::new("bids", indexes)
}

/// Represents a bid (offer) across an entire collection in the marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionBid {
    pub collection: Addr,
    pub bidder: Addr,
    pub price: Uint128,
    pub expires: Timestamp,
}

/// Primary key for bids: (collection, token_id, bidder)
pub type CollectionBidKey = (Addr, Addr);
/// Convenience collection bid key constructor
pub fn collection_bid_key(collection: Addr, bidder: Addr) -> CollectionBidKey {
    (collection, bidder)
}

/// Defines incides for accessing collection bids
pub struct CollectionBidIndicies<'a> {
    pub collection: MultiIndex<'a, Addr, CollectionBid, CollectionBidKey>,
    pub collection_price: MultiIndex<'a, (Addr, u128), CollectionBid, CollectionBidKey>,
    pub bidder: MultiIndex<'a, Addr, CollectionBid, CollectionBidKey>,
}

impl<'a> IndexList<CollectionBid> for CollectionBidIndicies<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<CollectionBid>> + '_> {
        let v: Vec<&dyn Index<CollectionBid>> =
            vec![&self.collection, &self.collection_price, &self.bidder];
        Box::new(v.into_iter())
    }
}

pub fn collection_bids<'a>(
) -> IndexedMap<'a, CollectionBidKey, CollectionBid, CollectionBidIndicies<'a>> {
    let indexes = CollectionBidIndicies {
        collection: MultiIndex::new(
            |d: &CollectionBid| d.collection.clone(),
            "col_bids",
            "col_bids__collection",
        ),
        collection_price: MultiIndex::new(
            |d: &CollectionBid| (d.collection.clone(), d.price.u128()),
            "col_bids",
            "col_bids__collection_price",
        ),
        bidder: MultiIndex::new(
            |d: &CollectionBid| d.bidder.clone(),
            "col_bids",
            "col_bids__bidder",
        ),
    };
    IndexedMap::new("col_bids", indexes)
}

pub const HOOKS: Hooks = Hooks::new("hooks");
