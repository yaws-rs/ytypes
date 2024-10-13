//! URI Scheme

use logos::{Lexer, Logos};

use crate::{Scheme, SchemeError};

#[derive(Debug, Logos)]
enum SchemeToken<'uri> {
    #[regex(r"[a-z]+", |lex| lex.slice(), priority = 200)]
    MaybeScheme(&'uri str),

    #[regex(r":\/*\/*")]
    SchemeSep,
}

#[derive(Debug, PartialEq)]
enum ParseStage<'uri> {
    WantScheme,
    WantSep(Scheme<'uri>),
}

pub fn parse_scheme<'uri>(
    lexer: &mut Lexer<'uri, SchemeToken<'uri>>,
) -> Result<Scheme<'uri>, SchemeError<'uri>> {
    let mut ret_scheme: Option<&'uri str> = None;
    while let Some(token) = lexer.next() {
        match token {
            Ok(SchemeToken::MaybeScheme(s)) if ret_scheme.is_none() => {
                ret_scheme = Some(s);
            }
            Ok(SchemeToken::SchemeSep) => {
                return match ret_scheme {
                    None => Err(SchemeError::Nothing),
                    Some(s) => Ok(s.try_into().map_err(|_| SchemeError::Invalid)?),
                }
            }
            _ => {
                let cut_slice = &lexer.source()[lexer.span().start..];
                let cut_span = &lexer.source()[lexer.span().start..lexer.span().end];

                let detail = crate::error::ParsingDetail {
                    component: "scheme",
                    span_start: lexer.span().start,
                    span_end: lexer.span().end,
                    source: lexer.source(),
                    clipped_span: cut_span,
                    clipped_remaining: cut_slice,
                };
                return Err(SchemeError::ParsingDetailed(detail));
            }
        }
    }
    Err(SchemeError::RunAway)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ParsingDetail;

    #[test]
    fn parse_http() {
        let mut lexer = SchemeToken::lexer("http://foobar.test");
        let p = parse_scheme(&mut lexer);
        assert_eq!(p, Ok(Scheme::Http(false)));
        assert_eq!(lexer.remainder(), "foobar.test");
    }
    #[test]
    fn parse_https() {
        let mut lexer = SchemeToken::lexer("https://foobar.test");
        let p = parse_scheme(&mut lexer);
        assert_eq!(p, Ok(Scheme::Http(true)));
        assert_eq!(lexer.remainder(), "foobar.test");
    }
    #[test]
    fn parse_https_flexible() {
        let mut lexer = SchemeToken::lexer("https:foobar.test");
        let p = parse_scheme(&mut lexer);
        assert_eq!(p, Ok(Scheme::Http(true)));
        assert_eq!(lexer.remainder(), "foobar.test");
    }
    #[test]
    fn parse_runaway() {
        let mut lexer = SchemeToken::lexer("https");
        let p = parse_scheme(&mut lexer);
        assert_eq!(p, Err(SchemeError::RunAway));
    }
    #[test]
    fn parse_error() {
        let mut lexer = SchemeToken::lexer("http-");
        let p = parse_scheme(&mut lexer);
        assert_eq!(
            p,
            Err(SchemeError::ParsingDetailed(ParsingDetail {
                component: "scheme",
                span_start: 4,
                span_end: 5,
                source: "http-",
                clipped_span: "-",
                clipped_remaining: "-"
            }))
        );
    }
}
