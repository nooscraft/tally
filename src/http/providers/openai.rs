/// OpenAI API client implementation.
#[cfg(feature = "load-test")]
use crate::error::AppError;
#[cfg(feature = "load-test")]
use crate::http::client::{ClientConfig, LlmClient, LlmResponse};
#[cfg(feature = "load-test")]
use reqwest::Client;
#[cfg(feature = "load-test")]
use serde::{Deserialize, Serialize};

/// OpenAI API client.
#[cfg(feature = "load-test")]
pub struct OpenAIClient {
    client: Client,
    config: ClientConfig,
}

/// OpenAI API request payload.
#[cfg(feature = "load-test")]
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: Option<f64>,
}

/// Message in OpenAI format.
#[cfg(feature = "load-test")]
#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// OpenAI API response.
#[cfg(feature = "load-test")]
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
    model: Option<String>,
}

/// Choice in OpenAI response.
#[cfg(feature = "load-test")]
#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

/// Message in OpenAI response.
#[cfg(feature = "load-test")]
#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

/// Token usage information.
#[cfg(feature = "load-test")]
#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: Option<usize>,
    completion_tokens: Option<usize>,
    total_tokens: Option<usize>,
}

#[cfg(feature = "load-test")]
impl OpenAIClient {
    /// Create a new OpenAI client.
    pub fn new(config: ClientConfig) -> Result<Self, AppError> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| AppError::Http(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Get the default OpenAI API endpoint.
    fn default_endpoint() -> String {
        "https://api.openai.com/v1/chat/completions".to_string()
    }
}

#[cfg(feature = "load-test")]
#[async_trait::async_trait]
impl LlmClient for OpenAIClient {
    async fn send_request(&self, prompt: &str, model: &str) -> Result<LlmResponse, AppError> {
        let endpoint = if self.config.endpoint.is_empty() {
            Self::default_endpoint()
        } else {
            self.config.endpoint.clone()
        };

        let messages = vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }];

        let request = OpenAIRequest {
            model: model.to_string(),
            messages,
            temperature: Some(0.7),
        };

        let mut req = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json");

        // Add custom headers
        for (key, value) in &self.config.headers {
            req = req.header(key, value);
        }

        let response = req
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Http(format!("Request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Api(format!(
                "API error ({}): {}",
                status, error_text
            )));
        }

        let api_response: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| AppError::Http(format!("Failed to parse JSON response: {}", e)))?;

        let content = api_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| AppError::Api("No response content".to_string()))?;

        let usage = api_response.usage;
        let model_name = api_response.model.unwrap_or_else(|| model.to_string());

        Ok(LlmResponse {
            content,
            input_tokens: usage.as_ref().and_then(|u| u.prompt_tokens),
            output_tokens: usage.as_ref().and_then(|u| u.completion_tokens),
            total_tokens: usage.as_ref().and_then(|u| u.total_tokens),
            model: model_name,
        })
    }

    fn provider_name(&self) -> &str {
        "openai"
    }
}

#[cfg(test)]
#[cfg(feature = "load-test")]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_openai_client_creation() {
        let config = ClientConfig {
            endpoint: String::new(),
            api_key: "test-key".to_string(),
            timeout: Duration::from_secs(60),
            headers: Vec::new(),
        };
        let client = OpenAIClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_default_endpoint() {
        let endpoint = OpenAIClient::default_endpoint();
        assert_eq!(endpoint, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_provider_name() {
        let config = ClientConfig {
            endpoint: String::new(),
            api_key: "test-key".to_string(),
            timeout: Duration::from_secs(60),
            headers: Vec::new(),
        };
        let client = OpenAIClient::new(config).unwrap();
        assert_eq!(client.provider_name(), "openai");
    }
}
