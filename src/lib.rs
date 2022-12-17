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

pub use element::*;
pub use isotope::*;
pub use properties::*;

#[cfg(feature = "group")]
mod group;
#[cfg(feature = "group")]
pub use group::*;

#[cfg(feature = "std")]
use std::string::{String, ToString};

#[cfg(feature = "std")]
pub(crate) fn to_superscript(number: u32) -> String {
    number
        .to_string()
        .chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            c => c,
        })
        .collect()
}
