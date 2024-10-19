//! URI Authority

use logos::{Lexer, Logos};

use crate::{Authority, error::AuthorityError};
use crate::UserInfo;

#[derive(Debug, Logos)]
pub(super) enum AuthorityToken<'uri> {
    #[token("@", priority = 200)]
    At,

    #[token(":", priority = 200)]
    SemiColon,

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
    SeenSemiColon,
    WantHost,
}

pub(super) fn parse_authority<'uri>(lexer: &mut Lexer<'uri, AuthorityToken<'uri>>,
) -> Result<Authority<'uri>, AuthorityError<'uri>> {

    let mut stage: Stage = Stage::Nowhere;
    let mut host: Option<&'uri str> = None;
    let mut first_bit: Option<&'uri str> = None;
    let mut second_bit: Option<&'uri str> = None;
    
    while let Some(token) = lexer.next() {
        match token {
            Ok(AuthorityToken::At) if stage == Stage::WantSecondBit || stage == Stage::WantAt => {
                stage = Stage::WantHost;
            },
            Ok(AuthorityToken::SemiColon) if stage == Stage::SeenFirstBit => {
                stage = Stage::WantSecondBit;
            },
            Ok(AuthorityToken::MaybePathStart(start)) => {
                break;
            },
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
    
    if first_bit.is_some() && host.is_none() {
        return Authority { userinfo: None, raw_host: first_bit, port };
    }

    if host.is_some() {
        let userinfo = Userinfo { raw_user: first_bit, raw_authorization: second_bit };
        return AUthority { userinfo, raw_host: host, port };
    }
    
    Err(AuthorityError::InvalidAuthority)
}

#[cfg(test)]
mod test {
    use super::*;

//    #[test]
}
