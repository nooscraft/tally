/// Plain text parser.
use crate::error::ParseError;
use crate::parsers::{Message, Parser};

/// Parser for plain text input.
///
/// Treats the entire input as a single user message.
pub struct TextParser;

impl TextParser {
    /// Create a new text parser.
    pub fn new() -> Self {
        Self
    }
}

impl Default for TextParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for TextParser {
    fn parse(&self, input: &str) -> Result<Vec<Message>, ParseError> {
        Ok(vec![Message {
            role: "user".to_string(),
            content: input.to_string(),
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_parser() {
        let parser = TextParser::new();
        let messages = parser.parse("Hello, world!").unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[0].content, "Hello, world!");
    }

    #[test]
    fn test_text_parser_empty() {
        let parser = TextParser::new();
        let messages = parser.parse("").unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "");
    }
}
