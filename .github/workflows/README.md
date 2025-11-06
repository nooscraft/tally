# GitHub Actions Workflows

This directory contains GitHub Actions workflows for continuous integration and deployment.

## Workflows

### `ci.yml` - Continuous Integration

Runs on every push and pull request to `main` and `develop` branches.

**Jobs:**
1. **Lint & Format** - Checks code formatting and runs Clippy linter
2. **Test (stable)** - Runs tests with different feature combinations
3. **Test Rust Versions** - Tests on stable, beta, and MSRV (1.70.0)
4. **Test Platforms** - Tests on Ubuntu, macOS, and Windows
5. **Security Audit** - Runs `cargo audit` to check for known vulnerabilities

**Features:**
- Caching for faster builds
- Parallel job execution
- Multiple Rust version testing
- Cross-platform testing
- Security vulnerability scanning

## Adding New Workflows

To add a new workflow:
1. Create a new `.yml` file in `.github/workflows/`
2. Follow the same structure as `ci.yml`
3. Use appropriate triggers (`on:` section)
4. Test locally with `act` (optional) before pushing

## Local Testing

You can test workflows locally using [act](https://github.com/nektos/act):

```bash
# Install act
brew install act  # macOS
# or download from https://github.com/nektos/act/releases

# Run the CI workflow
act push

# Run a specific job
act -j lint
```

Note: Some jobs may require Docker and may not work perfectly locally.

