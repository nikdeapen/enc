use crate::{DecodeFromSlice, Error};

/// A value that can decode itself from a slice it borrows.
///
/// # Note
/// - The value can keep a reference to the slice instead of copying the encoded data.
/// - A [DecodeFromSlice] value is also a [DecodeFromSliceBorrowed] value that doesn't keep the
///   reference.
pub trait DecodeFromSliceBorrowed<'a>: Sized {
    /// Decodes a value from the `data` slice.
    ///
    /// # Note
    /// - The `data` slice must contain only the encoded value.
    fn decode_from_slice_borrowed(data: &'a [u8]) -> Result<Self, Error>;
}

impl<'a, T> DecodeFromSliceBorrowed<'a> for T
where
    T: DecodeFromSlice,
{
    fn decode_from_slice_borrowed(data: &'a [u8]) -> Result<Self, Error> {
        Self::decode_from_slice(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{DecodeFromSlice, DecodeFromSliceBorrowed, Error};

    /// A string that borrows its encoded data.
    #[must_use]
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    struct TextRef<'a> {
        text: &'a str,
    }

    impl<'a> DecodeFromSliceBorrowed<'a> for TextRef<'a> {
        fn decode_from_slice_borrowed(data: &'a [u8]) -> Result<Self, Error> {
            match std::str::from_utf8(data) {
                Ok(text) => Ok(Self { text }),
                Err(error) => Err(Error::invalid_encoded_data_with(error)),
            }
        }
    }

    /// A string that copies its encoded data.
    #[must_use]
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct Text {
        text: String,
    }

    impl DecodeFromSlice for Text {
        fn decode_from_slice(data: &[u8]) -> Result<Self, Error> {
            let text: TextRef<'_> = TextRef::decode_from_slice_borrowed(data)?;
            Ok(Self {
                text: text.text.to_string(),
            })
        }
    }

    #[test]
    fn text_ref_decode_from_slice_borrowed() {
        let test_cases: &[(&[u8], Option<&str>)] = &[
            (b"", Some("")),
            (b"abc", Some("abc")),
            (b"\xC3\xA9", Some("\u{E9}")),
            (b"\xFF", None),    // invalid
            (b"abc\xC3", None), // truncated
        ];

        for (encoded, expected) in test_cases {
            let result: Option<TextRef<'_>> = TextRef::decode_from_slice_borrowed(encoded).ok();
            let text: Option<&str> = result.map(|result: TextRef<'_>| result.text);
            assert_eq!(text, *expected, "encoded={:?}", encoded);
            if let Some(text) = text {
                // The value reads the encoded data in place.
                assert_eq!(text.as_ptr(), encoded.as_ptr(), "encoded={:?}", encoded);
            }
        }
    }

    #[test]
    fn text_decode_from_slice_borrowed() {
        let test_cases: &[(&[u8], Option<&str>)] = &[
            (b"", Some("")),
            (b"abc", Some("abc")),
            (b"\xC3\xA9", Some("\u{E9}")),
            (b"\xFF", None),    // invalid
            (b"abc\xC3", None), // truncated
        ];

        for (encoded, expected) in test_cases {
            // The value outlives the encoded data.
            let result: Option<Text> = {
                let data: Vec<u8> = encoded.to_vec();
                Text::decode_from_slice_borrowed(data.as_slice()).ok()
            };
            let text: Option<&str> = result.as_ref().map(|result: &Text| result.text.as_str());
            assert_eq!(text, *expected, "encoded={:?}", encoded);
        }
    }
}
