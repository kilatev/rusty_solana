use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::collections::HashMap;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/getSlot")]
async fn getSlot() -> impl Responder {
    let resp = request_slot();
    let re = resp.await;
    match re {
        Ok(v) => HttpResponse::Ok().body(format!("{} {}", "Hey there!", v)),
        _ => HttpResponse::Ok().body(format!("{} {:#?}", "Hey there!", re)),
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
    let resp = request_slot().await;
    match resp {
        Ok(z) => HttpResponse::Ok().body(format!("{} {}", "Origin!", z)),
        _ => HttpResponse::Ok().body(format!("{} {:#?}", "Error ws there!", resp)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5000/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(getSlot)
            .route("/hey", web::get().to(manual_hello))
            .route("/getSlot2", web::get().to(get_slot))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
