# Lexington

Lexington is a lexer combinator library. At it's core Lexington has three levels, the matcher, the rule and the lexer.
A matcher operates on input and communitcates how far it got. A rule encloses a matcher and pairs it with an action (emit/skip).
Lastly the lexer stores a collection of rules and tries to apply them one by one before deciding on what to emit. A lexer itself is
just an iterator over events (token/error).

## Installation

```sh
cargo add https://github.com/Palladium02/lexington
```

## Example

```rust
use lexington::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Kind {
    Let,
    Identifier,
    Int,
    Eq,
    Semicolon,
}

fn main() {
    let lexer = Lexer::builder()
        .rule("let", just("let").kind(Kind::Let))
        .rule(
            "identifier",
            ascii_lower().between(1..).kind(Kind::Identifier),
        )
        .rule("integer", digit().between(1..).kind(Kind::Int))
        .rule("equals", symbol('=').kind(Kind::Eq))
        .rule("semicolon", symbol(';').kind(Kind::Semicolon))
        .rule("space", symbol(' ').skip())
        .build(&Utf8("let x = 42;"));

    let events = lexer.collect::<Vec<Event<Kind>>>();
    println!("{events:?}")
}
```
