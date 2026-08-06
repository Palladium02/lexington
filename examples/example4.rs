use lexington::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Kind {
    UpperIdent,
}

fn create_lexer<'a>(input: &'a Utf8<'a>) -> Lexer<'a, Utf8<'a>, Kind> {
    Lexer::builder()
        .rule("upper-ident", ascii_upper().kind(Kind::UpperIdent))
        .build(input)
}

fn main() {
    let input = Utf8("This is some input");
    let _ = create_lexer(&input);
}
