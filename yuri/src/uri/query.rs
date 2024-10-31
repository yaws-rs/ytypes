//! URI Query types

/// Query
#[derive(Clone, Debug, PartialEq)]
pub struct Query<'uri> {
    /// Raw query
    pub raw_query: &'uri str,
}
