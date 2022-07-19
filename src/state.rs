use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum State {
    IsActive(),
    Info(),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StateReply {
    IsActive(bool),
    Info(AuctionInfo),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct AuctionInfo {
    pub nft_contract_actor_id: ActorId,
    pub token_id: U256,
    pub token_owner: ActorId,
    pub starting_price: u128,
    pub current_price: u128,
    pub discount_rate: u128,
    pub time_left: u64,
}
