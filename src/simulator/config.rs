/// Configuration for load testing simulator.
#[cfg(feature = "load-test")]
use std::time::Duration;

/// Simulator configuration.
#[cfg(feature = "load-test")]
#[derive(Debug, Clone)]
pub struct SimulatorConfig {
    /// Number of concurrent requests
    pub concurrency: usize,
    /// Total number of requests to make
    pub runs: usize,
    /// Retry count on failure
    pub retry: u32,
    /// Think time between requests (optional)
    pub think_time: Option<ThinkTime>,
    /// Request timeout
    pub timeout: Duration,
    /// Dry run mode (estimate costs without making calls)
    pub dry_run: bool,
    /// Maximum cost threshold (stop if exceeded)
    pub max_cost: Option<f64>,
}

/// Think time configuration.
#[cfg(feature = "load-test")]
#[derive(Debug, Clone)]
pub struct ThinkTime {
    /// Minimum delay in milliseconds
    pub min_ms: u64,
    /// Maximum delay in milliseconds
    pub max_ms: u64,
}

#[cfg(feature = "load-test")]
impl SimulatorConfig {
    /// Create a new simulator configuration.
    pub fn new(concurrency: usize, runs: usize) -> Self {
        Self {
            concurrency,
            runs,
            retry: 3,
            think_time: None,
            timeout: Duration::from_secs(60),
            dry_run: false,
            max_cost: None,
        }
    }

    /// Parse think time from string (e.g., "250-750ms" or "500ms").
    pub fn parse_think_time(s: &str) -> Result<ThinkTime, String> {
        let s = s.trim().to_lowercase();
        let s = s.strip_suffix("ms").unwrap_or(&s);

        if let Some((min_str, max_str)) = s.split_once('-') {
            let min_ms = min_str
                .trim()
                .parse::<u64>()
                .map_err(|_| "Invalid min think time")?;
            let max_ms = max_str
                .trim()
                .parse::<u64>()
                .map_err(|_| "Invalid max think time")?;

            if min_ms > max_ms {
                return Err("Min think time must be <= max think time".to_string());
            }

            Ok(ThinkTime { min_ms, max_ms })
        } else {
            let ms = s
                .trim()
                .parse::<u64>()
                .map_err(|_| "Invalid think time format. Use '250-750ms' or '500ms'".to_string())?;
            Ok(ThinkTime {
                min_ms: ms,
                max_ms: ms,
            })
        }
    }
}

#[cfg(feature = "load-test")]
impl Default for SimulatorConfig {
    fn default() -> Self {
        Self::new(10, 100)
    }
}

#[cfg(test)]
#[cfg(feature = "load-test")]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_config_new() {
        let config = SimulatorConfig::new(5, 50);
        assert_eq!(config.concurrency, 5);
        assert_eq!(config.runs, 50);
        assert_eq!(config.retry, 3);
        assert!(config.think_time.is_none());
        assert!(!config.dry_run);
    }

    #[test]
    fn test_parse_think_time_single() {
        let think_time = SimulatorConfig::parse_think_time("500ms").unwrap();
        assert_eq!(think_time.min_ms, 500);
        assert_eq!(think_time.max_ms, 500);
    }

    #[test]
    fn test_parse_think_time_range() {
        let think_time = SimulatorConfig::parse_think_time("250-750ms").unwrap();
        assert_eq!(think_time.min_ms, 250);
        assert_eq!(think_time.max_ms, 750);
    }

    #[test]
    fn test_parse_think_time_no_suffix() {
        let think_time = SimulatorConfig::parse_think_time("500").unwrap();
        assert_eq!(think_time.min_ms, 500);
        assert_eq!(think_time.max_ms, 500);
    }

    #[test]
    fn test_parse_think_time_invalid_range() {
        let result = SimulatorConfig::parse_think_time("750-250ms");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Min think time must be <= max think time"));
    }

    #[test]
    fn test_parse_think_time_invalid_format() {
        let result = SimulatorConfig::parse_think_time("invalid");
        assert!(result.is_err());
    }
}
