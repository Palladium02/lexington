use std::fmt::Display;

/// A source of symbols that can be consumed by a lexer.
///
/// `Input` abstracts over the representation of the source data. The lexer
/// operates on logical symbols while the input implementation handles the
/// underlying encoding.
///
/// Implementations may represent symbols using different storage formats,
/// such as UTF-8 strings or raw bytes.
pub trait Input {
    /// The logical unit consumed by matchers
    ///
    /// For example:
    /// - `char` for UTF-8 text
    /// - `u8` for byte-oriented input
    type Symbol: Copy + Eq + Display; // we add Format here so Matcher can include the symbol in the error message using format!

    /// Reads the symbol starting at `offset`.
    ///
    /// Returns the symbol and the offset immediately after it.
    ///
    /// The returned offset is used by [`Cursor`] to advance through the input.
    ///
    /// Returns `None` when the end  of input is reached or when the offset does not point to a valid symbol boundary.
    fn read(&self, offset: usize) -> Option<(Self::Symbol, usize)>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A convinience implementation of the input trait for string slices.
#[derive(Clone, Copy)]
pub struct Utf8<'a>(pub &'a str);

impl Input for Utf8<'_> {
    type Symbol = char;

    fn read(&self, offset: usize) -> Option<(Self::Symbol, usize)> {
        let remainder = self.0.get(offset..)?;
        let character = remainder.chars().next()?;
        let next = offset + character.len_utf8();

        Some((character, next))
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
