# enc
This library aids in processing encoded data.

## Dependencies & Features
**Dependency:**

    enc = { version = "0.6.0", features = ["full"] }

**Transitive Dependencies:**

    no dependencies

**Features:**

- `full`: all features
- `hex`: hexadecimal data processing
- `var-int`: variable-length encoded integers

## Data & Value Encoders
There are separate traits for encoding raw data vs arbitrary values.

The `src/data` folder holds the traits: `Encoder`, `TextEncoder`, `Decoder` and `Validator`. These
provide an interface for encoders that operate on slices of data such as `hex` & `base-64`.

The `src/value` folder holds the traits: `EncodedLen`, `EncodeToSlice`, `EncodeToWrite` etc. These
provide the interface for values that know how to encode and decode themselves such as 
`variable-length integers` and can be easily extended by custom types.


## Examples

### Hex
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


### Var-Int
Examples for processing variable-length encoded integers.

    /// Imports
    use crate::var_int::VarInt64;
    use crate::{DecodeFromReadPrefix, EncodeToSlice};

    /// Encoding
    let value: VarInt64 = VarInt64::from(0xFFFF);
    let encoded: Vec<u8> = value.encode_as_vec();
    assert_eq!(encoded, b"\xFF\xFF\x03");

    /// Decoding
    let encoded: Vec<u8> = b"\xFF\xFF\x03".to_vec();
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(encoded);
    let result: u64 = VarInt64::decode_from_read_prefix(&mut cursor)?.value;
    assert_eq!(result, 0xFFFF);
