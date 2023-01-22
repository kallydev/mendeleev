//! Mendeleev is a crate containing all known chemical elements as an enum
//! and as a list, as well as methods that return some properties for each
//! of them.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

#[cfg(feature = "std")]
extern crate std;

mod element;
mod isotope;
mod properties;
mod superscript;

pub use element::*;
pub use isotope::*;
pub use properties::*;

#[cfg(feature = "group")]
mod group;
#[cfg(feature = "group")]
pub use group::*;
