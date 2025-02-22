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
newtype_f64!(KiloJoulePerMole, "kilojoules per mole", "kJ/mol");
newtype_f64!(Kelvin, "Kelvin", "K");
newtype_f64!(
    GramPerCubicCentimeter,
    "grams per cubic centimeter",
    "g/cm³"
);
newtype_f64!(Electronvolt, "electronvolts", "eV");

/// A unit of time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeUnit {
    /// 10^-24 seconds
    Yoctosecond,

    /// 10^-21 seconds
    Zeptosecond,

    /// 10^-18 seconds
    Attosecond,

    /// 10^-15 seconds
    Femtosecond,

    /// 10^-12 seconds
    Picosecond,

    /// 10^-9 seconds
    Nanosecond,

    /// 10^-6 seconds
    Microsecond,

    /// 10^-3 seconds
    Millisecond,

    /// SI standard second
    Second,

    /// 60 seconds
    Minute,

    /// 60 minutes
    Hour,

    /// 24 hours
    Day,

    /// 365 days
    Year,

    /// 10^3 years
    Kiloyear,

    /// 10^6 years
    Megayear,

    /// 10^9 years
    Gigayear,

    /// 10^12 years
    Terayear,

    /// 10^15 years
    Petayear,

    /// 10^18 years
    Exayear,

    /// 10^21 years
    Zettayear,

    /// 10^24 years
    Yottayear,
}

impl TimeUnit {
    /// The value of this unit in seconds
    ///
    /// # Examples
    ///
    /// ```
    /// use mendeleev::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Second.seconds(), 1.0);
    /// assert_eq!(TimeUnit::Minute.seconds(), 60.0);
    /// assert_eq!(TimeUnit::Hour.seconds(), 60.0 * TimeUnit::Minute.seconds());
    /// assert_eq!(TimeUnit::Day.seconds(), 24.0 * TimeUnit::Hour.seconds());
    /// assert_eq!(TimeUnit::Year.seconds(), 365.0 * TimeUnit::Day.seconds());
    /// ```
    pub const fn seconds(&self) -> f64 {
        match self {
            Self::Yoctosecond => 1e-24,
            Self::Zeptosecond => 1e-21,
            Self::Attosecond => 1e-18,
            Self::Femtosecond => 1e-18,
            Self::Picosecond => 1e-18,
            Self::Nanosecond => 1e-9,
            Self::Microsecond => 1e-6,
            Self::Millisecond => 1e-3,
            Self::Second => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3600.0,
            Self::Day => 86400.0,
            Self::Year => 31536000.0,
            Self::Kiloyear => 31536000e3,
            Self::Megayear => 31536000e6,
            Self::Gigayear => 31536000e9,
            Self::Terayear => 31536000e12,
            Self::Petayear => 31536000e15,
            Self::Exayear => 31536000e18,
            Self::Zettayear => 31536000e21,
            Self::Yottayear => 31536000e24,
        }
    }

    /// The SI abbreviation for this unit
    ///
    /// # Examples
    ///
    /// ```
    /// use mendeleev::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Microsecond.abbreviation(), "µs");
    /// assert_eq!(TimeUnit::Second.abbreviation(), "s");
    /// assert_eq!(TimeUnit::Minute.abbreviation(), "min");
    /// assert_eq!(TimeUnit::Hour.abbreviation(), "h");
    /// assert_eq!(TimeUnit::Day.abbreviation(), "d");
    /// assert_eq!(TimeUnit::Year.abbreviation(), "a");
    /// assert_eq!(TimeUnit::Megayear.abbreviation(), "Ma");
    /// ```
    pub const fn abbreviation(&self) -> &'static str {
        match self {
            Self::Yoctosecond => "ys",
            Self::Zeptosecond => "zs",
            Self::Attosecond => "as",
            Self::Femtosecond => "fs",
            Self::Picosecond => "ps",
            Self::Nanosecond => "ns",
            Self::Microsecond => "µs",
            Self::Millisecond => "ms",
            Self::Second => "s",
            Self::Minute => "min",
            Self::Hour => "h",
            Self::Day => "d",
            Self::Year => "a",
            Self::Kiloyear => "ka",
            Self::Megayear => "Ma",
            Self::Gigayear => "Ga",
            Self::Terayear => "Ta",
            Self::Petayear => "Pa",
            Self::Exayear => "Ea",
            Self::Zettayear => "Za",
            Self::Yottayear => "Ya",
        }
    }
}
