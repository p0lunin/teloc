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
    let sp = ServiceProvider::new()
        .add_singleton::<Arc<Repository>>()
        .add_transient::<Arc<ActixService>>();
    let sp = Arc::new(sp);

    HttpServer::new(move || {
        App::new().route("/", web::get().to(DIActixHandler::new(sp.clone(), index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
