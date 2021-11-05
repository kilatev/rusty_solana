use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct SlotResponse {
    jsonrpc: String,
    result: i32,
    id: String,
}

async fn request_slot() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut req_body = HashMap::new();
    req_body.insert("jsonrpc", "2.0");
    req_body.insert("id", "1");
    req_body.insert("method", "getSlot");

    let resp = client
        .post("https://api.mainnet-beta.solana.com")
        .json(&req_body)
        .send()
        .await?;
    let r_json = resp.json::<SlotResponse>().await?;
    print!("{:?}", r_json);
    Ok(String::from("5454"))
}

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    let r = request_slot().await;
    // match r{
    //     Ok(_) -> "PiPiu",
    //     Err(_) -> "Do Nothing"
    // }
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
