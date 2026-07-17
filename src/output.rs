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
#[derive(Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
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

impl<K: Debug> Token<K> {
    pub fn new(kind: K, span: Span) -> Self {
        Self { kind, span }
    }
}
