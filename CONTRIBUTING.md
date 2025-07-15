# Contributing to Regen Network Client

Thank you for your interest in contributing to the Regen Network Client! This guide will help you get set up for development.

## Development Environment

This project uses Nix for reproducible development environments and build processes. The Nix flake provides several apps to streamline development workflows.

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- No need to install Rust, Cargo, or other tools manually - Nix handles everything!

### Getting Started

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd regen
   ```

2. **Enter the development environment**
   ```bash
   nix develop
   ```

   This provides you with:
   - Latest stable Rust with essential extensions
   - Protocol Buffers toolchain
   - Cargo extensions (nextest, audit, etc.)
   - All required system dependencies

## Available Nix Apps

You can run these apps directly without entering the development shell:

### Building

```bash
# Development build
nix run .#build

# Release build
nix run .#build-release
```

### Testing

```bash
# Run all tests
nix run .#test

# Run tests including ignored ones
nix run .#test-ignored
```

### Code Quality

```bash
# Run linting (clippy)
nix run .#lint

# Format all code (Rust + Nix)
nix run .#format

# Security audit
nix run .#audit
```

### Combined Workflows

```bash
# Build + lint + test (efficient with shared cache)
nix run .#build-and-check

# Complete CI-style check (format + lint + test + audit)
nix run .#full-check
```

### Protobuf Management

```bash
# Download Regen Network protobuf definitions
nix run .#download-regen-protos

# Generate Rust types from protobuf definitions
nix run .#generate-regen-types
```

## Recommended Development Workflow

1. **Setup**: `nix develop` (enter development shell)
2. **Development**: Make your changes
3. **Quick check**: `nix run .#build-and-check` (build + lint + test)
4. **Before committing**: `nix run .#full-check` (comprehensive checks)

## Editor Integration

The development shell includes `rust-analyzer` and other LSP tools. Most editors will automatically detect and use them when you're in the Nix environment.

### VS Code

If using VS Code, the rust-analyzer extension should automatically work when you're in the Nix development shell.

### Other Editors

For Vim/Neovim, Emacs, or other editors with LSP support, ensure you start your editor from within the `nix develop` shell to have access to all the tools.

## Code Standards

- **Formatting**: All code must be formatted with `cargo fmt` 
- **Linting**: Code must pass all clippy checks without warnings
- **Testing**: All tests must pass, including ignored integration tests
- **Documentation**: Public APIs should be documented

## Pull Request Process

1. Ensure all checks pass with `nix run .#full-check`
2. Write clear commit messages
3. Update documentation if needed
4. Submit your pull request

## Getting Help

If you run into issues:

1. Make sure you're using the latest version of Nix
2. Try cleaning your environment: exit `nix develop` and re-enter
3. Check that flakes are enabled in your Nix configuration
4. Open an issue if you're still having problems 