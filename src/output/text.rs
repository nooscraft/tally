/// Text formatter for human-readable output.
use crate::output::{Formatter, TokenResult};

/// Text formatter for human-readable output.
pub struct TextFormatter {
    show_breakdown: bool,
}

impl TextFormatter {
    /// Create a new text formatter.
    ///
    /// # Arguments
    ///
    /// * `show_breakdown` - Whether to show role-based breakdown.
    pub fn new(show_breakdown: bool) -> Self {
        Self { show_breakdown }
    }
}

impl Formatter for TextFormatter {
    fn format_result(&self, result: &TokenResult) -> String {
        let mut output = Vec::new();

        output.push(format!("Model: {}", result.model));
        output.push(format!("Tokens: {}", result.tokens));

        if let Some(breakdown) = &result.breakdown {
            if self.show_breakdown {
                output.push(String::new());
                output.push(format!("System:     {} tokens", breakdown.system));
                output.push(format!("User:       {} tokens", breakdown.user));
                output.push(format!("Assistant:   {} tokens", breakdown.assistant));
                output.push("-".repeat(30));
            }
        }

        if let Some(cost) = result.input_cost {
            // Use more precision for very small costs
            if cost < 0.0001 {
                output.push(format!("Cost: ${:.6} (input)", cost));
            } else {
                output.push(format!("Cost: ${:.4} (input)", cost));
            }
        }

        if let Some(cost) = result.output_cost {
            // Use more precision for very small costs
            if cost < 0.0001 {
                output.push(format!("Cost: ${:.6} (output)", cost));
            } else {
                output.push(format!("Cost: ${:.4} (output)", cost));
            }
        }

        // Show total cost if both input and output costs are available
        if let (Some(input_cost), Some(output_cost)) = (result.input_cost, result.output_cost) {
            let total_cost = input_cost + output_cost;
            // Use 6 decimals if either component or total is very small
            if total_cost < 0.001 || input_cost < 0.0001 || output_cost < 0.0001 {
                output.push(format!("Total: ${:.6}", total_cost));
            } else {
                output.push(format!("Total: ${:.4}", total_cost));
            }
        }

        output.join("\n")
    }

    fn format_comparison(&self, results: &[TokenResult]) -> String {
        let mut output = Vec::new();

        // Header
        output.push(format!("{:<20} {:<10} {}", "Model", "Tokens", "Cost"));
        output.push("-".repeat(50));

        // Rows
        for result in results {
            let cost_str = match (result.input_cost, result.output_cost) {
                (Some(input), Some(output)) => {
                    let total = input + output;
                    format!("${:.4} (total)", total)
                }
                (Some(input), None) => format!("${:.4} (input)", input),
                (None, Some(output)) => format!("${:.4} (output)", output),
                (None, None) => "n/a".to_string(),
            };
            output.push(format!(
                "{:<20} {:<10} {}",
                result.model, result.tokens, cost_str
            ));
        }

        output.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_result() {
        let formatter = TextFormatter::new(false);
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
    }

    #[test]
    fn test_format_comparison() {
        let formatter = TextFormatter::new(false);
        let results = vec![
            TokenResult {
                model: "gpt-4".to_string(),
                tokens: 100,
                input_cost: Some(0.003),
                output_cost: None,
                breakdown: None,
            },
            TokenResult {
                model: "gpt-3.5-turbo".to_string(),
                tokens: 95,
                input_cost: Some(0.00015),
                output_cost: None,
                breakdown: None,
            },
        ];
        let output = formatter.format_comparison(&results);
        assert!(output.contains("gpt-4"));
        assert!(output.contains("gpt-3.5-turbo"));
    }
}
