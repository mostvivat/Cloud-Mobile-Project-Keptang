use actix_web::{middleware, App, HttpServer, get, Responder, HttpResponse};
use env_logger::Env;
use actix_cors::Cors;
mod handlers;
mod models;
mod config;
pub mod routes;

use crate::routes::{
    money_routes,
    addmoney_routes,
    deletemoney_routes,
    editmoney_routes,
    profilemoney_routes,
    todaymoney_routes,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Welcome to web-Keptang!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    
    

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(index)
            .configure(profilemoney_routes::config)
            .configure(money_routes::config)
            .configure(todaymoney_routes::config)
            .configure(addmoney_routes::config)
            .configure(editmoney_routes::config)
            .configure(deletemoney_routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


