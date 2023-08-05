use actix_web::{post, web, HttpResponse};
use config::Config;
use http::header::AUTHORIZATION;
use once_cell::sync::Lazy;
use request::*;
use response::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::navitia::response::StopAreasResponse;

pub mod request;
pub mod response;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::builder()
        .add_source(config::File::with_name("src/Settings.toml"))
        .build()
        .unwrap()
});

#[post("/stop_areas")]
async fn stop_areas(request: web::Json<StopAreaRequest>) -> HttpResponse {
    log::info!("received /stop_areas request : {:?}", &request);

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
    let query_params = [("q", &request.name)];

    let req = client
        .get("https://api.navitia.io/v1/coverage/fr-idf/places")
        .timeout(Duration::from_secs(5))
        .insert_header((AUTHORIZATION, api_key))
        .query(&query_params)
        .unwrap();

    log::info!("{}", req.get_uri());
    log::info!("starting /stop_areas request to Navitia");

    // send request and await response
    let mut res = req.send().await.unwrap();
    log::info!(
        "got /stop_areas response from Navitia, http status : {:?}",
        res.status()
    );

    // extract body
    let json_payload = res.body().await.unwrap();

    let stop_areas_response: StopAreasResponse = serde_json::from_slice(&json_payload).unwrap();

    // extract stop_areas only
    let mut result: Vec<PlaceLight> = Vec::new();

    for place in stop_areas_response.places {
        if place.embedded_type.eq("stop_area") {
            let place_light = PlaceLight {
                id: place.id,
                name: place.name,
            };
            result.push(place_light);
        }
    }

    HttpResponse::Ok().json(result)
}

#[post("/journey")]
async fn journey(request: web::Json<JourneyRequest>) -> HttpResponse {
    log::info!("received /journey request : {:?}", &request);

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
    let query_params = (
        ("from", &request.from),
        ("max_duration", &request.max_duration),
        ("count", &request.count),
    );

    let req = client
        .get("https://api.navitia.io/v1/coverage/fr-idf/journeys")
        .timeout(Duration::from_secs(5))
        .insert_header((AUTHORIZATION, api_key))
        .query(&query_params)
        .unwrap();

    log::info!("{}", req.get_uri());
    log::info!("starting /journey request to Navitia");

    // send request and await response
    let mut res = req.send().await.unwrap();
    log::info!(
        "got / journey response from Navitia, http status : {:?}",
        res.status()
    );

    // extract body
    let payload = res.body().await.unwrap();

    HttpResponse::Ok().body(payload)
}
