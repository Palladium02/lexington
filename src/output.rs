use std::fmt::Debug;

/// A location inside the original input.
///
/// Spans use half-open ranges:
///
/// ```text
/// start <= position < end
/// ```
///
/// This matches Rust's range conventions and allows empty spans.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }
}

/// A token produced by the lexer.
///
/// Tokens contain their semantic kind and their location inside the source
/// input. The actual text can be retrieved by using the token span with the
/// original input.
#[derive(Debug)]
pub struct Token<K: Debug> {
    kind: K,
    span: Span,
}

impl<K: Debug + Copy> Token<K> {
    #[must_use]
    pub const fn new(kind: K, span: Span) -> Self {
        Self { kind, span }
    }

    #[must_use]
    pub const fn kind(&self) -> K {
        self.kind
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
}
