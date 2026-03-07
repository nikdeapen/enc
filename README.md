# enc

[![Crates.io](https://img.shields.io/crates/v/enc.svg)](https://crates.io/crates/enc)
[![Docs.rs](https://docs.rs/enc/badge.svg)](https://docs.rs/enc)
[![License: MIT](https://img.shields.io/crates/l/enc.svg)](https://opensource.org/licenses/MIT)

This library aids in processing encoded data. No dependencies.

    enc = "0.19.0-rc.1"

## Features

    full
    base-64
    hex
    percent
    var-int

For more features see the [Crate Docs](https://docs.rs/enc/latest/enc/).

## Data & Value Encoding

There are separate traits for handling encoded data and encoded values.

The `src/data` folder holds the traits: `Encoder`, `StringEncoder`, `Decoder` and `Validator`. These traits provide an
interface for encoders that operate on byte slices, such as `base-64` & `hex`.

The `src/value` folder holds the traits: `EncodedLen`, `EncodeToSlice`, `EncodeToWrite` etc. These traits provide an
interface for values that know how to encode and decode themselves such as `var-int`.

## Examples

### Base-64
```rust
use enc::base_64::Base64Encoder;
use enc::StringEncoder;

let encoder: Base64Encoder = Base64Encoder::url_safe_encoder();
let encoded: String = encoder.encode_as_string(b"Hello, World!")?;
assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ");
```

### Hex
```rust
use enc::hex::HexEncoder;
use enc::StringEncoder;

let encoded: String = HexEncoder::LOWER.encode_as_string(b"Hello, World!")?;
assert_eq!(encoded, "48656c6c6f2c20576f726c6421");
```

### Percent Encoding
```rust
use enc::percent::PercentEncoder;
use enc::StringEncoder;

let encoder: PercentEncoder = "+-.".into();
let encoded: String = encoder.encode_as_string(b"Hello, World!")?;
assert_eq!(encoded, "Hello%2C%20World%21");
```

### Variable-Length Integers
```rust
use enc::var_int::VarInt32;
use enc::{EncodeToSlice, DecodeFromReadPrefix};

let value: VarInt32 = VarInt32::from(123_456u32);
let bytes: Vec<u8> = value.encode_as_vec()?;

let decoded: VarInt32 = VarInt32::decode_from_read_prefix(&mut bytes.as_slice())?;
assert_eq!(decoded.value(), 123_456u32);
```

## Issues & Contributing

See [ISSUES.md](ISSUES.md) for future work and [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
