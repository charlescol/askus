use crate::{
    common::error::ApiError,
    poll::{error::PollError, models::PollRequest, service::PollService},
};
use actix_web::{HttpResponse, post, web};
use validator::Validate;

#[utoipa::path(
    post,
    responses(
        (status = 201, description = "Handle the creation of a new poll")
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
