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
        use Element as E;
        match self {
            E::H => kel!(14.01),
            E::He => kel!(0.95),
            E::Li => kel!(553.69),
            E::Be => kel!(1551.0),
            E::B => kel!(2573.0),
            E::C => kel!(3820.0),
            E::N => kel!(63.29),
            E::O => kel!(54.8),
            E::F => kel!(53.53),
            E::Ne => kel!(48.0),
            E::Na => kel!(370.96),
            E::Mg => kel!(922.0),
            E::Al => kel!(933.5),
            E::Si => kel!(1683.0),
            E::P => kel!(317.3),
            E::S => kel!(386.0),
            E::Cl => kel!(172.2),
            E::Ar => kel!(83.8),
            E::K => kel!(336.8),
            E::Ca => kel!(1112.0),
            E::Sc => kel!(1814.0),
            E::Ti => kel!(1933.0),
            E::V => kel!(2160.0),
            E::Cr => kel!(2130.0),
            E::Mn => kel!(1517.0),
            E::Fe => kel!(1808.0),
            E::Co => kel!(1768.0),
            E::Ni => kel!(1726.0),
            E::Cu => kel!(1356.6),
            E::Zn => kel!(692.73),
            E::Ga => kel!(302.93),
            E::Ge => kel!(1210.6),
            E::As => kel!(1090.0),
            E::Se => kel!(490.0),
            E::Br => kel!(265.9),
            E::Kr => kel!(116.6),
            E::Rb => kel!(312.2),
            E::Sr => kel!(1042.0),
            E::Y => kel!(1795.0),
            E::Zr => kel!(2125.0),
            E::Nb => kel!(2741.0),
            E::Mo => kel!(2890.0),
            E::Tc => kel!(2445.0),
            E::Ru => kel!(2583.0),
            E::Rh => kel!(2239.0),
            E::Pd => kel!(1825.0),
            E::Ag => kel!(1235.1),
            E::Cd => kel!(594.1),
            E::In => kel!(429.32),
            E::Sn => kel!(505.1),
            E::Sb => kel!(903.9),
            E::Te => kel!(722.7),
            E::I => kel!(386.7),
            E::Xe => kel!(161.3),
            E::Cs => kel!(301.6),
            E::Ba => kel!(1002.0),
            E::La => kel!(1194.0),
            E::Ce => kel!(1072.0),
            E::Pr => kel!(1204.0),
            E::Nd => kel!(1294.0),
            E::Pm => kel!(1441.0),
            E::Sm => kel!(1350.0),
            E::Eu => kel!(1095.0),
            E::Gd => kel!(1586.0),
            E::Tb => kel!(1629.0),
            E::Dy => kel!(1685.0),
            E::Ho => kel!(1747.0),
            E::Er => kel!(1802.0),
            E::Tm => kel!(1818.0),
            E::Yb => kel!(1097.0),
            E::Lu => kel!(1936.0),
            E::Hf => kel!(2503.0),
            E::Ta => kel!(3269.0),
            E::W => kel!(3680.0),
            E::Re => kel!(3453.0),
            E::Os => kel!(3327.0),
            E::Ir => kel!(2683.0),
            E::Pt => kel!(2045.0),
            E::Au => kel!(1337.58),
            E::Hg => kel!(234.28),
            E::Tl => kel!(576.6),
            E::Pb => kel!(600.65),
            E::Bi => kel!(544.5),
            E::Po => kel!(527.0),
            E::At => kel!(575.0),
            E::Rn => kel!(202.0),
            E::Fr => kel!(300.0),
            E::Ra => kel!(973.0),
            E::Ac => kel!(1320.0),
            E::Th => kel!(2028.0),
            E::Pa => kel!(2113.0),
            E::U => kel!(1405.5),
            E::Np => kel!(913.0),
            E::Pu => kel!(914.0),
            E::Am => kel!(1267.0),
            E::Cm => kel!(1340.0),
            E::Bk => None,
            E::Cf => kel!(900.0),
            E::Es => None,
            E::Fm => kel!(1800.0),
            E::Md => kel!(1100.0),
            E::No => kel!(1100.0),
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
