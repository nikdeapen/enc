# enc

This library aids in processing encoded data.

## Dependencies & Features

Dependency:

    enc = { version = "0.8.0", features = ["full"] }

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

The `src/value` folder holds the traits: `EncodedLen`, `EncodeToSlice`, `EncodeToWrite` etc. These provide an interface
for values that know how to encode and decode themselves such as `var-int`s and can be easily extended by custom types.
