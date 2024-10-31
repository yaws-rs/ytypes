//! URI Fragment Parsing

use crate::{error::FragmentError, Fragment};
use logos::{Lexer, Logos};

#[derive(Debug, Logos)]
pub(super) enum FragmentToken<'uri> {
    #[regex(r".+", |lex| lex.slice(), priority = 100)]
    MaybeFragment(&'uri str),
}

pub(super) fn parse_fragment<'uri>(
    lexer: &mut Lexer<'uri, FragmentToken<'uri>>,
) -> Result<(Option<Fragment<'uri>>, Option<&'uri str>), FragmentError<'uri>> {
    let mut res: Option<Fragment<'uri>> = None;
    let mut carry: Option<&'uri str> = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(FragmentToken::MaybeFragment(whatever)) => {
                res = Some(Fragment {
                    raw_fragment: whatever,
                });
            }
            _ => {
                let cut_slice = &lexer.source()[lexer.span().start..];
                let cut_span = &lexer.source()[lexer.span().start..lexer.span().end];

                let detail = crate::error::ParsingDetail {
                    component: "fragment",
                    span_start: lexer.span().start,
                    span_end: lexer.span().end,
                    source: lexer.source(),
                    clipped_span: cut_span,
                    clipped_remaining: cut_slice,
                };
                return Err(FragmentError::ParsingDetailed(detail));
            }
        }
    }

    // Fragment is optional
    Ok((res, carry))
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("nn", "", Ok((Some(Fragment { raw_fragment: "nn" }), None)))]
    fn t_fragment(
        #[case] s: &'static str,
        #[case] remaining: &'static str,
        #[case] expected: Result<(Option<Fragment<'static>>, Option<&str>), FragmentError<'static>>,
    ) {
        let mut lexer = FragmentToken::lexer(s);
        let a = parse_fragment(&mut lexer);
        assert_eq!(expected, a);
        assert_eq!(remaining, lexer.remainder());
    }
}
