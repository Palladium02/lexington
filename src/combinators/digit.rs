use crate::{combinators::predicate::predicate, input::Input, matcher::Matcher};

#[must_use]
pub fn digit<I>() -> impl Matcher<I>
where
    I: Input<Symbol = char>,
{
    predicate(|symbol: char| symbol.is_ascii_digit())
}
