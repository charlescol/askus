use actix_web::{App, HttpServer, web};
use app::{common::config::Config, poll::handlers::create_poll};
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

    let config = Config::new().expect("Failed to load configuration");

    let poll_service = web::Data::new(app::poll::service::PollService::new(
        app::common::config::LLMConfig {
            api_key: config.llm.api_key,
            api_url: config.llm.api_url,
            model: config.llm.model,
        },
    ));

    HttpServer::new(move || {
        let mut openapi = OpenApi::default();
        openapi.info = metadata.info.clone();
        openapi.servers = Some(metadata.servers.clone());

        App::new()
            .into_utoipa_app()
            .openapi(openapi)
            .app_data(poll_service.clone())
            .service(scope::scope("/api/v1").service(create_poll))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
