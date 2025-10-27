# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions workflow for automated building and releasing
- Cross-platform binary releases (Linux, Windows, macOS Intel/ARM)
- Comprehensive CI pipeline with testing, formatting, and security checks
- Issue templates for bug reports and feature requests
- Pull request template for structured contributions
- Dependabot configuration for automated dependency updates

### Changed
- Enhanced documentation with binary release instructions

### Fixed
- N/A

### Security
- Added security audit checks in CI pipeline

## [0.1.0] - 2025-01-XX

### Added
- Initial release of Learning Programming App
- File watching and automatic code execution
- Support for Python (.py) and Go (.go) files
- SQLite-based execution history persistence
- Execution statistics and reporting
- Go learning problem generator with 10 sections
- Error-tolerant continuous operation
- Command-line interface with multiple commands:
  - `watch` - Start file monitoring
  - `sections` - List available sections
  - `history` - Show execution history
  - `stats` - Display execution statistics
  - `clear` - Clear execution history
  - `run` - Execute specific file once
  - `generate-go` - Generate Go learning problems
- Comprehensive test suite
- Documentation with sequence diagrams, workflows, and ER diagrams

### Technical Features
- Asynchronous file watching using `notify` crate
- Modular architecture with separate CLI, Core, and Handler components
- Extensible language handler system
- Graceful error handling and recovery
- Cross-platform compatibility (Windows, macOS, Linux)

### Dependencies
- Rust 1.82+ (2021 edition)
- tokio (async runtime)
- clap (CLI parsing)
- sqlx (database operations)
- notify (file watching)
- console (terminal styling)
- And other supporting crates

[Unreleased]: https://github.com/your-username/learning-programming-app/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-username/learning-programming-app/releases/tag/v0.1.0