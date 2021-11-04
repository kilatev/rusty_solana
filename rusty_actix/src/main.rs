use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/getSlot")]
async fn getSlot() -> impl Responder {
    let resp = request_slot();
    if resp.is_ok() {
        HttpResponse::Ok().body(format!("{} {:#?}", "Hey there!", resp))
    } else {
        print!("{:#?}", resp);
        HttpResponse::Ok().body(format!("{} {}", "Hey there!", "aa"))
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn opa() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(resp)
}
fn request_slot() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    println!("AHAHAHHAHAHHAH");
    let resp = client
        .post("https://api.mainnet-beta.solana.com")
        .body("{\"jsonrpc\":\"2.0\",\"id\":1, \"method\":\"getSlot\"}")
        .send()?
        .json::<HashMap<String, String>>()?;
    let res = format!("{:#?}", resp);
    Ok(res)
}

async fn manual_hello() -> impl Responder {
    let resp = opa();
    match resp {
        Ok(z) => {
            let mut res = "";
            match z.get("origin") {
                Some(v) => res = v,
                _ => res = "",
            }
            HttpResponse::Ok().body(format!("{} {}", "Origin!", res))
        }
        _ => HttpResponse::Ok().body(format!("{} {:#?}", "Error ws there!", resp)),
    }
}
async fn get_slot() -> impl Responder {
    let resp = request_slot();
    if resp.is_ok() {
        HttpResponse::Ok().body(format!("{} {:#?}", "Hey there!", resp))
    } else {
        HttpResponse::Ok().body(format!("{} {}", "Hey there!", "aa"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(getSlot)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
