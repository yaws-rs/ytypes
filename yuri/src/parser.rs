//! URI Parser

mod scheme;
use scheme::SchemeToken;

mod authority;
use authority::AuthorityToken;

use crate::{Uri, error::{SchemeError, UriError}};

use logos::{Lexer, Logos};
use crate::Scheme;

impl<'uri> TryFrom<&'uri str> for Uri<'uri> {
    type Error = UriError<'uri>;
    fn try_from(raw: &'uri str) -> Result<Uri<'uri>, Self::Error> {
        let mut lexer = SchemeToken::lexer(raw);
        let scheme = scheme::parse_scheme(&mut lexer)
            .map_err(|e| UriError::Scheme(e))?;

        let mut authority = if lexer.remainder().starts_with("//") {
            lexer.bump(2);
            None
        }
        else {
            None
        };

        match scheme {
            Scheme::Http(_) | Scheme::Ftp(_) | Scheme::Ldap(_) => {
                let mut auth_lexer: Lexer<'uri, AuthorityToken<'uri>> = lexer.morph();
            },
            _ => {
                return Err(UriError::Scheme(SchemeError::Unimplemented(scheme)));
            },
        }

        let scheme_data: crate::SchemeData<'uri> = crate::SchemeData { raw: None };
        
        //scheme::data::scheme_morph(&scheme, &mut lexer)
        //    .map_err(|e| UriError::Scheme(SchemeError::SchemeSelect(e)))?;
        
        //let scheme_data = scheme::data::parse_scheme_data(&mut scheme_lexer, &mut authority)
        //    .map_err(|e| UriError::SchemeData(e))?;
        
        Ok ( Uri { scheme, authority, scheme_data } )
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_https_host_only() {
        let uri = "https://foobar.test";
        let res: Uri<'static> = uri.try_into().unwrap();
        assert_eq!(res.scheme, Scheme::Http(true));
    }
    
}
