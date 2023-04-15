use super::Element;
use crate::Kelvin;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum melting point across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, MELTING_POINT_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.melting_point());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, MELTING_POINT_RANGE);
/// ```
pub const MELTING_POINT_RANGE: core::ops::RangeInclusive<Kelvin> = Kelvin(0.95)..=Kelvin(3820.0);

macro_rules! kel {
    ($value:literal) => {
        Some(Kelvin($value))
    };
}

impl Element {
    /// Returns the element's melting point, if known.
    ///
    /// ```
    /// use mendeleev::{Element, Kelvin};
    /// assert_eq!(Element::H.melting_point(), Some(Kelvin(14.01)));
    /// assert_eq!(Element::C.melting_point(), Some(Kelvin(3820.0)));
    /// assert_eq!(Element::Og.melting_point(), None);
    /// ```
    pub const fn melting_point(&self) -> Option<Kelvin> {
        use Element::*;
        match self {
            H => kel!(14.01),
            He => kel!(0.95),
            Li => kel!(553.69),
            Be => kel!(1551.0),
            B => kel!(2573.0),
            C => kel!(3820.0),
            N => kel!(63.29),
            O => kel!(54.8),
            F => kel!(53.53),
            Ne => kel!(48.0),
            Na => kel!(370.96),
            Mg => kel!(922.0),
            Al => kel!(933.5),
            Si => kel!(1683.0),
            P => kel!(317.3),
            S => kel!(386.0),
            Cl => kel!(172.2),
            Ar => kel!(83.8),
            K => kel!(336.8),
            Ca => kel!(1112.0),
            Sc => kel!(1814.0),
            Ti => kel!(1933.0),
            V => kel!(2160.0),
            Cr => kel!(2130.0),
            Mn => kel!(1517.0),
            Fe => kel!(1808.0),
            Co => kel!(1768.0),
            Ni => kel!(1726.0),
            Cu => kel!(1356.6),
            Zn => kel!(692.73),
            Ga => kel!(302.93),
            Ge => kel!(1210.6),
            As => kel!(1090.0),
            Se => kel!(490.0),
            Br => kel!(265.9),
            Kr => kel!(116.6),
            Rb => kel!(312.2),
            Sr => kel!(1042.0),
            Y => kel!(1795.0),
            Zr => kel!(2125.0),
            Nb => kel!(2741.0),
            Mo => kel!(2890.0),
            Tc => kel!(2445.0),
            Ru => kel!(2583.0),
            Rh => kel!(2239.0),
            Pd => kel!(1825.0),
            Ag => kel!(1235.1),
            Cd => kel!(594.1),
            In => kel!(429.32),
            Sn => kel!(505.1),
            Sb => kel!(903.9),
            Te => kel!(722.7),
            I => kel!(386.7),
            Xe => kel!(161.3),
            Cs => kel!(301.6),
            Ba => kel!(1002.0),
            La => kel!(1194.0),
            Ce => kel!(1072.0),
            Pr => kel!(1204.0),
            Nd => kel!(1294.0),
            Pm => kel!(1441.0),
            Sm => kel!(1350.0),
            Eu => kel!(1095.0),
            Gd => kel!(1586.0),
            Tb => kel!(1629.0),
            Dy => kel!(1685.0),
            Ho => kel!(1747.0),
            Er => kel!(1802.0),
            Tm => kel!(1818.0),
            Yb => kel!(1097.0),
            Lu => kel!(1936.0),
            Hf => kel!(2503.0),
            Ta => kel!(3269.0),
            W => kel!(3680.0),
            Re => kel!(3453.0),
            Os => kel!(3327.0),
            Ir => kel!(2683.0),
            Pt => kel!(2045.0),
            Au => kel!(1337.58),
            Hg => kel!(234.28),
            Tl => kel!(576.6),
            Pb => kel!(600.65),
            Bi => kel!(544.5),
            Po => kel!(527.0),
            At => kel!(575.0),
            Rn => kel!(202.0),
            Fr => kel!(300.0),
            Ra => kel!(973.0),
            Ac => kel!(1320.0),
            Th => kel!(2028.0),
            Pa => kel!(2113.0),
            U => kel!(1405.5),
            Np => kel!(913.0),
            Pu => kel!(914.0),
            Am => kel!(1267.0),
            Cm => kel!(1340.0),
            Bk => None,
            Cf => kel!(900.0),
            Es => None,
            Fm => kel!(1800.0),
            Md => kel!(1100.0),
            No => kel!(1100.0),
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
