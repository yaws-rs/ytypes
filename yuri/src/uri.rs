//! Jri Public types

/// URI Scheme
#[non_exhaustive]
pub enum Scheme<'uri> {
    Http,
    Https,
    Ftp,
    Ldap,
    Telnet,
    Urn,
    Unknown(&'uri str),
}

pub enum ParseBehaviours {
    /// (default) Strict RFC compliance
    Strict,
    /// Handle User:Password in Userinfo
    /// RFC 3986 s. 3.2.1. deprecates user:pass
    /// Default is to Reject
    /// Application should reject the storage of such data in unencrypted form
    /// The passing of authentication information in clear text has proven to be
    /// a security risk in almost every case where it has been used.    
    DeprecatedPassword(AllowOrIgnore),
}

pub enum AllowOrIgnore {
    Allow,
    Ignore,
}
