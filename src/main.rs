#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use reqwest::Error;

#[derive(Serialize, Deserialize)]
struct BazaarData {
    products: serde_json::Value,
}

#[get("/bazaar")]
async fn get_bazaar() -> Result<Json<BazaarData>, String> {
    let api_key = "f9cfea41-3f48-4fde-b41c-0508ef3eb5ab";
    let url = format!("https://api.hypixel.net/skyblock/bazaar?key={}", api_key);

    let response = reqwest::get(&url).await.map_err(|err| err.to_string())?;
    let bazaar_data: BazaarData = response.json().await.map_err(|err| err.to_string())?;

    Ok(Json(bazaar_data))

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_bazaar])
}