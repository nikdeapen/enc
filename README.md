# enc

This library aids in processing encoded data.

## Features & Dependencies

    enc = { version = "0.14.0-rc.2", features = ["full"] }

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

## Future

The following improvements are planned but not in progress:

- Adding better tests, including fuzzing.
- Improving performance, it is already good but could use SIMD.
- Chunked or partial encoding, to help avoid allocating buffers.
- Adding better testing utilities, especially for data.
- Add more encodings, current priorities are base-32, 36, & 62.

Please ask if you need any of the above, I am also happy to take PRs.
