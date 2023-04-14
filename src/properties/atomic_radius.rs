use super::Element;
use crate::Picometer;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum atomic radius across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, ATOMIC_RADIUS_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.atomic_radius());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, ATOMIC_RADIUS_RANGE);
/// ```
pub const ATOMIC_RADIUS_RANGE: core::ops::RangeInclusive<Picometer> =
    Picometer(25.0)..=Picometer(260.0);

macro_rules! pm {
    ($value:literal) => {
        Some(Picometer($value))
    };
}

impl Element {
    /// Returns the element's empirically measured atomic radius, if available.
    ///
    /// ```
    /// use mendeleev::{Element, Picometer};
    /// assert_eq!(Element::H.atomic_radius(), Some(Picometer(25.0)));
    /// ```
    pub const fn atomic_radius(&self) -> Option<Picometer> {
        use Element::*;
        match self {
            H => pm!(25.0),
            He => pm!(120.0),
            Li => pm!(145.0),
            Be => pm!(105.0),
            B => pm!(85.0),
            C => pm!(70.0),
            N => pm!(65.0),
            O => pm!(60.0),
            F => pm!(50.0),
            Ne => pm!(160.0),
            Na => pm!(180.0),
            Mg => pm!(150.0),
            Al => pm!(125.0),
            Si => pm!(110.0),
            P => pm!(100.0),
            S => pm!(100.0),
            Cl => pm!(100.0),
            Ar => pm!(71.0),
            K => pm!(220.0),
            Ca => pm!(180.0),
            Sc => pm!(160.0),
            Ti => pm!(140.0),
            V => pm!(135.0),
            Cr => pm!(140.0),
            Mn => pm!(140.0),
            Fe => pm!(140.0),
            Co => pm!(135.0),
            Ni => pm!(135.0),
            Cu => pm!(135.0),
            Zn => pm!(135.0),
            Ga => pm!(130.0),
            Ge => pm!(125.0),
            As => pm!(115.0),
            Se => pm!(115.0),
            Br => pm!(115.0),
            Kr => None,
            Rb => pm!(235.0),
            Sr => pm!(200.0),
            Y => pm!(180.0),
            Zr => pm!(155.0),
            Nb => pm!(145.0),
            Mo => pm!(145.0),
            Tc => pm!(135.0),
            Ru => pm!(130.0),
            Rh => pm!(135.0),
            Pd => pm!(140.0),
            Ag => pm!(160.0),
            Cd => pm!(155.0),
            In => pm!(155.0),
            Sn => pm!(145.0),
            Sb => pm!(145.0),
            Te => pm!(140.0),
            I => pm!(140.0),
            Xe => None,
            Cs => pm!(260.0),
            Ba => pm!(215.0),
            La => pm!(195.0),
            Ce => pm!(185.0),
            Pr => pm!(185.0),
            Nd => pm!(185.0),
            Pm => pm!(185.0),
            Sm => pm!(185.0),
            Eu => pm!(185.0),
            Gd => pm!(180.0),
            Tb => pm!(175.0),
            Dy => pm!(175.0),
            Ho => pm!(175.0),
            Er => pm!(175.0),
            Tm => pm!(175.0),
            Yb => pm!(175.0),
            Lu => pm!(175.0),
            Hf => pm!(155.0),
            Ta => pm!(145.0),
            W => pm!(135.0),
            Re => pm!(135.0),
            Os => pm!(130.0),
            Ir => pm!(135.0),
            Pt => pm!(135.0),
            Au => pm!(135.0),
            Hg => pm!(150.0),
            Tl => pm!(190.0),
            Pb => pm!(180.0),
            Bi => pm!(160.0),
            Po => pm!(190.0),
            At => None,
            Rn => None,
            Fr => None,
            Ra => pm!(215.0),
            Ac => pm!(195.0),
            Th => pm!(180.0),
            Pa => pm!(180.0),
            U => pm!(175.0),
            Np => pm!(175.0),
            Pu => pm!(175.0),
            Am => pm!(175.0),
            Cm => None,
            Bk => None,
            Cf => None,
            Es => None,
            Fm => None,
            Md => None,
            No => None,
            Lr => None,
            Rf => None,
            Db => None,
            Sg => None,
            Bh => None,
            Hs => None,
            Mt => None,
            Ds => None,
            Rg => None,
            Cn => None,
            Nh => None,
            Fl => None,
            Mc => None,
            Lv => None,
            Ts => None,
            Og => None,
        }
    }
}
