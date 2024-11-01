#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(clippy::single_match, rustdoc::bare_urls)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

//---------------------------------------------------------
// Re-exports on external types we may use
//---------------------------------------------------------

//---------------------------------------------------------
// Error types
//---------------------------------------------------------

pub mod error;
pub(crate) use error::*;

//---------------------------------------------------------
// uri types
//---------------------------------------------------------

mod uri;
pub use uri::*;

//--------------------------------------------------------
// Parsing implementations with type conversions
//--------------------------------------------------------

pub(crate) mod parser;

//--------------------------------------------------------
// Builder implementations with type conversions
//--------------------------------------------------------

pub(crate) mod builder;
