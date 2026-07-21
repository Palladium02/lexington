use crate::input::Input;

/// A position-aware view over an [`Input`].
///
/// A cursor represents a location inside an input and can be copied cheaply.
/// Matchers consume cursors by value and return an updated cursor when they
/// successfully consume input.
///
/// This allows matchers to compose without requiring mutable shared state.
pub struct Cursor<'a, I: Input> {
    input: &'a I,
    offset: usize,
}

impl<'a, I: Input> Clone for Cursor<'a, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, I: Input> Copy for Cursor<'a, I> {}

impl<'a, I: Input> Cursor<'a, I> {
    pub const fn new(input: &'a I, offset: usize) -> Self {
        Self { input, offset }
    }

    /// Returns the symbol at the current position without advancing.
    #[must_use]
    pub fn peek(&self) -> Option<I::Symbol> {
        let (symbol, _) = self.input.read(self.offset)?;
        Some(symbol)
    }

    /// Consumes the current symbol and advances the cursor.
    pub fn advance(&mut self) {
        if let Some((_, next)) = self.input.read(self.offset) {
            self.offset = next;
        }
    }

    /// Returns the current offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Checks if the cursor points to the end of the underlying input.
    #[must_use]
    pub fn eof(&self) -> bool {
        self.offset >= self.input.len()
    }
}
