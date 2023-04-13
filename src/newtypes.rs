use core::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/// Create a type containing a single f64 value.
///
/// The type implements Display and basic arithmetic operations.
/// This is meant for creating quantity types to be returned by the element/isotope properties, so
/// there is no ambiguity about which unit is used.
macro_rules! newtype_f64 {
    ($name:ident, $plural:literal, $abbrev:literal) => {
        #[doc = concat!("A value in ", $plural, " (", $abbrev, ").")]
        #[doc = ""]
        #[doc = "Basic arithmetic is implemented for convenience, and the underlying f64 value is"]
        #[doc = "accessible as a tuple field."]
        #[doc = ""]
        #[doc = "# Example"]
        #[doc = "```"]
        #[doc = concat!("use mendeleev::", stringify!($name), ";")]
        #[doc = "use core::cmp::Ordering;"]
        #[doc = ""]
        #[doc = concat!("let value1 = ", stringify!($name), "(1.0);")]
        #[doc = concat!("let value2 = ", stringify!($name), "(2.0);")]
        #[doc = "assert_eq!(value1.0, 1.0);"]
        #[doc = "assert!(value1 < value2);"]
        #[doc = "assert_eq!(value1.total_cmp(&value2), Ordering::Less);"]
        #[doc = concat!("assert_eq!(value1 + value2, ", stringify!($name), "(3.0));")]
        #[doc = concat!("assert_eq!(value1 - value2, ", stringify!($name), "(-1.0));")]
        #[doc = concat!("assert_eq!(value1 * 5.0, ", stringify!($name), "(5.0));")]
        #[doc = concat!("assert_eq!(value1 / 4.0, ", stringify!($name), "(0.25));")]
        #[doc = "```"]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name(pub f64);

        impl $name {
            /// Return the ordering between `self` and `other`.
            ///
            /// This simply calls [`f64::total_cmp`] on the inner value.
            pub fn total_cmp(&self, other: &Self) -> Ordering {
                self.0.total_cmp(&other.0)
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl Mul<f64> for $name {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl Div<f64> for $name {
            type Output = Self;

            fn div(self, rhs: f64) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl Display for $name {
            /// Displays the value along with the unit. Supports numeric formatting.
            ///
            /// A space is added between the number and the unit, in accordance with the SI
            /// convention.
            #[doc = "# Example"]
            #[doc = "```"]
            #[doc = concat!("use mendeleev::", stringify!($name), ";")]
            #[doc = ""]
            #[doc = concat!("let value = ", stringify!($name), "(1.2345);")]
            #[doc = concat!("assert_eq!(format!(\"{value:.1}\"), \"1.2 ", $abbrev, "\");")]
            #[doc = "```"]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)?;
                f.write_str(" ")?;
                f.write_str($abbrev)
            }
        }
    };
}

newtype_f64!(Percent, "percent", "%");
newtype_f64!(Picometer, "picometers", "pm");
newtype_f64!(KiloJoulePerMol, "kilojoules per mol", "kJ/mol");
newtype_f64!(Kelvin, "Kelvin", "K");
