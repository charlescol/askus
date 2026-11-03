use bon::Builder;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

pub enum Company {
    OpenAI,
}
#[derive(Builder)]
pub struct LlmClient {
    #[builder(skip = reqwest::Client::new())]
    http_client: reqwest::Client,

    api_key: String,

    #[builder(default = "https://api.openai.com/v1/responses".to_string())]
    api_url: String,

    #[builder(default = "gpt-6-luna".to_string())]
    model: String,
}

impl LlmClient {
    /// Generate a response from the LLM API.
    /// # Parameters
    /// - `instructions`: the instructions to send to the LLM API, used to guide the model's behavior.
    /// - `input`: the user input to send to the LLM API
    pub async fn generate<T>(&self, instructions: &str, input: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http_client
            .post(&self.api_url)
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": &self.model,
                "instructions": instructions,
                "input": [
                    {
                        "role": "user",
                        "content": format!(
                            "Return a valid JSON object.\n{input}"
                        )
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
            .await?;

        let status = response.status();

        if !status.is_success() {
            anyhow::bail!("LLM API error ({}): {}", status, response.text().await?);
        }

        let body: Value = response.json().await?;

        if body["status"] != "completed" {
            anyhow::bail!("LLM response not completed: {body}");
        }

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

        Ok(serde_json::from_str(content)?)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TestResponse {
        message: String,
    }

    #[tokio::test]
    #[ignore] // Ignore this test by default, as it requires a valid OpenAI API key and URL.
    async fn test_generate() {
        #[cfg(debug_assertions)]
        dotenv::dotenv().ok();

        let client = LlmClient::builder()
            .api_key(std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"))
            .api_url(std::env::var("OPENAI_API_URL").expect("OPENAI_API_URL must be set"))
            .build();

        let instructions = "Return a JSON object with a message field.";
        let input = "Hello, world!";

        let result: TestResponse = client.generate(instructions, input).await.unwrap();

        assert_eq!(result.message, "Hello, world!");
    }
}
