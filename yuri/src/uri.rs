//! Jri Public types

mod scheme;
pub use scheme::*;
mod authority;
pub use authority::*;
mod path;
pub use path::*;
mod query;
pub use query::*;
mod fragment;
pub use fragment::*;

/// URI
#[derive(Debug, Clone, PartialEq)]
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

use crate::error::UriError;

impl<'uri> Uri<'uri> {
    /// Construct a new URI from &str
    pub fn new(input: &'uri str) -> Result<Self, UriError<'uri>> {
        input.try_into()
    }
}
