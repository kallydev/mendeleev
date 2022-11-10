//! Mendeleev is a crate containing all known chemical elements as an enum
//! and as a list, as well as methods that return some properties for each
//! of them.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

mod element;
mod isotope;
mod properties;

pub use element::*;
pub use isotope::*;
pub use properties::*;

#[cfg(feature = "group")]
mod group;
#[cfg(feature = "group")]
pub use group::*;
