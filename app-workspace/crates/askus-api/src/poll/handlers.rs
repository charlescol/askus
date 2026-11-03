use crate::{
    common::error::ApiError,
    poll::{
        error::PollError,
        models::{Poll, PollRequest},
        service::PollService,
    },
};
use actix_web::{HttpResponse, post, web};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/poll",
    request_body = PollRequest,
    responses(
        (status = 201, description = "Poll created", body = Poll),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Polls"
)]
#[post("/poll")]
async fn create_poll(
    service: web::Data<PollService>,
    request: web::Json<PollRequest>,
) -> Result<HttpResponse, ApiError> {
    request
        .validate()
        .map_err(|e| PollError::InvalidPollRequest(e.to_string()))?;

    let poll = service.generate_poll(request.into_inner()).await?;

    Ok(HttpResponse::Created().json(poll))
}
