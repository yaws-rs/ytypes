//! URI Parser

mod scheme;
use scheme::SchemeToken;

mod authority;
use authority::AuthorityToken;

use crate::{
    error::{SchemeError, UriError},
    Uri,
};

use crate::Scheme;
use logos::{Lexer, Logos};

impl<'uri> TryFrom<&'uri str> for Uri<'uri> {
    type Error = UriError<'uri>;
    fn try_from(raw: &'uri str) -> Result<Uri<'uri>, Self::Error> {
        let mut lexer: Lexer<'uri, SchemeToken<'uri>> = SchemeToken::lexer(raw);
        let scheme = scheme::parse_scheme(&mut lexer).map_err(|e| UriError::Scheme(e))?;

        if lexer.remainder().starts_with("//") {
            lexer.bump(2);
        }

        let authority = match scheme {
            Scheme::Http(_) | Scheme::Ftp(_) | Scheme::Ldap(_) => {
                let mut authority_lexer: Lexer<'uri, AuthorityToken<'uri>> = lexer.morph();
                let authority = Some(
                    authority::parse_authority(&mut authority_lexer)
                        .map_err(|e| UriError::Authority(e))?,
                );
                lexer = authority_lexer.morph();
                authority
            }
            _ => {
                return Err(UriError::Scheme(SchemeError::Unimplemented(scheme)));
            }
        };

        let scheme_data: crate::SchemeData<'uri> = crate::SchemeData { raw: None };

        Ok(Uri {
            scheme,
            authority,
            scheme_data,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::Authority;

    #[test]
    fn parse_https_host_only() {
        let uri = "https://foobar.test";
        let res: Uri<'static> = uri.try_into().unwrap();
        assert_eq!(res.scheme, Scheme::Http(true));
        assert_eq!(
            res.authority,
            Some(Authority {
                userinfo: None,
                raw_host: "foobar.test",
                port: None
            })
        );
    }
}
