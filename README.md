# enc

This library aids in processing encoded data.

## Features & Dependencies

    enc = { version = "0.16.0-rc.1", features = ["full"] }

### Primary Features

    full
    base-64           
    hex
    percent
    var-int

For more features see the [Feature Flag Docs](https://docs.rs/crate/enc/latest/features).

### Dependencies

This crate has no dependencies.

# Data & Value Encoding

There are separate traits for handling encoded data and encoded values.

The `src/data` folder holds the traits: `Encoder`, `StringEncoder`, `Decoder` and `Validator`. These traits provide an
interface for encoders that operate on byte slices, such as `base-64` & `hex`.

The `src/value` folder holds the traits: `EncodedLen`, `EncodeToSlice`, `EncodeToWrite` etc. These traits provide an
interface for values that know how to encode and decode themselves such as `var-int`.
