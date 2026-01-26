# Contributing to WorkBench-Pro

Thank you for your interest in contributing to WorkBench-Pro!

## How to Contribute

### Reporting Bugs

- Check if the bug has already been reported in [Issues](https://github.com/johanmcad/workbench-pro/issues)
- If not, create a new issue with:
  - A clear, descriptive title
  - Steps to reproduce the bug
  - Expected vs actual behavior
  - Your system information (Windows version, CPU, RAM)
  - Any relevant error messages or screenshots

### Suggesting Features

- Open an issue with the "feature request" label
- Describe the feature and why it would be useful
- If possible, outline how it might be implemented

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and ensure the code compiles: `cargo build --release`
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings
- Add comments for complex logic

### Adding New Benchmarks

New benchmarks should:
- Implement the `Benchmark` trait in `src/benchmarks/traits.rs`
- Measure something relevant to developer workflows
- Be reproducible and consistent
- Work on the target platforms
- Include clear documentation of what is being measured

## Development Setup

```bash
# Clone your fork
git clone https://github.com/johanmcad/workbench-pro.git
cd workbench-pro

# Build in debug mode for faster iteration
cargo build

# Run
cargo run
```

## Questions?

Feel free to open an issue for any questions about contributing.
