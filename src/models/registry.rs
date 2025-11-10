/// Model registry for managing available models and their tokenizers.
use crate::error::ModelError;
use crate::models::pricing::PricingConfig;
use crate::tokenizers::Tokenizer;

#[cfg(feature = "openai")]
use crate::tokenizers::OpenAITokenizer;

#[cfg(feature = "gemini")]
use crate::tokenizers::GeminiTokenizer;

use std::collections::HashMap;

/// Information about a model.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ModelInfo {
    /// The provider name (e.g., "openai", "anthropic").
    pub provider: String,
    /// The model name/identifier.
    pub model: String,
    /// Input price per 1K tokens in USD.
    pub input_price: Option<f64>,
    /// Output price per 1K tokens in USD.
    pub output_price: Option<f64>,
}

/// Registry for managing models and their tokenizers.
pub struct ModelRegistry {
    models: HashMap<String, ModelInfo>,
}

impl ModelRegistry {
    /// Create a new model registry with default models.
    pub fn new() -> Self {
        let mut registry = Self {
            models: HashMap::new(),
        };
        registry.register_default_models();
        registry
    }

    /// Create a model registry and apply pricing overrides.
    pub fn new_with_pricing(pricing_path: Option<&str>) -> Result<Self, ModelError> {
        let mut registry = Self::new();
        registry.apply_pricing_from(pricing_path)?;
        Ok(registry)
    }

    /// Get information about a model.
    #[allow(dead_code)]
    pub fn get_model_info(&self, model_name: &str) -> Option<&ModelInfo> {
        // Try direct lookup first
        if let Some(info) = self.models.get(model_name) {
            return Some(info);
        }

        // Try aliases
        let alias = self.resolve_alias(model_name);
        self.models.get(&alias)
    }

    /// Create a tokenizer for the specified model.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model (e.g., "gpt-4").
    ///
    /// # Returns
    ///
    /// A boxed tokenizer, or an error if the model is not supported.
    ///
    /// # Errors
    ///
    /// Returns `ModelError::ModelNotFound` if the model is not registered,
    /// or `ModelError::Tokenizer` if the tokenizer cannot be created.
    pub fn get_tokenizer(&self, model_name: &str) -> Result<Box<dyn Tokenizer>, ModelError> {
        let model = self.resolve_alias(model_name);

        #[cfg(feature = "openai")]
        if model.starts_with("gpt-") || model.starts_with("text-") {
            return OpenAITokenizer::new(&model)
                .map(|t| Box::new(t) as Box<dyn Tokenizer>)
                .map_err(ModelError::from);
        }

        #[cfg(feature = "gemini")]
        if model.starts_with("gemini-") {
            // Note: This will fail without a model file, but provides the structure
            // In production, you'd handle model file loading or use an approximation
            return GeminiTokenizer::new(&model)
                .map(|t| Box::new(t) as Box<dyn Tokenizer>)
                .map_err(ModelError::from);
        }

        Err(ModelError::ModelNotFound {
            model: model_name.to_string(),
        })
    }

    /// Register default models.
    fn register_default_models(&mut self) {
        // OpenAI models
        #[cfg(feature = "openai")]
        {
            self.upsert_model("openai", "gpt-4", Some(0.03), Some(0.06));
            self.upsert_model("openai", "gpt-4-turbo", Some(0.01), Some(0.03));
            self.upsert_model("openai", "gpt-3.5-turbo", Some(0.0015), Some(0.002));
        }

        // Gemini models
        #[cfg(feature = "gemini")]
        {
            self.upsert_model("google", "gemini-pro", Some(0.00125), Some(0.01));
            self.upsert_model("google", "gemini-2.5-pro", Some(0.00125), Some(0.01));
            self.upsert_model("google", "gemini-2.5-flash", Some(0.000075), Some(0.0003));
        }
    }

    /// List all registered models.
    #[allow(dead_code)]
    pub fn list_models(&self) -> Vec<&ModelInfo> {
        self.models.values().collect()
    }

