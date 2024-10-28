//! URI Authority

use logos::{Lexer, Logos};

use crate::UserInfo;
use crate::{error::AuthorityError, Authority};

#[derive(Debug, Logos)]
pub(super) enum AuthorityToken<'uri> {
    #[token("@", priority = 200)]
    At,

    #[token(":", priority = 200)]
    Colon,

    #[regex(r"[/?#]", priority = 200)]
    MaybePathStart(&'uri str),

    #[regex(r"[^@:/?#]+", |lex| lex.slice(), priority = 100)]
    MaybeSomethingElse(&'uri str),
}

#[derive(Debug, PartialEq)]
enum Stage {
    Nowhere,
    WantAt,
    SeenAt,
    SeenFirstBit,
    WantSecondBit,
    SeenSecondBit,
    SeenColon,
    WantHost,
    GotHost,
    WantPort,
    GotPort,
}

pub(super) fn parse_authority<'uri>(
    lexer: &mut Lexer<'uri, AuthorityToken<'uri>>,
) -> Result<(Authority<'uri>, Option<&'uri str>), AuthorityError<'uri>> {
    let mut stage: Stage = Stage::Nowhere;
    let mut host: Option<&'uri str> = None;
    let mut first_bit: Option<&'uri str> = None;
    let mut second_bit: Option<&'uri str> = None;
    let mut port: Option<u16> = None;
    let mut carry: Option<&'uri str> = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(AuthorityToken::At) if stage == Stage::WantSecondBit || stage == Stage::WantAt => {
                stage = Stage::WantHost;
            }
            Ok(AuthorityToken::Colon) if stage == Stage::SeenFirstBit => {
                stage = Stage::WantSecondBit;
            }
            Ok(AuthorityToken::Colon) if stage == Stage::GotHost => {
                stage = Stage::WantPort;
            }
            Ok(AuthorityToken::MaybePathStart(start)) => {
                carry = Some(start);
                break;
            }
            Ok(AuthorityToken::MaybeSomethingElse(something)) if stage == Stage::Nowhere => {
                first_bit = Some(something);
                stage = Stage::SeenFirstBit;
            }
            Ok(AuthorityToken::MaybeSomethingElse(something)) if stage == Stage::WantSecondBit => {
                second_bit = Some(something);
                stage = Stage::WantAt;
            }
            Ok(AuthorityToken::MaybeSomethingElse(something)) if stage == Stage::WantHost => {
                host = Some(something);
                stage = Stage::GotHost;
            }
            Ok(AuthorityToken::MaybeSomethingElse(something)) if stage == Stage::WantPort => {
                port = Some(something.parse().map_err(|e| AuthorityError::InvalidPort)?);
                stage = Stage::GotPort;
            }
            _ => {
                let cut_slice = &lexer.source()[lexer.span().start..];
                let cut_span = &lexer.source()[lexer.span().start..lexer.span().end];

                let detail = crate::error::ParsingDetail {
                    component: "authority",
                    span_start: lexer.span().start,
                    span_end: lexer.span().end,
                    source: lexer.source(),
                    clipped_span: cut_span,
                    clipped_remaining: cut_slice,
                };
                return Err(AuthorityError::ParsingDetailed(detail));
            }
        }
    }

    if stage == Stage::Nowhere {
        return Err(AuthorityError::ParsedNothing);
    }

    if host.is_none() {
        if let Some(first_bit) = first_bit {
            if let Some(second_bit) = second_bit {
                port = Some(
                    second_bit
                        .parse()
                        .map_err(|e| AuthorityError::InvalidPort)?,
                );
            }
            return Ok((
                Authority {
                    userinfo: None,
                    raw_host: first_bit,
                    port,
                },
                carry,
            ));
        }
        return Err(AuthorityError::MissingHost);
    }

    if let Some(host) = host {
        let userinfo = if let Some(first_bit) = first_bit {
            Some(UserInfo {
                raw_user: first_bit,
                raw_authorization: second_bit,
            })
        } else {
            None
        };
        return Ok((
            Authority {
                userinfo,
                raw_host: host,
                port,
            },
            carry,
        ));
    }

    Err(AuthorityError::InvalidAuthority)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::ParsingDetail;
    use rstest::rstest;

    const fn userinfo(user: &'static str, auth: Option<&'static str>) -> Option<UserInfo<'static>> {
        Some(UserInfo {
            raw_user: user,
            raw_authorization: auth,
        })
    }
    const fn auth(
        userinfo: Option<UserInfo<'static>>,
        host: &'static str,
        port: Option<u16>,
    ) -> Authority<'static> {
        Authority {
            userinfo,
            raw_host: host,
            port,
        }
    }

    #[rstest]
    #[case(
        "user:authorization@foo.test:8181/path/nowhere?foo=bar",
        "path/nowhere?foo=bar",
        Ok((auth(userinfo("user", Some("authorization")), "foo.test", Some(8181)), Some("/")))
    )]
    #[case(
        "user:authorization@foo.test:/path/nowhere?foo=bar",
        "path/nowhere?foo=bar",
        Ok((auth(userinfo("user", Some("authorization")), "foo.test", None), Some("/")))
    )]
    #[case(
        "user:authorization@foo.test/path/nowhere?foo=bar",
        "path/nowhere?foo=bar",
        Ok((auth(userinfo("user", Some("authorization")), "foo.test", None), Some("/")))
    )]
    #[case(
        "user:@foo.test/path/nowhere?foo=bar",
        "path/nowhere?foo=bar",
        Ok((auth(userinfo("user", None), "foo.test", None), Some("/")))
    )]
    #[case(
        "foo.test/path/nowhere?foo=bar",
        "path/nowhere?foo=bar",
        Ok((auth(None, "foo.test", None), Some("/")))
    )]
    #[case(
        "foo.test:800/path/nowhere?foo=bar",
        "path/nowhere?foo=bar",
        Ok((auth(None, "foo.test", Some(800)), Some("/")))
    )]
    #[case(
        "foo.test:800?foo=bar",
        "foo=bar",
        Ok((auth(None, "foo.test", Some(800)), Some("?")))
    )]
    #[case(
        "foo.test:800#foobar",
        "foobar",
        Ok((auth(None, "foo.test", Some(800)), Some("#")))
    )]
    #[case(
        ":800/path/nowhere?foo=bar", "800/path/nowhere?foo=bar",
        Err(AuthorityError::ParsingDetailed(ParsingDetail { component: "authority", span_start: 0, span_end: 1, source: ":800/path/nowhere?foo=bar", clipped_span: ":", clipped_remaining: ":800/path/nowhere?foo=bar" }))
    )]
    #[case(
        "/path/nowhere?foo=bar",
        "path/nowhere?foo=bar",
        Err(AuthorityError::ParsedNothing)
    )]
    fn t_authority(
        #[case] s: &'static str,
        #[case] remaining: &'static str,
        #[case] expected: Result<(Authority<'static>, Option<&str>), AuthorityError<'static>>,
    ) {
        let mut lexer = AuthorityToken::lexer(s);
        let a = parse_authority(&mut lexer);
        assert_eq!(expected, a);
        assert_eq!(remaining, lexer.remainder());
    }
}
