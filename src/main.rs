mod structs;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use reqwest::Error;
use structs::{AuctionData, Auction};

#[get("/")]
async fn get_auctions() -> Result<Json<AuctionData>, String> {
    let api_key = "f9cfea41-3f48-4fde-b41c-0508ef3eb5ab"; // Your API key
    let url = format!("https://api.hypixel.net/v2/skyblock/auctions?key={}", api_key);

    // Fetch the data from the Hypixel API
    let response = reqwest::get(&url).await.map_err(|err| err.to_string())?;
    //println!("Headers:\n{:#?}", response.headers().get("Age").unwrap());
    // Parse the JSON response
    let mut auction_data: AuctionData = response.json().await.map_err(|err| err.to_string())?;

    Ok(Json(auction_data))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_auctions])
}
