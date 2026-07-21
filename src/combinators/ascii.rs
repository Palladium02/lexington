use crate::{
    combinators::predicate::predicate,
    input::Input,
    matcher::{Matcher, MatcherExt},
};

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher tests if a given character is ascii lowercase.
#[must_use]
pub fn ascii_lower<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    predicate(|symbol: char| symbol.is_ascii_lowercase())
}

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher tests if a given character is ascii uppercase.
#[must_use]
pub fn ascii_upper<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    predicate(|symbol: char| symbol.is_ascii_uppercase())
}

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher succeeds on ascii upper- and lowercase characters.
#[must_use]
pub fn ascii_alpha<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    ascii_lower().or(ascii_upper())
}
