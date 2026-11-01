use actix_web::{HttpResponse, Responder, post};

#[utoipa::path(
    post,
    responses(
        (status = 201, description = "Handle the creation of a new pool")
    ),
    tag = "Pools"
)]
#[post("/pool")]
async fn create_pool() -> impl Responder {
    // Handle the creation of a new pool
    HttpResponse::Created().finish()
}
