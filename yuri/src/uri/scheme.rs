//! URI Scheme Types

/// URI Scheme
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Scheme<'uri> {
    /// HTTP RFC x
    Http(SchemeTls),
    /// FTP RFC x
    Ftp(SchemeTls),
    /// LDAP RFC x
    Ldap(SchemeTls),
    /// Telnet RFC X
    Telnet,
    /// URN RFC X
    Urn,
    /// Unknwon scheme
    Unknown(&'uri str),
}

/// Whether the Scheme indicates explicit TLS
pub type SchemeTls = bool;

use core::str::FromStr;

use crate::error::SchemeError;

impl<'uri> TryFrom<&'uri str> for Scheme<'uri> {
    type Error = SchemeError<'uri>;
    fn try_from(raw: &'uri str) -> Result<Self, Self::Error> {
        match raw {
            "http" => Ok(Self::Http(false)),
            "https" => Ok(Self::Http(true)),
            _ => Err(SchemeError::Invalid),
        }
    }
}

use core::fmt;
use core::fmt::Display;

impl<'uri> Display for Scheme<'uri> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(false) => write!(f, "http"),
            Self::Http(true) => write!(f, "https"),
            Self::Ftp(false) => write!(f, "ftp"),
            Self::Ftp(true) => write!(f, "ftps"),
            Self::Ldap(false) => write!(f, "ldap"),
            Self::Ldap(true) => write!(f, "ldaps"),
            Self::Telnet => write!(f, "telnet"),
            Self::Urn => write!(f, "urn"),
            Self::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn http_from_str() {
        let s: Scheme<'static> = "http".try_into().unwrap();
        assert_eq!(s, Scheme::Http(false));
    }
    #[test]
    fn https_from_str() {
        let s: Scheme<'static> = "https".try_into().unwrap();
        assert_eq!(s, Scheme::Http(true));
    }
    #[test]
    fn https_to_str() {
        let s = format!("{}", Scheme::Http(true));
        assert_eq!(s, "https");
    }
    #[test]
    fn http_to_str() {
        let s = format!("{}", Scheme::Http(false));
        assert_eq!(s, "http");
    }
}
