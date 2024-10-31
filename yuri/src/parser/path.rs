//! URI Path Parsing

use crate::{error::PathError, Path};
use logos::{Lexer, Logos};

#[derive(Debug, Logos)]
pub(super) enum PathToken<'uri> {
    #[regex(r"[?#]", priority = 200)]
    MaybeQueryLocationStart(&'uri str),

    #[regex(r"[^?#]+", |lex| lex.slice(), priority = 100)]
    MaybeSomethingElse(&'uri str),
}

pub(super) fn parse_path<'uri>(
    lexer: &mut Lexer<'uri, PathToken<'uri>>,
) -> Result<(Option<Path<'uri>>, Option<&'uri str>), PathError<'uri>> {
    let mut res: Option<Path<'uri>> = None;
    let mut carry: Option<&'uri str> = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(PathToken::MaybeSomethingElse(whatever)) => {
                res = Some(Path { raw_path: whatever });
            }
            Ok(PathToken::MaybeQueryLocationStart(start)) => {
                carry = Some(start);
                break;
            }
            _ => {
                let cut_slice = &lexer.source()[lexer.span().start..];
                let cut_span = &lexer.source()[lexer.span().start..lexer.span().end];

                let detail = crate::error::ParsingDetail {
                    component: "path",
                    span_start: lexer.span().start,
                    span_end: lexer.span().end,
                    source: lexer.source(),
                    clipped_span: cut_span,
                    clipped_remaining: cut_slice,
                };
                return Err(PathError::ParsingDetailed(detail));
            }
        }
    }

    // Path is optional
    Ok((res, carry))
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("foo/bar?baz=nn", "baz=nn", Ok((Some(Path { raw_path: "foo/bar" }), Some("?"))))]
    #[case("foo/bar#nn", "nn", Ok((Some(Path { raw_path: "foo/bar" }), Some("#"))))]
    #[case("foo/bar", "", Ok((Some(Path { raw_path: "foo/bar" }), None)))]
    fn t_path(
        #[case] s: &'static str,
        #[case] remaining: &'static str,
        #[case] expected: Result<(Option<Path<'static>>, Option<&str>), PathError<'static>>,
    ) {
        let mut lexer = PathToken::lexer(s);
        let a = parse_path(&mut lexer);
        assert_eq!(expected, a);
        assert_eq!(remaining, lexer.remainder());
    }
}
