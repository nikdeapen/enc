# Future Work

- **Fuzz testing.** Add fuzz tests to find any edge cases in encoding and decoding.
- **Performance.** Profile and optimize encoding & decoding for latency, throughput, and memory usage.
- **Third-party encodings.** Support optional backends like `faster-hex` for improved performance.
- **Serde integration.** Derive `Serialize` and `Deserialize` for encoded types.
- **Async support.** Provide async variants of `EncodeToWrite` and `DecodeFromRead` for non-blocking I/O.
- **No-std support.** Allow use in embedded and `no_std` environments.
- **Unsafe audit.** Review all `unsafe` blocks for soundness and minimize their use.
- **Documentation.** Add doc examples (`///` examples) to public traits and types for docs.rs.
- **Streaming encoding.** Support incremental encoding/decoding for large data that doesn't fit in memory.
- **Additional formats.** Add base-32, base-16, and other common encoding formats.
- **Const construction.** Make more constructors `const` where possible (e.g., `Base64Encoder::new`).
- **Decode from `&str`.** Add convenience methods accepting `&str` to reduce `.as_bytes()` boilerplate.
- **Error context.** Add more context to errors (e.g., byte offset where decoding failed) for easier debugging.
