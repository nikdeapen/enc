use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

/// A set of special chars for percent encoding.
///
/// There are 32 possible chars (all other chars are ignored):
/// [1]:        SPACE
/// [4]:        !"#$
/// [10]:       &'()*+,-./
/// [7]:        :;<=>?@
/// [6]:        [\]^_`
/// [4]:        {|}~
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SpecialSet {
    flags: u32,
}

impl SpecialSet {
    //! Characters

    /// The possible chars. (in order by value)
    pub const CHARS: &'static [u8; 32] = b" !\"#$&'()*+,-./:;<=>?@[\\]^_`{|}~";
}

impl Default for SpecialSet {
    fn default() -> Self {
        Self { flags: 0 }
    }
}

impl From<&[u8]> for SpecialSet {
    fn from(chars: &[u8]) -> Self {
        let mut set: Self = Self::default();
        for c in chars {
            set.add(*c);
        }
        set
    }
}

impl From<&str> for SpecialSet {
    fn from(chars: &str) -> Self {
        Self::from(chars.as_bytes())
    }
}

impl SpecialSet {
    //! Search

    /// Gets the index of the char.
    pub(crate) fn index_of(c: u8) -> Option<usize> {
        let segment: Range<usize> = if c <= b':' {
            if c <= b'(' {
                0..8
            } else {
                8..16
            }
        } else {
            if c <= b'\\' {
                16..24
            } else {
                24..32
            }
        };
        let start: usize = segment.start;
        Self::CHARS[segment]
            .iter()
            .position(|b| *b == c)
            .map(|i| i + start)
    }
}

impl SpecialSet {
    //! Mutations

    /// Adds the char. This function has no effect if the char is invalid or is already present.
    pub fn add(&mut self, c: u8) {
        if let Some(index) = Self::index_of(c) {
            self.flags |= 0x80000000u32 >> index;
        }
    }

    /// Removes the char. This function has no effect if the char is invalid or was not present.
    pub fn remove(&mut self, c: u8) {
        if let Some(index) = Self::CHARS.iter().position(|b| *b == c) {
            self.flags &= !(0x80000000u32 >> index);
        }
    }
}

impl SpecialSet {
    //! Properties

    /// Gets the number of chars in the set.
    pub fn size(&self) -> u32 {
        self.flags.count_ones()
    }

    /// Checks if the set contains the char.
    pub fn contains(&self, c: u8) -> bool {
        if let Some(index) = Self::index_of(c) {
            let bit: u32 = 0x80000000u32 >> index;
            (bit & self.flags) != 0
        } else {
            false
        }
    }
}

impl Debug for SpecialSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.flags)
    }
}

impl Display for SpecialSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe {
            String::from_utf8_unchecked(self.iter().collect())
        })
    }
}

impl SpecialSet {
    //! Iteration

    /// Creates an iterator for the special set.
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        SpecialSetIterator { flags: self.flags }
    }
}

// An iterator for a special set.
struct SpecialSetIterator {
    flags: u32,
}

impl Iterator for SpecialSetIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flags == 0 {
            None
        } else {
            let leading_zeros: u32 = self.flags.leading_zeros();
            self.flags &= 0x7FFFFFFFu32 >> leading_zeros;
            Some(SpecialSet::CHARS[leading_zeros as usize])
        }
    }
}

impl Debug for SpecialSetIterator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.flags)
    }
}
