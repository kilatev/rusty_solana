use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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

async fn manual_hello() -> impl Responder {
    let resp = opa();
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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
