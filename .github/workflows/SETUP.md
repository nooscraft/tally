# GitHub Actions Setup

## ✅ What's Configured

A comprehensive CI/CD pipeline has been set up for the Tokuin project with the following workflows:

### Main Workflow: `ci.yml`

This workflow runs on every push and pull request to `main` and `develop` branches.

#### Jobs Included:

1. **Lint & Format** (`lint`)
   - Checks code formatting with `cargo fmt`
   - Runs Clippy linter with all warnings as errors
   - Ensures code quality standards

2. **Test (stable)** (`test`)
   - Tests with different feature combinations:
     - Default features
     - `openai` only
     - `openai,markdown`
     - `openai,gemini`
     - `all` features
   - Ensures all feature combinations work correctly

3. **Test Rust Versions** (`test-versions`)
   - Tests on multiple Rust versions:
     - `stable` (latest stable)
     - `beta` (latest beta)
     - `1.70.0` (MSRV - Minimum Supported Rust Version)
   - Ensures backward compatibility

4. **Test Platforms** (`test-platforms`)
   - Tests on multiple operating systems:
     - Ubuntu (Linux)
     - macOS
     - Windows
   - Verifies cross-platform compatibility
   - Tests the binary works on each platform

5. **Security Audit** (`audit`)
   - Runs `cargo audit` to check for known vulnerabilities
   - Reports security issues (doesn't fail the build)

## 🚀 How It Works

### Automatic Triggers

The workflow automatically runs when:
- Code is pushed to `main` or `develop` branches
- Pull requests are opened/updated targeting `main` or `develop`

### Caching

The workflow uses GitHub Actions caching to speed up builds:
- Cargo registry cache
- Build artifacts cache
- Separate caches per Rust version and OS

### Parallel Execution

All jobs run in parallel (except where dependencies are specified), making the CI fast and efficient.

## 📊 Viewing Results

1. Go to your repository on GitHub: https://github.com/nooscraft/tokuin
2. Click on the "Actions" tab
3. You'll see all workflow runs with their status
4. Click on any run to see detailed logs for each job

## 🎯 CI Badge

A CI badge has been added to the README.md that shows the status of the latest workflow run:
- ✅ Green: All checks passing
- ❌ Red: Some checks failing
- 🟡 Yellow: Workflow in progress

## 🔧 Customization

### Adding More Tests

To add more test scenarios, edit `.github/workflows/ci.yml` and add to the matrix:

```yaml
strategy:
  matrix:
    features:
      - "your-new-feature"
```

### Adding More Platforms

Add to the `test-platforms` job matrix:

```yaml
os: [ubuntu-latest, macos-latest, windows-latest, your-new-platform]
```

### Skipping CI

To skip CI for a commit, add `[skip ci]` to your commit message:

```bash
git commit -m "Update docs [skip ci]"
```

## 🐛 Troubleshooting

### Workflow Not Running

- Check that the workflow file is in `.github/workflows/`
- Verify the branch name matches (`main` or `develop`)
- Check GitHub Actions is enabled in repository settings

### Tests Failing

- Check the workflow logs in the Actions tab
- Run tests locally: `cargo test`
- Check formatting: `cargo fmt --check`
- Check linting: `cargo clippy --all-targets --all-features`

### Security Audit Failing

The security audit job is set to `continue-on-error: true`, so it won't fail the build. It will just report vulnerabilities. To fix:
1. Update dependencies: `cargo update`
2. Check for patches: `cargo audit fix`
3. Review the audit report in the workflow logs

## 📝 Next Steps

1. **Push the workflow file** to trigger the first CI run:
   ```bash
   git add .github/workflows/ci.yml
   git commit -m "Add GitHub Actions CI workflow"
   git push
   ```

2. **Check the Actions tab** to see the workflow run

3. **Review any failures** and fix them

4. **Optional: Add release workflow** - Uncomment the `build-release` job in `ci.yml` when ready to automate releases

## 🔗 Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust GitHub Actions](https://github.com/actions-rs)
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) - Action used for Rust installation

