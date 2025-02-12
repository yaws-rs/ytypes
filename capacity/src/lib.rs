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

use core::marker::PhantomData;
#[cfg(feature = "bounded")]
use core::{fmt, fmt::Display};

/// Bounded Maximums & Minimums for each Kind
#[cfg(feature = "bounded")]
pub trait Bounded<Kind> {
    /// Return the Maximum that can be set for the given Kind
    fn maximum(&self, _: &Kind) -> usize;
    /// Return the Minimum that can be set for the given Kind    
    fn minimum(&self, _: &Kind) -> usize;
}

/// Base trait for setting the capacity
pub trait Setting<Kind> {
    /// Set the capacities for the given Kind
    fn setting(&self, _: &Kind) -> usize;
}

/// Capacity Planner
pub struct Capacity<H, Kind> {
    holder: H,
    _kind: PhantomData<Kind>,
}

/// Capacity indidator when capacity setting is bounded.
#[derive(Clone, Debug, PartialEq)]
#[cfg(feature = "bounded")]
pub enum CapacityIndicator {
    /// Over the maximum
    OverMax(usize),
    /// Under the minimum
    UnderMin(usize),
    /// Capacity Within Bounds
    WithinBounds(usize),
}

#[cfg(feature = "bounded")]
impl Display for CapacityIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OverMax(max) => write!(f, "Over {}", max),
            Self::UnderMin(min) => write!(f, "Under {}", min),
            Self::WithinBounds(cap) => write!(f, "Capacity {}", cap),
        }
    }
}

impl<H, Kind> Capacity<H, Kind>
where
    H: Setting<Kind>,
{
    /// Create new capacity planning.
    #[inline]
    pub fn with_planned(holder: H) -> Self {
        Self {
            holder,
            _kind: PhantomData,
        }
    }
    /// Returns the current setting without bounds.
    #[inline(always)]
    pub fn of_unbounded(&self, of: &Kind) -> usize {
        self.holder.setting(of)
    }
}

#[cfg(feature = "bounded")]
impl<H, Kind> Capacity<H, Kind>
where
    H: Setting<Kind> + Bounded<Kind>,
{
    /// Minimum set by Kind
    #[inline(always)]
    pub fn minimum(&self, of: &Kind) -> usize {
        self.holder.minimum(of)
    }
    /// Maximum set by Kind    
    #[inline(always)]
    pub fn maximum(&self, of: &Kind) -> usize {
        self.holder.maximum(of)
    }
    /// Returns the current setting and if within the set bounds.
    #[inline(always)]
    pub fn of_bounded(&self, of: &Kind) -> CapacityIndicator {
        if self.holder.minimum(of) > self.holder.setting(of) {
            return CapacityIndicator::UnderMin(self.holder.minimum(of));
        }
        if self.holder.maximum(of) < self.holder.setting(of) {
            return CapacityIndicator::OverMax(self.holder.maximum(of));
        }
        CapacityIndicator::WithinBounds(self.holder.setting(of))
    }
}
