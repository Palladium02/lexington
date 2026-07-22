use crate::{combinators::predicate::predicate, input::Input, matcher::Matcher};

#[must_use]
#[deprecated(since = "0.2.0", note = "Use ascii::ascii_digit instead")]
pub fn digit<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    predicate(|symbol: char| symbol.is_ascii_digit())
}
