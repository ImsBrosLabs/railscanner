use actix_web::{get, web, HttpResponse};
use config::Config;
use http::header::AUTHORIZATION;
use log::info;
use once_cell::sync::Lazy;
use request::Request;
use std::collections::HashMap;

mod request;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::builder()
        .add_source(config::File::with_name("src/Settings.toml"))
        .build()
        .unwrap()
});

#[get("/navitia")]
async fn navitia(request: web::Json<Request>) -> HttpResponse {
    info!("received request : {}", request);

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
        .get("https://api.navitia.io/v1/coverage/fr-idf/journeys?from=stop_area%3AIDFM%3A68105&max_duration=3600&count=4&")
        .insert_header((AUTHORIZATION, api_key));

    info!("starting request to Navitia");

    // send request and await response
    let mut res = req.send().await.unwrap();
    info!(
        "got response from Navitia, http status : {:?}",
        res.status()
    );

    let payload = res.body().await.unwrap();

    HttpResponse::Ok().body(payload)
}
