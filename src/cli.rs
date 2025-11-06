use crate::error::AppError;
use crate::models::ModelRegistry;
#[cfg(feature = "markdown")]
use crate::output::MarkdownFormatter;
use crate::output::{Formatter, JsonFormatter, TextFormatter, TokenBreakdown, TokenResult};
use crate::parsers::{JsonParser, Parser as InputParser, TextParser};
use crate::tokenizers::Tokenizer;
#[cfg(feature = "markdown")]
use crate::utils::markdown;
/// CLI argument parsing and command execution.
use clap::{Parser, ValueEnum};
use std::io::{self, Read};
#[cfg(feature = "watch")]
use std::path::PathBuf;
#[cfg(feature = "watch")]
use std::time::Duration;

/// Tokuin - Estimate token usage and costs for LLM prompts.
#[derive(Parser, Debug)]
#[command(name = "tokuin")]
#[command(about = "A fast CLI tool to estimate token usage and API costs for LLM prompts")]
#[command(version)]
pub struct Cli {
    /// Input file path (use '-' for stdin or omit for direct text input)
    #[arg(value_name = "FILE|TEXT")]
    pub input: Option<String>,

    /// Model to use for tokenization (e.g., gpt-4, gpt-3.5-turbo)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Compare multiple models
    #[arg(short, long, num_args = 1..)]
    pub compare: Vec<String>,

    /// Show token breakdown by role (system/user/assistant)
    #[arg(short, long)]
    pub breakdown: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    pub format: OutputFormat,

    /// Show pricing information
    #[arg(short, long)]
    pub price: bool,

    /// Strip markdown formatting to show token savings
    #[arg(long)]
    #[cfg(feature = "markdown")]
    pub minify: bool,

    /// Compare two prompts and show token differences
    #[arg(long)]
    pub diff: Option<String>,

    /// Watch file for changes and re-run automatically
    #[arg(short, long)]
    #[cfg(feature = "watch")]
    pub watch: bool,
}

/// Output format options.
#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text output
    Text,
    /// JSON output for scripting
    Json,
    /// Markdown report format
    #[cfg(feature = "markdown")]
    Markdown,
}

impl Cli {
    /// Execute the CLI command.
    pub fn run(self) -> Result<(), AppError> {
        #[cfg(feature = "watch")]
        if self.watch {
            return self.run_watch();
        }

        // Handle diff mode
        if let Some(ref diff_file) = self.diff {
            return self.run_diff(diff_file);
        }

        let registry = ModelRegistry::new();

        // Determine input
        let input = self.get_input()?;

        // Apply minify if requested
        #[cfg(feature = "markdown")]
        let original_input = if self.minify {
            let stripped = markdown::strip_markdown(&input);
            let savings = markdown::calculate_savings(&input, &stripped);
            eprintln!(
                "Markdown stripped: {} characters saved (~{} tokens)",
                savings,
                savings / 4
            );
            stripped
        } else {
            input.clone()
        };

        #[cfg(not(feature = "markdown"))]
        let original_input = input.clone();

        let breakdown = self.breakdown;
        let price = self.price;

        // Determine models to use
        let models = if !self.compare.is_empty() {
            self.compare.clone()
        } else if let Some(model) = &self.model {
            vec![model.clone()]
        } else {
            return Err(AppError::Parse(crate::error::ParseError::InvalidFormat(
                "No model specified. Use --model or --compare".to_string(),
            )));
        };

        // Parse input
        let parser: Box<dyn InputParser> = if original_input.trim_start().starts_with('{')
            || original_input.trim_start().starts_with('[')
        {
            Box::new(JsonParser::new())
        } else {
            Box::new(TextParser::new())
        };

        let messages = parser.parse(&original_input)?;

        // Process each model
        let mut results = Vec::new();
        for model_name in &models {
            let tokenizer = registry.get_tokenizer(model_name)?;
            let result = Self::count_tokens(&*tokenizer, &messages, model_name, breakdown, price)?;
            results.push(result);
        }

        // Format and print output
        let formatter: Box<dyn Formatter> = match self.format {
            OutputFormat::Text => Box::new(TextFormatter::new(self.breakdown)),
            OutputFormat::Json => Box::new(JsonFormatter::new()),
            #[cfg(feature = "markdown")]
            OutputFormat::Markdown => Box::new(MarkdownFormatter::new(self.breakdown)),
        };

        if results.len() == 1 {
            println!("{}", formatter.format_result(&results[0]));
        } else {
            println!("{}", formatter.format_comparison(&results));
        }

        Ok(())
    }

