use actix_web::{App, HttpServer};
use log::info;

mod endpoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up Actix webserver");

    HttpServer::new(|| App::new().service(endpoint::navitia))
        .bind(("127.0.0.1", 8080))?
        .workers(1)
        .run()
        .await
}
