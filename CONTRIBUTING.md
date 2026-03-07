# Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on
[GitHub](https://github.com/nikdeapen/enc).

## Guidelines

- **No dependencies.** This crate is intentionally zero-dependency. Do not add external crates.
- **Feature gating.** New functionality should be behind a feature flag.
- **Unsafe code.** Avoid `unsafe` where possible. If necessary, document the safety invariants.
- **Testing.** Run all tests before submitting: `cargo test --features dev`
