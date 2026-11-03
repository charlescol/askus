use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct PollRequest {
    /// The subject of the poll, eg "Friends night"
    #[validate(length(min = 1))]
    #[schema(min_length = 1, example = "Friends night")]
    pub subject: String,

    /// The participants name of the poll, eg ["Alice", "Bob", "Charlie", "Dave"]
    #[validate(length(min = 4))]
    #[schema(min_items = 4, example = json!(["Alice", "Bob", "Charlie", "Dave"]))]
    pub participants: Vec<String>,

    /// The intensity for the llm model, eg 3   
    #[validate(range(min = 1, max = 5))]
    #[schema(minimum = 1, maximum = 5, example = 3)]
    pub intensity: u8,

    /// The type of the poll, eg "Person"
    #[schema(example = "Person")]
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
    /// Type of poll generated.
    #[serde(rename = "type")]
    pub poll_type: PollType,

    /// Targeted participant for PERSON polls. Null for WHO polls.
    #[schema(example = "Alice", nullable)]
    pub target: Option<String>,

    /// Question generated for the poll.
    #[validate(length(min = 1))]
    #[schema(min_length = 1, example = "Why should Alice never organize a party?")]
    pub question: String,

    /// Exactly 4 possible answers.
    #[validate(length(equal = 4))]
    #[schema(min_items = 4, max_items = 4, example = json!(["Alice is too loud", "Alice is too messy", "Alice is too boring", "Alice is too expensive"]))]
    pub answers: Vec<String>,
}
