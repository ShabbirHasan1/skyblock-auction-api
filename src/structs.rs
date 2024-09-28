use rocket::serde::{Deserialize, Serialize};  // For Rocket compatibility
use serde::{Deserialize as SerdeDeserialize};  // Using `serde::Deserialize`
use base64::decode;
use flate2::read::GzDecoder;
use std::io::prelude::*;
use serde_json::Value;


#[derive(Serialize, Deserialize)]
pub struct AuctionData {
    pub page: u32,
    pub totalPages: u32,
    pub auctions: Vec<Auction>,  // Make this field public
}

#[derive(Serialize, Deserialize)]
pub struct Auction {
    pub category: String,
    pub starting_bid: u64,
    pub tier: String,
    pub bin: bool,
    pub item_bytes: String,
}

