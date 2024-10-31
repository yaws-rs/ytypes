//! Jri Public types

pub mod scheme;
pub use scheme::*;
pub mod authority;
pub use authority::*;
pub mod path;
pub use path::*;
pub mod query;
pub use query::*;
pub mod fragment;
pub use fragment::*;

/// URI
pub struct Uri<'uri> {
    /// Scheme
    pub scheme: Scheme<'uri>,
    /// Authority
    pub authority: Option<Authority<'uri>>,
    /// Path
    pub path: Option<Path<'uri>>,
    /// Query
    pub query: Option<Query<'uri>>,
    /// Fragment
    pub fragment: Option<Fragment<'uri>>,
    /// Scheme based data
    pub scheme_data: SchemeData<'uri>,
}

/// By default parsing is strict RFC
/// Some behaviours can be overriden
pub enum ParseOverrides {
    /// Handle User:Password in Userinfo
    /// RFC 3986 s. 3.2.1. deprecates user:pass
    /// Default is to Reject
    /// Application should reject the storage of such data in unencrypted form
    /// The passing of authentication information in clear text has proven to be
    /// a security risk in almost every case where it has been used.    
    DeprecatedPassword(AllowOrIgnore),
}

/// Allow or ignore type for type storage
pub enum AllowOrIgnore {
    /// Allow to store value
    Allow,
    /// Ignore and don't store
    Ignore,
}
