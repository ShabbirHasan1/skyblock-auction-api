use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Auction {
    uuid: String,
    item_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    success: bool,
    auctions: Vec<Auction>,
}