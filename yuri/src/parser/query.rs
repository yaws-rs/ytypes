//! URI Query Parsing

use crate::{error::QueryError, Query};
use logos::{Lexer, Logos};

#[derive(Debug, Logos)]
pub(super) enum QueryToken<'uri> {
    #[regex(r"[#]", priority = 200)]
    MaybeLocationStart(&'uri str),

    #[regex(r"[^#]+", |lex| lex.slice(), priority = 100)]
    MaybeSomethingElse(&'uri str),
}

pub(super) fn parse_query<'uri>(
    lexer: &mut Lexer<'uri, QueryToken<'uri>>,
) -> Result<(Option<Query<'uri>>, Option<&'uri str>), QueryError<'uri>> {
    let mut res: Option<Query<'uri>> = None;
    let mut carry: Option<&'uri str> = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(QueryToken::MaybeSomethingElse(whatever)) => {
                res = Some(Query {
                    raw_query: whatever,
                });
            }
            Ok(QueryToken::MaybeLocationStart(start)) => {
                carry = Some(start);
                break;
            }
            _ => {
                let cut_slice = &lexer.source()[lexer.span().start..];
                let cut_span = &lexer.source()[lexer.span().start..lexer.span().end];

                let detail = crate::error::ParsingDetail {
                    component: "query",
                    span_start: lexer.span().start,
                    span_end: lexer.span().end,
                    source: lexer.source(),
                    clipped_span: cut_span,
                    clipped_remaining: cut_slice,
                };
                return Err(QueryError::ParsingDetailed(detail));
            }
        }
    }

    // Query is optional
    Ok((res, carry))
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("baz=nn#nn", "nn", Ok((Some(Query { raw_query: "baz=nn" }), Some("#"))))]
    #[case("foo=bar&baz=foo#nn", "nn", Ok((Some(Query { raw_query: "foo=bar&baz=foo" }), Some("#"))))]
    #[case("foo=bar", "", Ok((Some(Query { raw_query: "foo=bar" }), None)))]
    fn t_query(
        #[case] s: &'static str,
        #[case] remaining: &'static str,
        #[case] expected: Result<(Option<Query<'static>>, Option<&str>), QueryError<'static>>,
    ) {
        let mut lexer = QueryToken::lexer(s);
        let a = parse_query(&mut lexer);
        assert_eq!(expected, a);
        assert_eq!(remaining, lexer.remainder());
    }
}
