//! Error types

#[derive(Clone, Debug, PartialEq)]
pub enum UriError<'uri> {
    Scheme(SchemeError<'uri>),
    SchemeData(SchemeDataError<'uri>),
}

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
    /// Unimplemented
    Unimplemented(crate::Scheme<'uri>),
    /// Expected Scheme, got nothing
    Nothing,
    /// Expected : separator but did not find it
    RunAway,
    /// Parsing error with detail
    ParsingDetailed(ParsingDetail<'uri>),
}

/// Authority releated errors
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorityError<'uri> {
    /// Expected @ separator but did not find it
    RunAway,
    /// Nothing seen - expected host / authority
    ParsedNothing,
    /// Invalid Authority portition
    InvalidAuthority,
    /// Parsing error with detail
    ParsingDetailed(ParsingDetail<'uri>),    
}

/// Scheme date related errors
#[derive(Clone, Debug, PartialEq)]
pub enum SchemeDataError<'uri> {
    /// Parsing error with detail
    ParsingDetailed(ParsingDetail<'uri>),    
}
