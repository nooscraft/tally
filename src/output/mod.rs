pub mod json;
/// Output formatters for displaying results.
pub mod text;

#[cfg(feature = "markdown")]
pub mod markdown;

pub use json::JsonFormatter;
pub use text::TextFormatter;

#[cfg(feature = "markdown")]
pub use markdown::MarkdownFormatter;

/// Token count breakdown by role.
#[derive(Debug, Clone)]
pub struct TokenBreakdown {
    /// Token count for system messages.
    pub system: usize,
    /// Token count for user messages.
    pub user: usize,
    /// Token count for assistant messages.
    pub assistant: usize,
    /// Total token count.
    pub total: usize,
}

impl TokenBreakdown {
    /// Create a new breakdown with all counts set to zero.
    pub fn new() -> Self {
        Self {
            system: 0,
            user: 0,
            assistant: 0,
            total: 0,
        }
    }
}

impl Default for TokenBreakdown {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of token counting.
#[derive(Debug, Clone)]
pub struct TokenResult {
    /// The model name.
    pub model: String,
    /// Total token count.
    pub tokens: usize,
    /// Cost for input tokens (in USD).
    pub input_cost: Option<f64>,
    /// Cost for output tokens (in USD).
    pub output_cost: Option<f64>,
    /// Breakdown by role (if available).
    pub breakdown: Option<TokenBreakdown>,
}

/// Trait for formatting token results.
pub trait Formatter {
    /// Format a single token result.
    fn format_result(&self, result: &TokenResult) -> String;

    /// Format multiple results (for comparison).
    fn format_comparison(&self, results: &[TokenResult]) -> String;
}
