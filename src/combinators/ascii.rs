use crate::{combinators::predicate::predicate, input::Input, matcher::Matcher};

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher tests if a given character is ascii lowercase.
pub fn ascii_lower<I>() -> impl Matcher<I> + Sized + 'static
where
    I: Input<Symbol = char> + Sized + 'static,
{
    predicate(|symbol: char| symbol.is_ascii_lowercase())
}

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher tests if a given character is ascii uppercase.
pub fn ascii_upper<I>() -> impl Matcher<I> + Sized + 'static
where
    I: Input<Symbol = char> + Sized + 'static,
{
    predicate(|symbol: char| symbol.is_ascii_uppercase())
}
