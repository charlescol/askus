use actix_web::{App, HttpServer};
use app::pool::handlers::create_pool;
use serde::Deserialize;
use utoipa::openapi::{Info, OpenApi, Server};
use utoipa_actix_web::{AppExt, scope};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone, Deserialize)]
struct ApiMetadata {
    info: Info,
    servers: Vec<Server>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let metadata: ApiMetadata =
        serde_json::from_str(include_str!("../openapi.json")).expect("Invalid OpenAPI metadata");

    HttpServer::new(move || {
        let mut openapi = OpenApi::default();
        openapi.info = metadata.info.clone();
        openapi.servers = Some(metadata.servers.clone());

        App::new()
            .into_utoipa_app()
            .openapi(openapi)
            .service(scope::scope("/api/v1").service(create_pool))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
