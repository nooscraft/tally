# Guide: Adding Support for New Models

This guide will help you add support for a new LLM model/provider to prompt-tokens.

> **Important**: Before starting, please review the [Rust Best Practices](../OPEN_SOURCE_PLAN.md#-rust-best-practices) section to ensure your code follows project standards.

## Overview

Adding a new model involves:
1. Understanding the model's tokenizer
2. Implementing the `Tokenizer` trait
3. Registering the model in the registry
4. Adding pricing information
5. Writing tests
6. Updating documentation

## Step-by-Step Guide

### Step 1: Research the Model

Before starting, gather information about:
- **Tokenizer type**: What tokenization algorithm does it use?
- **Tokenizer library**: Is there an existing Rust crate?
- **Pricing**: What are the input/output costs per 1K tokens?
- **Model variants**: Are there different versions (e.g., gpt-4, gpt-4-turbo)?

### Step 2: Create Issue (Optional but Recommended)

Create a "Model Support Request" issue first to:
- Get feedback from maintainers
- Ensure no one else is working on it
- Discuss implementation approach

### Step 3: Implement the Tokenizer

#### 3.1 Create Tokenizer File

Create a new file in `src/tokenizers/`:
```rust
// src/tokenizers/provider_name.rs

use crate::tokenizers::{Tokenizer, TokenizerError};

pub struct ProviderNameTokenizer {
    // Add fields as needed
}

impl ProviderNameTokenizer {
    pub fn new(model_name: &str) -> Result<Self, TokenizerError> {
        // Initialize tokenizer
    }
}

impl Tokenizer for ProviderNameTokenizer {
    fn encode(&self, text: &str) -> Result<Vec<usize>, TokenizerError> {
        // Implement encoding
    }

    fn decode(&self, tokens: &[usize]) -> Result<String, TokenizerError> {
        // Implement decoding
    }

    fn count_tokens(&self, text: &str) -> Result<usize, TokenizerError> {
        // Optimized token counting (can call encode and count)
        let tokens = self.encode(text)?;
        Ok(tokens.len())
    }

    fn name(&self) -> &str {
        "provider-name"
    }

    fn input_price_per_1k(&self) -> Option<f64> {
        // Return pricing if known
        Some(0.03)
    }

    fn output_price_per_1k(&self) -> Option<f64> {
        Some(0.06)
    }
}
```

#### 3.2 Common Tokenizer Types

**OpenAI (tiktoken):**
```rust
use tiktoken_rs::{get_bpe_from_model, CoreBPE};

pub struct OpenAITokenizer {
    bpe: CoreBPE,
}

impl OpenAITokenizer {
    pub fn new(model: &str) -> Result<Self, TokenizerError> {
        let bpe = get_bpe_from_model(model)
            .map_err(|e| TokenizerError::InitializationFailed(e.to_string()))?;
        Ok(Self { bpe })
    }
}
```

**SentencePiece (Claude, LLaMA):**
```rust
use sentencepiece::SentencePieceProcessor;

pub struct SentencePieceTokenizer {
    processor: SentencePieceProcessor,
}

impl SentencePieceTokenizer {
    pub fn new(model_path: &str) -> Result<Self, TokenizerError> {
        let processor = SentencePieceProcessor::from_file(model_path)
            .map_err(|e| TokenizerError::InitializationFailed(e.to_string()))?;
        Ok(Self { processor })
    }
}
```

**Custom/Regex-based:**
If no library exists, you may need to implement a custom tokenizer based on the model's documentation.

#### 3.3 Register in Module

Add to `src/tokenizers/mod.rs`:
```rust
pub mod provider_name;

pub use provider_name::ProviderNameTokenizer;
```

### Step 4: Add to Model Registry

Update `src/models/registry.rs`:

```rust
use crate::tokenizers::{OpenAITokenizer, ProviderNameTokenizer};

pub fn get_tokenizer(model_name: &str) -> Result<Box<dyn Tokenizer>, ModelError> {
    match model_name {
        // Existing models...
        "provider-model-1" | "provider-model-2" => {
            Ok(Box::new(ProviderNameTokenizer::new(model_name)?))
        }
        _ => Err(ModelError::UnsupportedModel(model_name.to_string()))
    }
}

pub fn get_model_info(model_name: &str) -> Option<ModelInfo> {
    match model_name {
        // Existing models...
        "provider-model-1" => Some(ModelInfo {
            provider: "ProviderName",
            model: "model-1",
            input_price: 0.03,
            output_price: 0.06,
        }),
        _ => None
    }
}
```

### Step 5: Update Pricing Configuration

Add to `src/config/pricing.toml`:

```toml
[provider_name.model-1]
input = 0.03
output = 0.06

[provider_name.model-2]
input = 0.025
output = 0.05
```

### Step 6: Write Tests

Create comprehensive tests in `src/tokenizers/provider_name.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let tokenizer = ProviderNameTokenizer::new("model-1").unwrap();
        let text = "Hello, world!";
        let tokens = tokenizer.encode(text).unwrap();
        let decoded = tokenizer.decode(&tokens).unwrap();
        assert_eq!(text, decoded);
    }

    #[test]
    fn test_count_tokens() {
        let tokenizer = ProviderNameTokenizer::new("model-1").unwrap();
        let count = tokenizer.count_tokens("Hello, world!").unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_unicode_handling() {
        let tokenizer = ProviderNameTokenizer::new("model-1").unwrap();
        let text = "Hello 世界 🌍";
        let count = tokenizer.count_tokens(text).unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_empty_string() {
        let tokenizer = ProviderNameTokenizer::new("model-1").unwrap();
        let count = tokenizer.count_tokens("").unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_known_token_count() {
        // Test against a known prompt with verified token count
        let tokenizer = ProviderNameTokenizer::new("model-1").unwrap();
        let text = "The quick brown fox jumps over the lazy dog.";
        let count = tokenizer.count_tokens(text).unwrap();
        // Verify against known count from provider API or documentation
        assert_eq!(count, EXPECTED_COUNT);
    }
}
```

### Step 7: Integration Test

Add integration test in `tests/integration/tokenizer_tests.rs`:

```rust
#[test]
fn test_provider_model_integration() {
    let tokenizer = get_tokenizer("provider-model-1").unwrap();
    let prompt = read_test_fixture("prompts/simple.txt");
    let count = tokenizer.count_tokens(&prompt).unwrap();
    assert!(count > 0);
}
```

### Step 8: Update Documentation

1. **README.md**: Add model to supported models list
2. **docs/ADDING_MODELS.md**: Add example if complex
3. **CHANGELOG.md**: Note the new model support

### Step 9: Validate Against Provider API

If possible, validate your implementation against the actual provider API:

```rust
#[test]
#[ignore] // Only run when API key is available
fn test_against_provider_api() {
    let tokenizer = ProviderNameTokenizer::new("model-1").unwrap();
    let text = "Test prompt";
    let our_count = tokenizer.count_tokens(&text).unwrap();
    
    // Call actual API (if available)
    let api_count = call_provider_api(&text).unwrap();
    
    assert_eq!(our_count, api_count);
}
```

## Common Patterns

### Multiple Models with Same Tokenizer

If multiple models use the same tokenizer:

```rust
impl ProviderNameTokenizer {
    pub fn new(model_name: &str) -> Result<Self, TokenizerError> {
        match model_name {
            "model-1" | "model-2" | "model-3" => {
                // Same initialization
            }
            _ => Err(TokenizerError::UnsupportedModel(model_name.to_string()))
        }
    }
}
```

### Model-Specific Pricing

Handle different pricing per model:

```rust
impl Tokenizer for ProviderNameTokenizer {
    fn input_price_per_1k(&self) -> Option<f64> {
        match self.model_name.as_str() {
            "model-1" => Some(0.03),
            "model-2" => Some(0.025),
            _ => None
        }
    }
}
```

### Lazy Initialization

For tokenizers that require heavy initialization:

```rust
use std::sync::OnceLock;

static TOKENIZER: OnceLock<ProviderNameTokenizer> = OnceLock::new();

pub fn get_tokenizer() -> &'static ProviderNameTokenizer {
    TOKENIZER.get_or_init(|| {
        ProviderNameTokenizer::new().expect("Failed to initialize tokenizer")
    })
}
```

## Testing Checklist

- [ ] Unit tests for encode/decode
- [ ] Unit tests for count_tokens
- [ ] Test with empty string
- [ ] Test with unicode characters
- [ ] Test with emojis
- [ ] Test with code blocks
- [ ] Test with markdown
- [ ] Integration test
- [ ] Validate against provider API (if possible)
- [ ] Test error handling (invalid input, etc.)

## Common Issues

### Issue: No Rust library available

**Solution**: 
- Implement based on documentation
- Port reference implementation from Python/JavaScript
- Use regex-based approximation (with clear documentation)

### Issue: Tokenizer requires model files

**Solution**:
- Bundle model files in the crate (if small)
- Download on first use (with caching)
- Require users to provide model file path

### Issue: Pricing not publicly available

**Solution**:
- Mark pricing as `None` in implementation
- Allow users to provide pricing via config
- Document that pricing is approximate

## Example: Complete Implementation

See `src/tokenizers/openai.rs` for a complete reference implementation.

## Getting Help

If you encounter issues:
1. Check existing tokenizer implementations
2. Ask in GitHub Discussions
3. Create an issue with your questions
4. Reach out to maintainers

## Submitting Your Contribution

Once complete:
1. Ensure all tests pass
2. Run `cargo clippy` and fix warnings
3. Update documentation
4. Create a pull request with:
   - Clear description
   - Link to related issue (if any)
   - Test results
   - Example usage

Thank you for contributing! 🎉

