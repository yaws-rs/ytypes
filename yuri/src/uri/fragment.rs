//! URI Fragment types

/// Fragment
#[derive(Clone, Debug, PartialEq)]
pub struct Fragment<'uri> {
    /// Raw fragment
    pub raw_fragment: &'uri str,
}
