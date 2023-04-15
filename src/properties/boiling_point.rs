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
        use Element::*;
        match self {
            H => kel!(20.28),
            He => kel!(4.216),
            Li => kel!(1118.15),
            Be => kel!(3243.0),
            B => kel!(3931.0),
            C => kel!(5100.0),
            N => kel!(77.4),
            O => kel!(90.19),
            F => kel!(85.01),
            Ne => kel!(27.1),
            Na => kel!(1156.1),
            Mg => kel!(1363.0),
            Al => kel!(2740.0),
            Si => kel!(2628.0),
            P => kel!(553.0),
            S => kel!(717.824),
            Cl => kel!(238.6),
            Ar => kel!(87.3),
            K => kel!(1047.0),
            Ca => kel!(1757.0),
            Sc => kel!(3104.0),
            Ti => kel!(3560.0),
            V => kel!(3650.0),
            Cr => kel!(2945.0),
            Mn => kel!(2235.0),
            Fe => kel!(3023.0),
            Co => kel!(3143.0),
            Ni => kel!(3005.0),
            Cu => kel!(2840.0),
            Zn => kel!(1180.0),
            Ga => kel!(2676.0),
            Ge => kel!(3103.0),
            As => kel!(876.0),
            Se => kel!(958.1),
            Br => kel!(331.9),
            Kr => kel!(120.85),
            Rb => kel!(961.0),
            Sr => kel!(1657.0),
            Y => kel!(3611.0),
            Zr => kel!(4650.0),
            Nb => kel!(5015.0),
            Mo => kel!(4885.0),
            Tc => kel!(5150.0),
            Ru => kel!(4173.0),
            Rh => kel!(4000.0),
            Pd => kel!(3413.0),
            Ag => kel!(2485.0),
            Cd => kel!(1038.0),
            In => kel!(2353.0),
            Sn => kel!(2543.0),
            Sb => kel!(1908.0),
            Te => kel!(1263.0),
            I => kel!(457.5),
            Xe => kel!(166.1),
            Cs => kel!(951.6),
            Ba => kel!(1910.0),
            La => kel!(3730.0),
            Ce => kel!(3699.0),
            Pr => kel!(3785.0),
            Nd => kel!(3341.0),
            Pm => kel!(3000.0),
            Sm => kel!(2064.0),
            Eu => kel!(1870.0),
            Gd => kel!(3539.0),
            Tb => kel!(3296.0),
            Dy => kel!(2835.0),
            Ho => kel!(2968.0),
            Er => kel!(3136.0),
            Tm => kel!(2220.0),
            Yb => kel!(1466.0),
            Lu => kel!(3668.0),
            Hf => kel!(5470.0),
            Ta => kel!(5698.0),
            W => kel!(5930.0),
            Re => kel!(5900.0),
            Os => kel!(5300.0),
            Ir => kel!(4403.0),
            Pt => kel!(4100.0),
            Au => kel!(3080.0),
            Hg => kel!(629.73),
            Tl => kel!(1730.0),
            Pb => kel!(2013.0),
            Bi => kel!(1883.0),
            Po => kel!(1235.0),
            At => kel!(610.0),
            Rn => kel!(211.4),
            Fr => kel!(950.0),
            Ra => kel!(1413.0),
            Ac => kel!(3470.0),
            Th => kel!(5060.0),
            Pa => kel!(4300.0),
            U => kel!(4018.0),
            Np => kel!(4175.0),
            Pu => kel!(3505.0),
            Am => kel!(2880.0),
            Cm => None,
            Bk => None,
            Cf => None,
            Es => kel!(1130.0),
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
