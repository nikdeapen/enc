# enc
This library aids in processing encoded data.

## Dependencies & Features
**Dependency:**

    enc = { version = "0.6.0", features = ["full"] }

**Transitive Dependencies:**

    none

**Features:**

- `full`: all features
- `base-64`: base-64 encoding
- `hex`: hexadecimal encoding
- `percent`: url percent encoding
- `var-int`: variable-length encoded integers

## Data & Value Encoders
There are separate traits for encoding raw data vs arbitrary values.

The `src/data` folder holds the traits: `Encoder`, `TextEncoder`, `Decoder` and `Validator`. These
provide an interface for encoders that operate on slices of data such as `hex` & `base-64`.

The `src/value` folder holds the traits: `EncodedLen`, `EncodeToSlice`, `EncodeToWrite` etc. These
provide the interface for values that know how to encode and decode themselves such as 
`variable-length integers` and can be easily extended by custom types.

## Examples

### Hexadecimal Encoding
Examples for processing hexadecimal encoded data:
    
    /// Imports
    use enc::{Encoder, Decoder, Validator};
    use enc::hex::{HexEncoder, HexDecoder, HexValidator};
    
    /// Encoding 
    let data: &[u8] = b"\x01\x23\x45\x67\x89\xAB\xCD\xEF";
    let encoded: String = HexEncoder::LOWER.encode_as_string(data)?;
    assert_eq!(encoded, "0123456789ABCDEF");

    /// Decoding
    let encoded: &str = "0123456789ABCDEF";
    let decoded: Vec<u8> = HexDecoder::default().decode_as_vec(data)?;
    assert_eq!(decoded, b"\x01\x23\x45\x67\x89\xAB\xCD\xEF");

    /// Validation
    let encoded: &str = "0123456789ABCDEF";
    let valid: bool = HexValidator::CASELESS.is_valid(data)?;
    assert!(valid);

### URL Percent Encoding
Examples for processing URL percent encoded data.

    /// Imports
    use enc::{Encoder, Decoder, Validator};
    use enc::percent::{PercentEncoder, PercentDecoder, PercentValidator}

    /// Encoding
    let encoder: PercentEncoder = "+-!".into(); // these characters won't be encoded
    let data: &[u8] = b"Hello, World!";
    let encoded: String = encoder.encode_as_string(data)?;
    assert_eq(encoded, "Hello%2C%20World!");

    /// Decoding
    let decoder: PercentDecoder = PercentDecoder::default();
    let encoded: &str = "Hello%2C%20World!";
    let decoded: String = decoder.decode_as_vec(encoded.as_slice()).to_string_utf8_unchecked();
    assert_eq(encoded, "Hello, World!");

    /// Validation
    let encoded: &str = "%20%AX";
    let valid: bool = PercentValidator::default().is_valid(data)?;
    assert!(!valid);


### Variable Length Encoded Integers
Examples for processing variable-length encoded integers.

    /// Imports
    use enc::var_int::VarInt64;
    use enc::{DecodeFromReadPrefix, EncodeToSlice};

    /// Encoding
    let value: VarInt64 = VarInt64::from(0xFFFF);
    let encoded: Vec<u8> = value.encode_as_vec();
    assert_eq!(encoded, b"\xFF\xFF\x03");

    /// Decoding
    let encoded: Vec<u8> = b"\xFF\xFF\x03".to_vec();
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(encoded);
    let result: u64 = VarInt64::decode_from_read_prefix(&mut cursor)?.value;
    assert_eq!(result, 0xFFFF);
