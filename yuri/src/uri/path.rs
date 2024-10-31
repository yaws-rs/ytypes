//! URI Path types

/// Path
#[derive(Clone, Debug, PartialEq)]
pub struct Path<'uri> {
    /// Raw path
    pub raw_path: &'uri str,
}
