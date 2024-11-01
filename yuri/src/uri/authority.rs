//! URI Authority types

/// Authority
#[derive(Clone, Debug, PartialEq)]
pub struct Authority<'uri> {
    /// Userinfo
    pub userinfo: Option<UserInfo<'uri>>,
    /// Raw unparsed host
    pub raw_host: &'uri str,
    /// Port if supplied, otherwise default by scheme
    pub port: Option<u16>,
}

use crate::error::AuthorityError;

impl<'uri> Authority<'uri> {
    /// Construct new with host
    #[cfg(feature = "builder")]
    pub fn from_host_str(host: &'uri str) -> Result<Self, AuthorityError<'uri>> {
        Ok(Self {
            userinfo: None,
            raw_host: host,
            port: None,
        })
    }
}

/// Authority userinfo
#[derive(Clone, Debug, PartialEq)]
pub struct UserInfo<'uri> {
    /// Raw user part
    pub raw_user: &'uri str,
    /// Raw authorisation part
    /// **Warning** claertext password is deprecated & insecure
    pub(crate) raw_authorization: Option<&'uri str>,
}

impl<'uri> UserInfo<'uri> {
    /// Get raw authorization part
    /// **NOTE**: RFC has deprecated this field for cleartext passwords which are insecure.
    pub fn raw_authorization(&self) -> Option<&'uri str> {
        self.raw_authorization
    }
    /// Set raw authorization part
    /// **NOTE**: RFC has deprecated this field for cleartext passwords which are insecure.
    #[cfg(feature = "builder")]
    pub fn set_authorization(&mut self, user: &'uri str, auth: Option<&'uri str>) {
        self.raw_user = user;
        self.raw_authorization = auth;
    }
}
