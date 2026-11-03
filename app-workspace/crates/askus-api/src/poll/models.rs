use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct PollRequest {
    #[validate(length(min = 1))]
    pub subject: String,

    #[validate(length(min = 4))]
    pub participants: Vec<String>,

    #[validate(range(min = 1, max = 5))]
    pub intensity: u8,

    pub poll_type: PollType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum PollType {
    Person,
    Who,
    Random,
}

#[derive(Debug, Deserialize, Validate, Serialize, utoipa::ToSchema)]
pub struct Poll {
    #[serde(rename = "type")]
    pub poll_type: PollType,

    pub target: Option<String>,

    #[validate(length(min = 1))]
    pub question: String,

    #[validate(length(min = 4))]
    pub answers: Vec<String>,
}
