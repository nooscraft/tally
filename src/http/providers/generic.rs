/// Generic REST API client implementation.
#[cfg(feature = "load-test")]
// TODO: Implement in Phase 4
use crate::error::AppError;
#[cfg(feature = "load-test")]
use crate::http::client::{ClientConfig, LlmClient, LlmResponse};

/// Generic REST API client.
#[cfg(feature = "load-test")]
pub struct GenericClient {
    _config: ClientConfig,
}

#[cfg(feature = "load-test")]
impl GenericClient {
    /// Create a new generic client.
    pub fn new(_config: ClientConfig) -> Result<Self, AppError> {
        Err(AppError::Config(
            "Generic client not yet implemented".to_string(),
        ))
    }
}

#[cfg(feature = "load-test")]
#[async_trait::async_trait]
impl LlmClient for GenericClient {
    async fn send_request(&self, _prompt: &str, _model: &str) -> Result<LlmResponse, AppError> {
        Err(AppError::Config(
            "Generic client not yet implemented".to_string(),
        ))
    }

    fn provider_name(&self) -> &str {
        "generic"
    }
}
