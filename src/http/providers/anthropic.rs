/// Anthropic API client implementation.
#[cfg(feature = "load-test")]
// TODO: Implement in Phase 4
use crate::error::AppError;
#[cfg(feature = "load-test")]
use crate::http::client::{ClientConfig, LlmClient, LlmResponse};

/// Anthropic API client.
#[cfg(feature = "load-test")]
pub struct AnthropicClient {
    _config: ClientConfig,
}

#[cfg(feature = "load-test")]
impl AnthropicClient {
    /// Create a new Anthropic client.
    pub fn new(_config: ClientConfig) -> Result<Self, AppError> {
        Err(AppError::Config(
            "Anthropic client not yet implemented".to_string(),
        ))
    }
}

#[cfg(feature = "load-test")]
#[async_trait::async_trait]
impl LlmClient for AnthropicClient {
    async fn send_request(&self, _prompt: &str, _model: &str) -> Result<LlmResponse, AppError> {
        Err(AppError::Config(
            "Anthropic client not yet implemented".to_string(),
        ))
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }
}
