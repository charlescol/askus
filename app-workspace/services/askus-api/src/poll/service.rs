use validator::Validate;

use crate::{
    common::config::LLMConfig,
    poll::{
        error::PollError,
        models::{Poll, PollRequest},
    },
};

const LLM_INSTRUCTIONS: &str = include_str!("../../prompts/askus.md");

pub struct PollService {
    llm_client: llm_client::LlmClient,
}

impl PollService {
    /// Initialize the service with the given LLM configuration. The service will read the LLM instructions from the file `./prompts/askus.md`.
    /// # Parameters
    /// - `llm_config`: the LLM configuration to use for the service
    pub fn new(llm_config: LLMConfig) -> Self {
        let llm_client = llm_client::LlmClient::builder()
            .api_key(llm_config.api_key)
            .maybe_api_url(llm_config.api_url)
            .maybe_model(llm_config.model)
            .build();

        Self { llm_client }
    }

    /// Generate a poll using the LLM API.
    /// This service assumes the `PollRequest::validate` method has been called before calling this method.
    /// The client is responsible to provide an llm instructions that produces a valid Poll object as response.
    /// # Parameters
    /// - `request`: the poll request to send to the LLM API
    pub(super) async fn generate_poll(&self, request: PollRequest) -> Result<Poll, PollError> {
        let input = &serde_json::to_string(&request)
            .map_err(|e| PollError::InvalidPollRequest(e.to_string()))?;

        let poll = self
            .llm_client
            .generate::<Poll>(&LLM_INSTRUCTIONS, input)
            .await
            .map_err(|e| PollError::LLMServerError(e))?;

        poll.validate()
            .map_err(|e| PollError::InvalidPollResponse(e.to_string()))?;

        Ok(poll)
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::config::Config, poll::models::PollType};

    use super::*;

    #[actix_web::test]
    #[ignore = "Requires OpenAI API access"]
    async fn test_generate_pool_person() {
        let config = Config::new().expect("Failed to load configuration");
        let service = PollService::new(config.llm);

        let request = PollRequest {
            subject: "Test Poll".to_string(),
            participants: vec![
                "Alice".to_string(),
                "Bob".to_string(),
                "Charlie".to_string(),
                "Dave".to_string(),
            ],
            intensity: 3,
            poll_type: PollType::Person,
        };

        let poll = service
            .generate_poll(request)
            .await
            .expect("Failed to generate PERSON poll");

        println!("{poll:#?}");

        assert_eq!(poll.poll_type, PollType::Person);
        assert!(poll.target.is_some());
        assert!(!poll.question.trim().is_empty());
        assert_eq!(poll.answers.len(), 4);
        assert!(poll.answers.iter().all(|answer| !answer.trim().is_empty()));
    }

    #[actix_web::test]
    #[ignore = "Requires OpenAI API access"]
    async fn test_generate_pool_who() {
        let config = Config::new().expect("Failed to load configuration");
        let service = PollService::new(config.llm);

        let participants = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
            "Dave".to_string(),
        ];

        let request = PollRequest {
            subject: "Test Poll".to_string(),
            participants: participants.clone(),
            intensity: 3,
            poll_type: PollType::Who,
        };

        let poll = service
            .generate_poll(request)
            .await
            .expect("Failed to generate WHO poll");

        println!("{poll:#?}");

        assert_eq!(poll.poll_type, PollType::Who);
        assert!(poll.target.is_none());
        assert!(!poll.question.trim().is_empty());
        assert_eq!(poll.answers.len(), 4);

        assert!(
            poll.answers
                .iter()
                .all(|answer| participants.contains(answer))
        );

        let unique_answers: std::collections::HashSet<_> = poll.answers.iter().collect();

        assert_eq!(unique_answers.len(), 4);
    }
}
