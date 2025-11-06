/// JSON formatter for machine-readable output.
use crate::output::{Formatter, TokenResult};
use serde::Serialize;

/// JSON representation of a token result.
#[derive(Debug, Serialize)]
struct JsonResult {
    model: String,
    tokens: usize,
    input_cost: Option<f64>,
    output_cost: Option<f64>,
    breakdown: Option<JsonBreakdown>,
}

/// JSON representation of token breakdown.
#[derive(Debug, Serialize)]
struct JsonBreakdown {
    system: usize,
    user: usize,
    assistant: usize,
    total: usize,
}

/// JSON formatter for machine-readable output.
pub struct JsonFormatter;

impl JsonFormatter {
    /// Create a new JSON formatter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for JsonFormatter {
    fn format_result(&self, result: &TokenResult) -> String {
        let json_result = JsonResult {
            model: result.model.clone(),
            tokens: result.tokens,
            input_cost: result.input_cost,
            output_cost: result.output_cost,
            breakdown: result.breakdown.as_ref().map(|b| JsonBreakdown {
                system: b.system,
                user: b.user,
                assistant: b.assistant,
                total: b.total,
            }),
        };
        serde_json::to_string_pretty(&json_result).unwrap_or_else(|_| "{}".to_string())
    }

    fn format_comparison(&self, results: &[TokenResult]) -> String {
        let json_results: Vec<JsonResult> = results
            .iter()
            .map(|r| JsonResult {
                model: r.model.clone(),
                tokens: r.tokens,
                input_cost: r.input_cost,
                output_cost: r.output_cost,
                breakdown: r.breakdown.as_ref().map(|b| JsonBreakdown {
                    system: b.system,
                    user: b.user,
                    assistant: b.assistant,
                    total: b.total,
                }),
            })
            .collect();
        serde_json::to_string_pretty(&json_results).unwrap_or_else(|_| "[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_result() {
        let formatter = JsonFormatter::new();
        let result = TokenResult {
            model: "gpt-4".to_string(),
            tokens: 100,
            input_cost: Some(0.003),
            output_cost: None,
            breakdown: None,
        };
        let output = formatter.format_result(&result);
        assert!(output.contains("gpt-4"));
        assert!(output.contains("100"));
        assert!(serde_json::from_str::<serde_json::Value>(&output).is_ok());
    }
}
