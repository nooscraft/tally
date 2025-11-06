# Provider Support Plan

## Currently Supported

### OpenAI
- ✅ gpt-4
- ✅ gpt-4-turbo
- ✅ gpt-3.5-turbo
- ✅ gpt-3.5-turbo-16k

## Planned Providers

### Google Gemini
- **Models**: gemini-pro, gemini-pro-vision, gemini-2.5-pro, gemini-2.5-flash
- **Tokenizer**: SentencePiece (similar to other Google models)
- **Pricing**: 
  - Gemini 2.5 Pro: $1.25/$10 per 1M tokens (input/output)
  - Gemini 2.5 Flash: $0.075/$0.30 per 1M tokens
- **Status**: In Progress

### Anthropic Claude
- **Models**: claude-3-opus, claude-3-sonnet, claude-3-haiku, claude-2
- **Tokenizer**: Custom SentencePiece-based
- **Pricing**: Varies by model
- **Status**: Planned

### Mistral AI
- **Models**: mistral-large, mistral-medium, mistral-small, mistral-7b
- **Tokenizer**: SentencePiece
- **Pricing**: Varies by model
- **Status**: Planned

### Cohere
- **Models**: command, command-light, command-nightly
- **Tokenizer**: Custom tokenizer
- **Pricing**: Varies by model
- **Status**: Planned

### AI21 Labs
- **Models**: j2-ultra, j2-mid, j2-grande
- **Tokenizer**: Custom tokenizer
- **Pricing**: Varies by model
- **Status**: Planned

### Meta LLaMA
- **Models**: llama-2-70b, llama-2-13b, llama-2-7b, llama-3
- **Tokenizer**: SentencePiece
- **Pricing**: Open source (local)
- **Status**: Planned

### Hugging Face Models
- **Models**: Various (bert-base, roberta-base, etc.)
- **Tokenizer**: Depends on model (usually SentencePiece or BPE)
- **Pricing**: Open source (local)
- **Status**: Future consideration

## Implementation Priority

1. **High Priority** (v0.2.0)
   - Google Gemini
   - Anthropic Claude

2. **Medium Priority** (v0.3.0)
   - Mistral AI
   - Cohere

3. **Low Priority** (v0.4.0+)
   - AI21 Labs
   - Meta LLaMA
   - Hugging Face models

## Tokenizer Implementation Notes

### SentencePiece-based (Gemini, Claude, Mistral, LLaMA)
- Use `sentencepiece` crate
- May need model files or vocab files
- Consider downloading on first use or bundling

### Custom Tokenizers
- May need to implement based on provider documentation
- Some providers offer Rust SDKs with tokenization
- Consider approximation methods if exact tokenizer unavailable

## Pricing Updates

Pricing should be:
- Loaded from config file
- Updated regularly (consider auto-update mechanism)
- Clearly documented with last update date
- Allow user override via config

