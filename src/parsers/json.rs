/// JSON parser for chat-style message formats.
use crate::error::ParseError;
use crate::parsers::{Message, Parser};
use serde::{Deserialize, Serialize};

/// JSON message format (matching OpenAI chat format).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsonMessage {
    role: String,
    content: String,
}

/// Parser for JSON chat format.
///
/// Supports both single message objects and arrays of messages.
/// Format matches OpenAI's chat API format.
pub struct JsonParser;

impl JsonParser {
    /// Create a new JSON parser.
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for JsonParser {
    fn parse(&self, input: &str) -> Result<Vec<Message>, ParseError> {
        // Try parsing as array first
        if let Ok(messages) = serde_json::from_str::<Vec<JsonMessage>>(input) {
            return Ok(messages
                .into_iter()
                .map(|m| Message {
                    role: m.role,
                    content: m.content,
                })
                .collect());
        }

        // Try parsing as single message
        if let Ok(message) = serde_json::from_str::<JsonMessage>(input) {
            return Ok(vec![Message {
                role: message.role,
                content: message.content,
            }]);
        }

        Err(ParseError::InvalidFormat(
            "Input is not valid JSON message format".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parser_single_message() {
        let parser = JsonParser::new();
        let input = r#"{"role": "user", "content": "Hello!"}"#;
        let messages = parser.parse(input).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[0].content, "Hello!");
    }

    #[test]
    fn test_json_parser_array() {
        let parser = JsonParser::new();
        let input = r#"[{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "Hello!"}]"#;
        let messages = parser.parse(input).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
    }

    #[test]
    fn test_json_parser_invalid() {
        let parser = JsonParser::new();
        let input = "not json";
        assert!(parser.parse(input).is_err());
    }
}
