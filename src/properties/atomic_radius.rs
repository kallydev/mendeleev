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
        use Element as E;
        match self {
            E::H => pm!(25.0),
            E::He => pm!(120.0),
            E::Li => pm!(145.0),
            E::Be => pm!(105.0),
            E::B => pm!(85.0),
            E::C => pm!(70.0),
            E::N => pm!(65.0),
            E::O => pm!(60.0),
            E::F => pm!(50.0),
            E::Ne => pm!(160.0),
            E::Na => pm!(180.0),
            E::Mg => pm!(150.0),
            E::Al => pm!(125.0),
            E::Si => pm!(110.0),
            E::P => pm!(100.0),
            E::S => pm!(100.0),
            E::Cl => pm!(100.0),
            E::Ar => pm!(71.0),
            E::K => pm!(220.0),
            E::Ca => pm!(180.0),
            E::Sc => pm!(160.0),
            E::Ti => pm!(140.0),
            E::V => pm!(135.0),
            E::Cr => pm!(140.0),
            E::Mn => pm!(140.0),
            E::Fe => pm!(140.0),
            E::Co => pm!(135.0),
            E::Ni => pm!(135.0),
            E::Cu => pm!(135.0),
            E::Zn => pm!(135.0),
            E::Ga => pm!(130.0),
            E::Ge => pm!(125.0),
            E::As => pm!(115.0),
            E::Se => pm!(115.0),
            E::Br => pm!(115.0),
            E::Kr => None,
            E::Rb => pm!(235.0),
            E::Sr => pm!(200.0),
            E::Y => pm!(180.0),
            E::Zr => pm!(155.0),
            E::Nb => pm!(145.0),
            E::Mo => pm!(145.0),
            E::Tc => pm!(135.0),
            E::Ru => pm!(130.0),
            E::Rh => pm!(135.0),
            E::Pd => pm!(140.0),
            E::Ag => pm!(160.0),
            E::Cd => pm!(155.0),
            E::In => pm!(155.0),
            E::Sn => pm!(145.0),
            E::Sb => pm!(145.0),
            E::Te => pm!(140.0),
            E::I => pm!(140.0),
            E::Xe => None,
            E::Cs => pm!(260.0),
            E::Ba => pm!(215.0),
            E::La => pm!(195.0),
            E::Ce => pm!(185.0),
            E::Pr => pm!(185.0),
            E::Nd => pm!(185.0),
            E::Pm => pm!(185.0),
            E::Sm => pm!(185.0),
            E::Eu => pm!(185.0),
            E::Gd => pm!(180.0),
            E::Tb => pm!(175.0),
            E::Dy => pm!(175.0),
            E::Ho => pm!(175.0),
            E::Er => pm!(175.0),
            E::Tm => pm!(175.0),
            E::Yb => pm!(175.0),
            E::Lu => pm!(175.0),
            E::Hf => pm!(155.0),
            E::Ta => pm!(145.0),
            E::W => pm!(135.0),
            E::Re => pm!(135.0),
            E::Os => pm!(130.0),
            E::Ir => pm!(135.0),
            E::Pt => pm!(135.0),
            E::Au => pm!(135.0),
            E::Hg => pm!(150.0),
            E::Tl => pm!(190.0),
            E::Pb => pm!(180.0),
            E::Bi => pm!(160.0),
            E::Po => pm!(190.0),
            E::At => None,
            E::Rn => None,
            E::Fr => None,
            E::Ra => pm!(215.0),
            E::Ac => pm!(195.0),
            E::Th => pm!(180.0),
            E::Pa => pm!(180.0),
            E::U => pm!(175.0),
            E::Np => pm!(175.0),
            E::Pu => pm!(175.0),
            E::Am => pm!(175.0),
            E::Cm => None,
            E::Bk => None,
            E::Cf => None,
            E::Es => None,
            E::Fm => None,
            E::Md => None,
            E::No => None,
            E::Lr => None,
            E::Rf => None,
            E::Db => None,
            E::Sg => None,
            E::Bh => None,
            E::Hs => None,
            E::Mt => None,
            E::Ds => None,
            E::Rg => None,
            E::Cn => None,
            E::Nh => None,
            E::Fl => None,
            E::Mc => None,
            E::Lv => None,
            E::Ts => None,
            E::Og => None,
        }
    }
}
