# Contributing to zeitfetch

Thank you for your interest in contributing to zeitfetch! It is much appreciated.

## Development Setup

### Prerequisites

- **Rust** (latest stable version recommended)
- **Cargo** (comes with Rust)

### Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/zeitfetch.git
   cd zeitfetch
   ```

3. Build and run the project:
   ```bash
   cargo run
   ```

## Code Quality

Before submitting any changes, please ensure your code passes quality checks:

### Formatting
```bash
cargo fmt --all --
```

### Linting
```bash
cargo clippy
```

### Testing
Run the test suite:
```bash
cargo test
```

All PRs must pass formatting, linting, and tests before they can be merged.

## Contributing Guidelines

### Pull Request Process

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following our coding standards
3. Test your changes thoroughly
4. Run formatting and linting checks
5. Commit your changes with clear, descriptive messages
6. Push to your fork and submit a pull request

### Code Standards

- Follow Rust conventions and idioms
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions focused and reasonably sized
- Use meaningful variable and function names

### Commit Messages
- By default, try and stick to [conventional commit messages](https://www.conventionalcommits.org/en/v1.0.0/)
- Use clear and descriptive commit messages
- fix, feat, chore, docs, refactor. Any of these will do.
- These are not necessarily enforced but nice to have

## Areas for Contribution

Contributions are welcome in these areas:

### High Priority
- **More distro ASCII Art**: Add support for additional Linux distributions
- **Variable color configs**: Implement customizable colors for text sections
- **Code refactoring**: Improve code structure and maintainability

### Other Contributions
- Bug fixes and performance improvements
- Documentation improvements
- Test coverage expansion
- New feature implementations
- Cross-platform compatibility fixes

## Adding New OS Support

When adding support for a new operating system or distribution:

1. **Detection Logic**: Update `platform_util.rs` with OS detection code
2. **ASCII Art**: Add the logo to `logo.rs` in hex escape code format
3. **Attribution**: Update the README.md with proper credit for the ASCII art
4. **Testing**: Ensure the new OS is properly detected and displays correctly

## Reporting Issues

When reporting bugs or requesting features:

- Use the GitHub issue tracker
- Provide clear steps to reproduce bugs
- Include system information (OS, version, etc.)
- Add relevant error messages or screenshots

## Questions and Support

- Check existing issues before creating new ones
- Join discussions in existing issues and PRs
- Feel free to ask questions about implementation details

## License

By contributing to zeitfetch, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to zeitfetch! 🚀
