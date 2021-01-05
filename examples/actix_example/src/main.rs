mod services;

use crate::services::{ActixService, Repository};
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use teloc::{DIActixHandler, ServiceProvider};

async fn index(service: ActixService<'_>, data: String) -> String {
    service.change_and_get_previous(data).await
}

// For tests you can use curl:
// ```
// curl --header "Content-Type: application/json" --data '{"some":"json"}' "127.0.0.1:8080"
// ```
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the `ServiceProvider` struct that store itself all dependencies.
    let sp = ServiceProvider::new()
        // Add dependency with a `Singleton` lifetime. More about lifetimes see in README.md.
        .add_singleton::<Arc<Repository>>()
        // Add dependency with a `Transient` lifetime. More about lifetimes see in README.md.
        .add_transient::<ActixService>();
    // We need to wrap Arc around `ServiceProvider` for thread-safety and cloning.
    let sp = Arc::new(sp);

    HttpServer::new(move || {
        App::new().route(
            "/",
            web::post().to(
                // `DIActixHandler` gives as input a `ServiceProvider` and a handler function and inject
                // dependencies from the start args in function.
                DIActixHandler::new(
                    // Global `ServiceProvider`.
                    sp.clone(),
                    // Scope factory that can add scope instances that will be the same between different
                    // dependencies in one scope.
                    |s| s,
                    // Function that will be called for each `HttpRequest`.
                    index
                )
            ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
