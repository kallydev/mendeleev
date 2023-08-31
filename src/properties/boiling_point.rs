use super::Element;
use crate::Kelvin;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum boiling point across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, BOILING_POINT_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.boiling_point());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, BOILING_POINT_RANGE);
/// ```
pub const BOILING_POINT_RANGE: core::ops::RangeInclusive<Kelvin> = Kelvin(4.216)..=Kelvin(5930.0);

macro_rules! kel {
    ($value:literal) => {
        Some(Kelvin($value))
    };
}

impl Element {
    /// Returns the element's boiling point, if known.
    ///
    /// ```
    /// use mendeleev::{Element, Kelvin};
    /// assert_eq!(Element::H.boiling_point(), Some(Kelvin(20.28)));
    /// assert_eq!(Element::C.boiling_point(), Some(Kelvin(5100.0)));
    /// assert_eq!(Element::Og.boiling_point(), None);
    /// ```
    pub const fn boiling_point(&self) -> Option<Kelvin> {
        use Element as E;
        match self {
            E::H => kel!(20.28),
            E::He => kel!(4.216),
            E::Li => kel!(1118.15),
            E::Be => kel!(3243.0),
            E::B => kel!(3931.0),
            E::C => kel!(5100.0),
            E::N => kel!(77.4),
            E::O => kel!(90.19),
            E::F => kel!(85.01),
            E::Ne => kel!(27.1),
            E::Na => kel!(1156.1),
            E::Mg => kel!(1363.0),
            E::Al => kel!(2740.0),
            E::Si => kel!(2628.0),
            E::P => kel!(553.0),
            E::S => kel!(717.824),
            E::Cl => kel!(238.6),
            E::Ar => kel!(87.3),
            E::K => kel!(1047.0),
            E::Ca => kel!(1757.0),
            E::Sc => kel!(3104.0),
            E::Ti => kel!(3560.0),
            E::V => kel!(3650.0),
            E::Cr => kel!(2945.0),
            E::Mn => kel!(2235.0),
            E::Fe => kel!(3023.0),
            E::Co => kel!(3143.0),
            E::Ni => kel!(3005.0),
            E::Cu => kel!(2840.0),
            E::Zn => kel!(1180.0),
            E::Ga => kel!(2676.0),
            E::Ge => kel!(3103.0),
            E::As => kel!(876.0),
            E::Se => kel!(958.1),
            E::Br => kel!(331.9),
            E::Kr => kel!(120.85),
            E::Rb => kel!(961.0),
            E::Sr => kel!(1657.0),
            E::Y => kel!(3611.0),
            E::Zr => kel!(4650.0),
            E::Nb => kel!(5015.0),
            E::Mo => kel!(4885.0),
            E::Tc => kel!(5150.0),
            E::Ru => kel!(4173.0),
            E::Rh => kel!(4000.0),
            E::Pd => kel!(3413.0),
            E::Ag => kel!(2485.0),
            E::Cd => kel!(1038.0),
            E::In => kel!(2353.0),
            E::Sn => kel!(2543.0),
            E::Sb => kel!(1908.0),
            E::Te => kel!(1263.0),
            E::I => kel!(457.5),
            E::Xe => kel!(166.1),
            E::Cs => kel!(951.6),
            E::Ba => kel!(1910.0),
            E::La => kel!(3730.0),
            E::Ce => kel!(3699.0),
            E::Pr => kel!(3785.0),
            E::Nd => kel!(3341.0),
            E::Pm => kel!(3000.0),
            E::Sm => kel!(2064.0),
            E::Eu => kel!(1870.0),
            E::Gd => kel!(3539.0),
            E::Tb => kel!(3296.0),
            E::Dy => kel!(2835.0),
            E::Ho => kel!(2968.0),
            E::Er => kel!(3136.0),
            E::Tm => kel!(2220.0),
            E::Yb => kel!(1466.0),
            E::Lu => kel!(3668.0),
            E::Hf => kel!(5470.0),
            E::Ta => kel!(5698.0),
            E::W => kel!(5930.0),
            E::Re => kel!(5900.0),
            E::Os => kel!(5300.0),
            E::Ir => kel!(4403.0),
            E::Pt => kel!(4100.0),
            E::Au => kel!(3080.0),
            E::Hg => kel!(629.73),
            E::Tl => kel!(1730.0),
            E::Pb => kel!(2013.0),
            E::Bi => kel!(1883.0),
            E::Po => kel!(1235.0),
            E::At => kel!(610.0),
            E::Rn => kel!(211.4),
            E::Fr => kel!(950.0),
            E::Ra => kel!(1413.0),
            E::Ac => kel!(3470.0),
            E::Th => kel!(5060.0),
            E::Pa => kel!(4300.0),
            E::U => kel!(4018.0),
            E::Np => kel!(4175.0),
            E::Pu => kel!(3505.0),
            E::Am => kel!(2880.0),
            E::Cm => None,
            E::Bk => None,
            E::Cf => None,
            E::Es => kel!(1130.0),
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
