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
}

pub(super) fn parse_authority<'uri>(
    lexer: &mut Lexer<'uri, AuthorityToken<'uri>>,
) -> Result<Authority<'uri>, AuthorityError<'uri>> {
    let mut stage: Stage = Stage::Nowhere;
    let mut host: Option<&'uri str> = None;
    let mut first_bit: Option<&'uri str> = None;
    let mut second_bit: Option<&'uri str> = None;
    let mut port: Option<u16> = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(AuthorityToken::At) if stage == Stage::WantSecondBit || stage == Stage::WantAt => {
                stage = Stage::WantHost;
            }
            Ok(AuthorityToken::Colon) if stage == Stage::SeenFirstBit => {
                stage = Stage::WantSecondBit;
            }
            Ok(AuthorityToken::MaybePathStart(start)) => {
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
                break;
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
                return Err(AuthorityError::ParsingDetailed(detail));
            }
        }
    }

    if stage == Stage::Nowhere {
        return Err(AuthorityError::ParsedNothing);
    }

    if host.is_none() {
        if let Some(first_bit) = first_bit {
            return Ok(Authority {
                userinfo: None,
                raw_host: first_bit,
                port,
            });
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
        return Ok(Authority {
            userinfo,
            raw_host: host,
            port,
        });
    }

    Err(AuthorityError::InvalidAuthority)
}

#[cfg(test)]
mod test {
    use super::*;
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
        "user:authorization@foo.test:/path/nowhere?foo=bar",
        Ok(auth(userinfo("user", Some("authorization")), "foo.test", None))
    )]
    fn basic_full_authority(
        #[case] s: &'static str,
        #[case] expected: Result<Authority<'static>, AuthorityError<'static>>,
    ) {
        let mut lexer = AuthorityToken::lexer(s);
        let a = parse_authority(&mut lexer);
        assert_eq!(expected, a);
    }
}
