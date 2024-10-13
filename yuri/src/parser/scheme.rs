//! URI Scheme

use logos::{Lexer, Logos};

use crate::Scheme;

#[derive(Debug, Logos)]
enum SchemeToken<'uri> {
    #[regex(r"[a-z]+", |lex| lex.slice(), priority = 200)]
    MaybeScheme(&'uri str),

    #[token(":")]
    SchemeSep,
}
