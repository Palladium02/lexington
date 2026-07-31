use lexington::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Kind {
    LBrace,
    RBrace,
    Comma,
    Colon,
    StringLiteral,
}

fn main() {
    let input = Utf8(include_str!("./simple.json"));
    let source_map = input.source_map();
    let lexer = Lexer::builder()
        .rule("LBrace", symbol('{').kind(Kind::LBrace))
        .rule("RBrace", symbol('}').kind(Kind::RBrace))
        .rule("Comma", symbol(',').kind(Kind::Comma))
        .rule("Colon", symbol(':').kind(Kind::Colon))
        .rule(
            "StringLiteral",
            symbol('"')
                .then(ascii_alphanumeric().or(ascii_whitespace()).many())
                .then(symbol('"'))
                .kind(Kind::StringLiteral),
        )
        .rule("whitespace", ascii_whitespace().skip())
        .build(&input);

    lexer
        .filter_map(|event| match event {
            Event::Token(token) => Some(token),
            Event::Error(_, _) => None,
        })
        .for_each(|token| {
            let location = source_map.location(token.span().start());
            println!("{:?} found at {:?}", token.kind(), location);
        });
}
