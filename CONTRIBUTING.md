# Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on
[GitHub](https://github.com/nikdeapen/enc).

## Guidelines

- **No dependencies.** This crate is intentionally zero-dependency. Do not add external crates.
- **Feature gating.** New functionality should be behind a feature flag.
- **Unsafe code.** Avoid `unsafe` where possible. If necessary, document the safety invariants.
- **Testing.** Run all tests before submitting: `cargo test --features dev`

## Commits

A commit message is a tag with one sentence in the past tense. Example:

```text
[value] Added the slice decoder.
```

There shouldn't be a description.

### Tags

The tag denotes the section of the code base the commit is relevant to.

The project:

- `[cargo]`: the project, typically anything outside the `src` folder.
- `[*]`: multiple sections of the code base.

The features:

- `[data]`: the data encoding traits & utilities.
- `[base-64]`: the base-64 encoding.
- `[hex]`: the hexadecimal encoding.
- `[percent]`: the URL percent encoding.
- `[value]`: the value encoding traits & utilities.
- `[var-int]`: the variable-length encoded integers.
- `[dev]`: the testing utilities.
