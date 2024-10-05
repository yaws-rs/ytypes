//! Percent encoding

// pct-encoded = "%" HEXDIG HEXDIG

//      reserved    = gen-delims / sub-delims
//
//      gen-delims  = ":" / "/" / "?" / "#" / "[" / "]" / "@"
//
//      sub-delims  = "!" / "$" / "&" / "'" / "(" / ")"
//                  / "*" / "+" / "," / ";" / "="

//   unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"
//

// For consistency, percent-encoded octets in the ranges of ALPHA
//   (%41-%5A and %61-%7A), DIGIT (%30-%39), hyphen (%2D), period (%2E),
//   underscore (%5F), or tilde (%7E) should not be created by URI
//   producers and, when found in a URI, should be decoded to their
//   corresponding unreserved characters by URI normalizers.

