pub mod json;
/// Parsers for different input formats.
pub mod text;

pub use json::JsonParser;
pub use text::TextParser;

use crate::error::ParseError;

/// A message with a role (for chat-style APIs).
#[derive(Debug, Clone)]
pub struct Message {
    /// The role of the message (e.g., "system", "user", "assistant").
    pub role: String,
    /// The content of the message.
    pub content: String,
}

/// Trait for parsing prompts into messages.
pub trait Parser {
    /// Parse input into a list of messages.
    ///
    /// # Arguments
    ///
    /// * `input` - The input text to parse.
    ///
    /// # Returns
    ///
    /// A vector of messages, or an error if parsing fails.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input cannot be parsed.
    fn parse(&self, input: &str) -> Result<Vec<Message>, ParseError>;
}
