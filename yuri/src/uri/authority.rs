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

/// Authority userinfo
#[derive(Clone, Debug, PartialEq)]
pub struct UserInfo<'uri> {
    /// Raw user part
    pub raw_user: &'uri str,
    /// Raw authorisation part
    /// **Warning** claertext password is deprecated & insecure
    pub(crate) raw_authorization: Option<&'uri str>,
}
