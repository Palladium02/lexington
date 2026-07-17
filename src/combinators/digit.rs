use crate::{
    combinators::symbol::symbol,
    input::Input,
    matcher::{Matcher, MatcherExt},
};

pub fn digit<I>() -> impl Matcher<I> + Sized + 'static
where
    I: Input<Symbol = char> + Sized + 'static,
{
    symbol('0')
        .or(symbol('1'))
        .or(symbol('2'))
        .or(symbol('3'))
        .or(symbol('4'))
        .or(symbol('5'))
        .or(symbol('6'))
        .or(symbol('7'))
        .or(symbol('8'))
        .or(symbol('9'))
}
