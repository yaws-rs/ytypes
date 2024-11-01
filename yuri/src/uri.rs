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
