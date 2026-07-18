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

    let events = lexer.collect::<Vec<_>>();
    println!("{events:?}")
}