    /// Get input from file, stdin, or argument.
    fn get_input(&self) -> Result<String, AppError> {
        if let Some(input) = &self.input {
            if input == "-" {
                // Read from stdin
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                Ok(buffer)
            } else {
                // Read from file
                std::fs::read_to_string(input).map_err(|e| {
                    AppError::Io(std::io::Error::other(format!(
                        "Failed to read file '{}': {}",
                        input, e
                    )))
                })
            }
        } else {
            // Read from stdin (if no argument provided)
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }

    /// Count tokens for messages using the specified tokenizer.
    fn count_tokens(
        tokenizer: &dyn Tokenizer,
        messages: &[crate::parsers::Message],
        model_name: &str,
        breakdown: bool,
        price: bool,
    ) -> Result<TokenResult, AppError> {
        let mut total = 0;
        let mut token_breakdown = if breakdown {
            Some(TokenBreakdown::new())
        } else {
            None
        };

        for message in messages {
            let count = tokenizer.count_tokens(&message.content)?;
            total += count;

            if let Some(ref mut bd) = token_breakdown {
                match message.role.as_str() {
                    "system" => bd.system += count,
                    "user" => bd.user += count,
                    "assistant" => bd.assistant += count,
                    _ => {}
                }
            }
        }

        if let Some(ref mut bd) = token_breakdown {
            bd.total = total;
        }

        // Calculate costs
        let input_cost = if price {
            tokenizer
                .input_price_per_1k()
                .map(|price| (total as f64 / 1000.0) * price)
        } else {
            None
        };

        let output_cost = if price {
            tokenizer
                .output_price_per_1k()
                .map(|price| (total as f64 / 1000.0) * price)
        } else {
            None
        };

        Ok(TokenResult {
            model: model_name.to_string(),
            tokens: total,
            input_cost,
            output_cost,
            breakdown: token_breakdown,
        })
    }

    /// Run in diff mode, comparing two prompts.
    fn run_diff(&self, diff_file: &str) -> Result<(), AppError> {
        let registry = ModelRegistry::new();

        // Get both inputs
        let input1 = self.get_input()?;
        let input2 = std::fs::read_to_string(diff_file).map_err(|e| {
            AppError::Io(std::io::Error::other(format!(
                "Failed to read diff file '{}': {}",
                diff_file, e
            )))
        })?;

        // Determine model
        let model = self.model.as_ref().ok_or_else(|| {
            AppError::Parse(crate::error::ParseError::InvalidFormat(
                "Model required for diff mode. Use --model".to_string(),
            ))
        })?;

        let tokenizer = registry.get_tokenizer(model)?;

        // Parse both inputs
        let parser1: Box<dyn InputParser> =
            if input1.trim_start().starts_with('{') || input1.trim_start().starts_with('[') {
                Box::new(JsonParser::new())
            } else {
                Box::new(TextParser::new())
            };

        let parser2: Box<dyn InputParser> =
            if input2.trim_start().starts_with('{') || input2.trim_start().starts_with('[') {
                Box::new(JsonParser::new())
            } else {
                Box::new(TextParser::new())
            };

        let messages1 = parser1.parse(&input1)?;
        let messages2 = parser2.parse(&input2)?;

        // Count tokens for both
        let result1 = Self::count_tokens(&*tokenizer, &messages1, model, false, self.price)?;
        let result2 = Self::count_tokens(&*tokenizer, &messages2, model, false, self.price)?;

        // Show diff
        let diff = result2.tokens as i64 - result1.tokens as i64;
        println!("Model: {}", model);
        println!("Original: {} tokens", result1.tokens);
        println!("Modified: {} tokens", result2.tokens);
        println!(
            "Difference: {}{} tokens",
            if diff >= 0 { "+" } else { "" },
            diff
        );

        if self.price {
            if let (Some(cost1), Some(cost2)) = (result1.input_cost, result2.input_cost) {
                let cost_diff = cost2 - cost1;
                println!("Cost difference: ${:.4}", cost_diff.abs());
            }
        }

        Ok(())
    }

    /// Run in watch mode, monitoring file for changes.
    #[cfg(feature = "watch")]
    fn run_watch(&self) -> Result<(), AppError> {
        use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
        use std::sync::mpsc;

        let input_file = self.input.as_ref().ok_or_else(|| {
            AppError::Parse(crate::error::ParseError::InvalidFormat(
                "File path required for watch mode".to_string(),
            ))
        })?;

        if input_file == "-" {
            return Err(AppError::Parse(crate::error::ParseError::InvalidFormat(
                "Cannot watch stdin. Provide a file path.".to_string(),
            )));
        }

        let path = PathBuf::from(input_file);
        if !path.exists() {
            return Err(AppError::Parse(crate::error::ParseError::InvalidFormat(
                format!("File not found: {}", input_file),
            )));
        }

        println!(
            "Watching '{}' for changes. Press Ctrl+C to stop.",
            input_file
        );

        // Create channel for file events
        let (tx, rx) = mpsc::channel();

        // Create watcher with config
        let config = Config::default().with_poll_interval(Duration::from_secs(1));
        let mut watcher = RecommendedWatcher::new(tx, config).map_err(|e| {
            AppError::Io(std::io::Error::other(format!(
                "Failed to create file watcher: {}",
                e
            )))
        })?;

        watcher
            .watch(&path, RecursiveMode::NonRecursive)
            .map_err(|e| {
                AppError::Io(std::io::Error::other(format!(
                    "Failed to watch file: {}",
                    e
                )))
            })?;

        // Run initial analysis
        self.run_once()?;

        // Watch for changes
        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    if event.kind.is_modify() {
                        println!("\n--- File changed, re-analyzing ---\n");
                        if let Err(e) = self.run_once() {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {}", e);
                }
                Err(e) => {
                    eprintln!("Channel error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Run a single analysis (used by watch mode).
    #[cfg(feature = "watch")]
    fn run_once(&self) -> Result<(), AppError> {
        // Create a temporary CLI without watch flag
        let mut cli = self.clone();
        cli.watch = false;
        cli.run()
    }
}

#[cfg(feature = "watch")]
impl Clone for Cli {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
            model: self.model.clone(),
            compare: self.compare.clone(),
            breakdown: self.breakdown,
            format: self.format.clone(),
            price: self.price,
            #[cfg(feature = "markdown")]
            minify: self.minify,
            diff: self.diff.clone(),
            watch: false, // Always false in clone
        }
    }
}
