# enc

This library aids in processing encoded data.

## Dependencies & Features

Dependency:

    enc = { version = "0.7.0", features = ["full"] }

Primary Features:

- **full**
- **base-64**
- **hex**
- **percent**
- **var-int**

For more non encoding format features see the `Cargo.toml`.

## Data & Value Encoding

There are separate traits for handling encoded data and encoded values.

The `src/data` folder holds the traits: `Encoder`, `StringEncoder`, `Decoder` and `Validator`. These provide an
interface for encoders that operate on slices of data such as `hex` & `base-64`.

The `src/value` folder holds the traits: `EncodedLen`, `EncodeToSlice`, `EncodeToWrite` and more. These provide an
interface for values that know how to encode and decode themselves such as `var-int`s and can be easily extended by
custom types.

## Testing

The code currently has unit tests but much more testing is definitely needed. In the future there should be more
comprehensive unit testing along with fuzz testing.

## Performance

The code is written to be reasonably performant but no performance testing has been done. When performance testing is
done there will be 2 goals:

1. Remove `unsafe` where possible.
    - I don't believe unsafe code is bad, but I do believe it should be avoided when it does not improve the
      performance or usability of the code.
2. When processing `data`, optimize for small data.
    - Encoding large blocks of data with `base-64` or `hex` is typically a signal for a poorly designed system. There
      are still use cases for this but in general the optimizations should target small blocks of data and when
      optimizations for large blocks are implemented it should not come at the expense of small blocks.
