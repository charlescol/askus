use init_config::ConfigLoader;
use serde::{Deserialize, Serialize};

use serde_json::{Value, json};
use validator::Validate;

use std::sync::LazyLock;

static CONFIG: LazyLock<Result<Config, String>> =
    LazyLock::new(|| Config::load().map_err(|err| err.to_string()));

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PollType {
    Person,
    Who,
    Random,
}

#[derive(Debug, Serialize, Validate)]
pub struct PollRequest {
    #[validate(length(min = 1))]
    pub subject: String,

    #[validate(length(min = 4))]
    pub participants: Vec<String>,

    #[validate(range(min = 1, max = 5))]
    pub intensity: u8,

    pub poll_type: PollType,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Poll {
    #[serde(rename = "type")]
    pub poll_type: PollType,

    pub target: Option<String>,

    #[validate(length(min = 1))]
    pub question: String,

    #[validate(length(min = 4))]
    pub answers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    system_prompt: String,
    api_key: String,
    openai_api_url: String,
}

impl Config {
    /// Load the configuration from environment variables.
    fn load() -> Result<Self, anyhow::Error> {
        #[cfg(debug_assertions)]
        dotenv::dotenv().ok();

        let path = ConfigLoader::load_or_fail("ASKUS_PROMPT_PATH")?;
        Ok(Config {
            system_prompt: std::fs::read_to_string(path)?,
            api_key: ConfigLoader::load_or_fail("OPENAI_API_KEY")?,
            openai_api_url: ConfigLoader::load_or_fail("OPENAI_API_URL")?,
        })
    }

    // Get the configuration. Lazy load it if it hasn't been loaded yet.
    pub(crate) fn get() -> Result<&'static Self, anyhow::Error> {
        CONFIG.as_ref().map_err(|err| anyhow::anyhow!("{err}"))
    }
}

impl Poll {
    // Create a new Poll by sending a request to the OpenAI API.
    /// # Parameters
    /// - `request`: the PollRequest to send to the OpenAI API.
    pub async fn new(request: PollRequest) -> Result<Poll, anyhow::Error> {
        let config = Config::get()?;
        request.validate()?;

        let response = HTTP_CLIENT
            .post(&config.openai_api_url)
            .bearer_auth(&config.api_key)
            .json(&json!({
                "model": "gpt-6-luna",
                "instructions": config.system_prompt,
                "input": [
                    {
                        "role": "user",
                        "content": serde_json::to_string(&request)?
                    }
                ],
                "text": {
                    "format": {
                        "type": "json_object"
                    }
                },
                "max_output_tokens": 500
            }))
            .send()
            .await?
            .error_for_status()?;

        let body: Value = response.json().await?;

        let content = body["output"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing model output"))?
            .iter()
            .flat_map(|item| item["content"].as_array().into_iter().flatten())
            .find_map(|item| {
                if item["type"] == "output_text" {
                    item["text"].as_str()
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Missing text response: {body}"))?;

        let poll: Poll = serde_json::from_str(content)?;

        poll.validate()?;

        Ok(poll)
    }
}
