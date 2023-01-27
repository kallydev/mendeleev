use super::Element;

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
pub const MELTING_POINT_RANGE: core::ops::RangeInclusive<f64> = 0.95..=3820.0;

impl Element {
    /// Returns the element's melting point in Kelvin, if known.
    ///
    /// ```
    /// use mendeleev::Element;
    /// assert_eq!(Element::H.melting_point(), Some(14.01));
    /// assert_eq!(Element::C.melting_point(), Some(3820.0));
    /// assert_eq!(Element::Og.melting_point(), None);
    /// ```
    pub const fn melting_point(&self) -> Option<f64> {
        match self {
            Element::H => Some(14.01),
            Element::He => Some(0.95),
            Element::Li => Some(553.69),
            Element::Be => Some(1551.0),
            Element::B => Some(2573.0),
            Element::C => Some(3820.0),
            Element::N => Some(63.29),
            Element::O => Some(54.8),
            Element::F => Some(53.53),
            Element::Ne => Some(48.0),
            Element::Na => Some(370.96),
            Element::Mg => Some(922.0),
            Element::Al => Some(933.5),
            Element::Si => Some(1683.0),
            Element::P => Some(317.3),
            Element::S => Some(386.0),
            Element::Cl => Some(172.2),
            Element::Ar => Some(83.8),
            Element::K => Some(336.8),
            Element::Ca => Some(1112.0),
            Element::Sc => Some(1814.0),
            Element::Ti => Some(1933.0),
            Element::V => Some(2160.0),
            Element::Cr => Some(2130.0),
            Element::Mn => Some(1517.0),
            Element::Fe => Some(1808.0),
            Element::Co => Some(1768.0),
            Element::Ni => Some(1726.0),
            Element::Cu => Some(1356.6),
            Element::Zn => Some(692.73),
            Element::Ga => Some(302.93),
            Element::Ge => Some(1210.6),
            Element::As => Some(1090.0),
            Element::Se => Some(490.0),
            Element::Br => Some(265.9),
            Element::Kr => Some(116.6),
            Element::Rb => Some(312.2),
            Element::Sr => Some(1042.0),
            Element::Y => Some(1795.0),
            Element::Zr => Some(2125.0),
            Element::Nb => Some(2741.0),
            Element::Mo => Some(2890.0),
            Element::Tc => Some(2445.0),
            Element::Ru => Some(2583.0),
            Element::Rh => Some(2239.0),
            Element::Pd => Some(1825.0),
            Element::Ag => Some(1235.1),
            Element::Cd => Some(594.1),
            Element::In => Some(429.32),
            Element::Sn => Some(505.1),
            Element::Sb => Some(903.9),
            Element::Te => Some(722.7),
            Element::I => Some(386.7),
            Element::Xe => Some(161.3),
            Element::Cs => Some(301.6),
            Element::Ba => Some(1002.0),
            Element::La => Some(1194.0),
            Element::Ce => Some(1072.0),
            Element::Pr => Some(1204.0),
            Element::Nd => Some(1294.0),
            Element::Pm => Some(1441.0),
            Element::Sm => Some(1350.0),
            Element::Eu => Some(1095.0),
            Element::Gd => Some(1586.0),
            Element::Tb => Some(1629.0),
            Element::Dy => Some(1685.0),
            Element::Ho => Some(1747.0),
            Element::Er => Some(1802.0),
            Element::Tm => Some(1818.0),
            Element::Yb => Some(1097.0),
            Element::Lu => Some(1936.0),
            Element::Hf => Some(2503.0),
            Element::Ta => Some(3269.0),
            Element::W => Some(3680.0),
            Element::Re => Some(3453.0),
            Element::Os => Some(3327.0),
            Element::Ir => Some(2683.0),
            Element::Pt => Some(2045.0),
            Element::Au => Some(1337.58),
            Element::Hg => Some(234.28),
            Element::Tl => Some(576.6),
            Element::Pb => Some(600.65),
            Element::Bi => Some(544.5),
            Element::Po => Some(527.0),
            Element::At => Some(575.0),
            Element::Rn => Some(202.0),
            Element::Fr => Some(300.0),
            Element::Ra => Some(973.0),
            Element::Ac => Some(1320.0),
            Element::Th => Some(2028.0),
            Element::Pa => Some(2113.0),
            Element::U => Some(1405.5),
            Element::Np => Some(913.0),
            Element::Pu => Some(914.0),
            Element::Am => Some(1267.0),
            Element::Cm => Some(1340.0),
            Element::Bk => None,
            Element::Cf => Some(900.0),
            Element::Es => None,
            Element::Fm => Some(1800.0),
            Element::Md => Some(1100.0),
            Element::No => Some(1100.0),
            Element::Lr => None,
            Element::Rf => None,
            Element::Db => None,
            Element::Sg => None,
            Element::Bh => None,
            Element::Hs => None,
            Element::Mt => None,
            Element::Ds => None,
            Element::Rg => None,
            Element::Cn => None,
            Element::Nh => None,
            Element::Fl => None,
            Element::Mc => None,
            Element::Lv => None,
            Element::Ts => None,
            Element::Og => None,
        }
    }
}
