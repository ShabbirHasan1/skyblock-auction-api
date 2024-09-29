use hyper::{Client, Request};
use hyper::header::HeaderValue;
use hyper_tls::HttpsConnector;
use std::time::Duration;
use tokio::time::sleep;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = "https://api.hypixel.net/v2/skyblock/auctions?key=e08cfdcf-6502-4fdd-ae14-6abf38fb18c6";

    // Create an HTTPS connector and a Hyper client (default configuration)
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    loop {
        // Record the time before the request is sent
        let start_time = Instant::now();

        // Create and send the GET request
        let req = Request::builder()
            .uri(url)
            .body(hyper::Body::empty())?;
        let res = client.request(req).await?;

        // Stop the timer after the request is completed
        let elapsed_time = start_time.elapsed();
        println!("Response status: {}", res.status());
        println!("Time taken for the request: {:?}", elapsed_time);

        // Check if the "Age" header is present
        if let Some(age_header) = res.headers().get("Age") {
            // Parse the Age header
            let age: u64 = age_header.to_str()?.parse()?;
            println!("Age header value: {}", age);

            if age >= 60 {
                println!("Page update detected, fetching new data...");
            } else {
                // Wait until Age reaches 60 before the next request
                let wait_time = 60 - age;
                println!("Waiting for {} seconds before the next check...", wait_time);
                sleep(Duration::from_secs(wait_time)).await;
            }
        } else {
            println!("No Age header found in the response. Retrying in 60 seconds...");
            // If the Age header isn't found, wait for a default 60 seconds
            sleep(Duration::from_secs(60)).await;
        }
    }
}
