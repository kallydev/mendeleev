#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

#[cfg(feature = "std")]
extern crate std;

mod element;
mod isotope;
mod newtypes;
mod properties;
mod superscript;

pub use element::*;
pub use isotope::*;
pub use newtypes::*;
pub use properties::*;

#[cfg(feature = "group")]
mod group;
#[cfg(feature = "group")]
pub use group::*;