    /// Apply pricing overrides from a TOML file or environment variable.
    pub fn apply_pricing_from(&mut self, pricing_path: Option<&str>) -> Result<(), ModelError> {
        let explicit_path = pricing_path
            .map(|p| p.to_string())
            .or_else(|| std::env::var("TOKUIN_PRICING_FILE").ok());

        let Some(path) = explicit_path else {
            return Ok(());
        };

        let config = PricingConfig::from_file(&path).map_err(|e| {
            ModelError::ConfigLoadFailed(format!("Failed to load pricing file '{}': {}", path, e))
        })?;

        self.apply_pricing_config(config);

        Ok(())
    }

    /// Retrieve pricing information for a given model, if available.
    pub fn pricing_for(&self, model_name: &str) -> Option<(f64, f64)> {
        self.get_model_info(model_name).and_then(|info| {
            match (info.input_price, info.output_price) {
                (Some(input), Some(output)) => Some((input, output)),
                _ => None,
            }
        })
    }

    fn apply_pricing_config(&mut self, config: PricingConfig) {
        for (provider, provider_pricing) in config.providers {
            for (model, pricing) in provider_pricing.models {
                self.upsert_model(&provider, &model, Some(pricing.input), Some(pricing.output));
            }
        }
    }

    fn upsert_model(
        &mut self,
        provider: &str,
        model: &str,
        input_price: Option<f64>,
        output_price: Option<f64>,
    ) {
        let info = ModelInfo {
            provider: provider.to_string(),
            model: model.to_string(),
            input_price,
            output_price,
        };

        let keys = [
            model.to_string(),
            format!("{}/{}", provider, model),
            model.to_lowercase(),
        ];

        for key in keys {
            self.models
                .entry(key)
                .and_modify(|existing| {
                    existing.provider = info.provider.clone();
                    existing.model = info.model.clone();
                    existing.input_price = info.input_price;
                    existing.output_price = info.output_price;
                })
                .or_insert_with(|| info.clone());
        }
    }

    fn resolve_alias(&self, model_name: &str) -> String {
        let base = model_name.rsplit('/').next().unwrap_or(model_name);

        match base {
            "gpt-4-turbo" | "gpt-4-turbo-preview" => "gpt-4-turbo-preview".to_string(),
            _ => base.to_string(),
        }
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    #[cfg(feature = "openai")]
    fn test_get_tokenizer() {
        let registry = ModelRegistry::new();
        let tokenizer = registry.get_tokenizer("gpt-4");
        assert!(tokenizer.is_ok());
    }

    #[test]
    fn test_get_model_info() {
        let registry = ModelRegistry::new();
        let info = registry.get_model_info("gpt-4");
        assert!(info.is_some());
        if let Some(info) = info {
            assert_eq!(info.provider, "openai");
            assert_eq!(info.model, "gpt-4");
            assert_eq!(info.input_price, Some(0.03));
            assert_eq!(info.output_price, Some(0.06));
        }
    }

    #[test]
    fn test_list_models() {
        let registry = ModelRegistry::new();
        let models = registry.list_models();
        assert!(!models.is_empty());
    }

    #[test]
    fn apply_pricing_overrides_from_file() {
        let mut temp = NamedTempFile::new().expect("create temp pricing file");
        writeln!(
            temp,
            "[openai]\n[openai.gpt-4]\ninput = 0.02\noutput = 0.04\n"
        )
        .expect("write pricing overrides");

        let path = temp.path().to_str().expect("pricing path utf8");
        let registry =
            ModelRegistry::new_with_pricing(Some(path)).expect("registry with overrides");

        let pricing = registry.pricing_for("gpt-4").expect("pricing present");
        assert_eq!(pricing.0, 0.02);
        assert_eq!(pricing.1, 0.04);

        // Ensure alias lookup still works
        let pricing_alias = registry
            .pricing_for("openai/gpt-4")
            .expect("pricing for provider-prefixed key");
        assert_eq!(pricing_alias.0, 0.02);
    }
}
