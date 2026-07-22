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

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher succeeds on ascii digits.
#[must_use]
pub fn ascii_digit<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    predicate(|symbol: char| symbol.is_ascii_digit())
}

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher succeeds on ascii alphanumeric characters.
#[must_use]
pub fn ascii_alphanumeric<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    ascii_alpha().or(ascii_digit())
}

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher succeeds on ascii hexdigits.
#[must_use]
pub fn ascii_hex<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    predicate(|symbol: char| symbol.is_ascii_hexdigit())
}

/// Returns a matcher compatible with inputs that use `char` as symbol. The matcher succeeds on ascii whitespace.
#[must_use]
pub fn ascii_whitespace<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    predicate(|symbol: char| symbol.is_ascii_whitespace())
}
