# Issues

## Bugs

- **`impl_encode_to_write_stack_buf!` can cause undefined behavior.** The macro sizes a stack buffer from its
  `max_encoded_len` argument and fills it with `EncodeToSlice::encode_to_slice_unchecked`, which writes through
  `get_unchecked_mut`. A `max_encoded_len` smaller than the type's real maximum writes past the end of the buffer.
  Miri confirms the out-of-bounds write. Macros cannot be `unsafe`, so the mistake — the macro argument itself —
  carries no `unsafe` token, and the downstream `unsafe fn` in the call stack can be correct. Moving the maximum onto
  a trait as an associated const, so the macro reads it instead of taking it as an argument, removes the argument
  that can be wrong and costs nothing at runtime.

## Future Work

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
