use init_config::{ConfigLoader, error::ConfigError};

#[derive(Debug, Clone)]
pub struct Config {
    pub llm: LLMConfig,
}

#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub api_key: String,
    pub api_url: Option<String>,
    pub model: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        #[cfg(debug_assertions)]
        dotenv::dotenv().ok();

        Ok(Self {
            llm: LLMConfig {
                api_key: ConfigLoader::load_or_fail("OPENAI_API_KEY")?,
                api_url: ConfigLoader::optional_env_var("OPENAI_API_URL"),
                model: ConfigLoader::optional_env_var("OPENAI_MODEL"),
            },
        })
    }
}
