use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use config::Config;
use http::header::AUTHORIZATION;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::builder()
        .add_source(config::File::with_name("src/Settings.toml"))
        .build()
        .unwrap()
});

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/navitia")]
async fn navitia() -> HttpResponse {
    let api_key = CONFIG
        .to_owned()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
        .get("api_key")
        .unwrap()
        .clone();

    // create client
    let client = awc::Client::default();

    // construct request
    let req = client
        .get("https://api.navitia.io/v1/coverage/fr-idf/journeys?from=stop_area%3AIDFM%3A68105&to=stop_area%3AIDFM%3A71673&")
        .insert_header((AUTHORIZATION, api_key));

    log::info!("starting HTTP server to navitia");

    // send request and await response
    let mut res = req.send().await.unwrap();
    println!("Response: {:?}", res);

    let payload = res.body().await.unwrap();

    HttpResponse::Ok().body(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(navitia))
        .bind(("127.0.0.1", 8080))?
        .workers(1)
        .run()
        .await
}
