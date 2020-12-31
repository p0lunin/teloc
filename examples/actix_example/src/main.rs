mod services;

use crate::services::{ActixService, Repository};
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use teloc::{DIActixHandler, ServiceProvider};

async fn index(service: Arc<ActixService>, data: web::Json<String>) -> String {
    service.change_and_get_previous(data.0).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the `ServiceProvider` struct that store itself all dependencies.
    let sp = ServiceProvider::new()
        // Add dependency with a `Singleton` lifetime. More about lifetimes see in README.md.
        .add_singleton::<Arc<Repository>>()
        // Add dependency with a `Transient` lifetime. More about lifetimes see in README.md.
        .add_transient::<Arc<ActixService>>();
    // We need to wrap Arc around `ServiceProvider` for thread-safety and cloning.
    let sp = Arc::new(sp);

    HttpServer::new(move || {
        // `DIActixHandler` gives as input a `ServiceProvider` and a handler function and inject
        // dependencies from the start args in function.
        App::new().route("/", web::get().to(DIActixHandler::new(sp.clone(), std::convert::identity, index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
