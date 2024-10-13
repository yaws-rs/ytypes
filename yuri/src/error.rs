//! Error types

/// Parsing Detail relating to an Error
#[derive(Clone, Debug, PartialEq)]
pub struct ParsingDetail<'uri> {
    /// Component
    pub component: &'static str,
    /// Span start
    pub span_start: usize,
    /// Span end
    pub span_end: usize,
    /// Source
    pub source: &'uri str,
    /// Clipped span
    pub clipped_span: &'uri str,
    /// Clipped remaining
    pub clipped_remaining: &'uri str,
}

/// Scheme related errors
#[derive(Clone, Debug, PartialEq)]
pub enum SchemeError<'uri> {
    /// Invalid Scheme given
    Invalid,
    /// Expected Scheme, got nothing
    Nothing,
    /// Expected : separator but did not find it
    RunAway,
    /// Parsing error with detail
    ParsingDetailed(ParsingDetail<'uri>),
}
