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
pub const MELTING_POINT_RANGE: core::ops::RangeInclusive<Kelvin> = Kelvin(13.99)..=Kelvin(3687.15);

macro_rules! kel {
    ($value:literal) => {
        Some(Kelvin($value))
    };
}

impl Element {
    /// Returns the element's melting point, if known.
    ///
    /// For some elements that do not melt at atmospheric pressure, the value is given for triple
    /// point pressure.
    ///
    /// For elements that have multiple allotropes, one of them was chosen arbitrarily for the
    /// return value.
    ///
    /// ```
    /// use mendeleev::{Element, Kelvin};
    /// assert_eq!(Element::H.melting_point(), Some(Kelvin(13.99)));
    /// // Carbon allotropes have no melting point at standard pressure
    /// assert_eq!(Element::C.melting_point(), None);
    /// // White phosphorus
    /// assert_eq!(Element::P.melting_point(), Some(Kelvin(317.3)));
    /// assert_eq!(Element::Og.melting_point(), None);
    /// ```
    pub const fn melting_point(&self) -> Option<Kelvin> {
        use Element as E;
        match self {
            E::H => kel!(13.99),
            E::He => None,
            E::Li => kel!(453.65),
            E::Be => kel!(1560.15),
            E::B => kel!(2350.15),
            E::C => None, // graphite
            E::N => kel!(63.15),
            E::O => kel!(54.36),
            E::F => kel!(53.48),
            E::Ne => kel!(24.56),
            E::Na => kel!(370.944),
            E::Mg => kel!(923.15),
            E::Al => kel!(933.473),
            E::Si => kel!(1687.15),
            E::P => kel!(317.3),  // white
            E::S => kel!(368.35), // rhombic
            E::Cl => kel!(171.65),
            E::Ar => kel!(83.81),
            E::K => kel!(336.65),
            E::Ca => kel!(1115.15),
            E::Sc => kel!(1814.15),
            E::Ti => kel!(1943.15),
            E::V => kel!(2183.15),
            E::Cr => kel!(2180.15),
            E::Mn => kel!(1519.15),
            E::Fe => kel!(1811.15),
            E::Co => kel!(1768.15),
            E::Ni => kel!(1728.15),
            E::Cu => kel!(1357.77),
            E::Zn => kel!(692.677),
            E::Ga => kel!(302.9146),
            E::Ge => kel!(1211.4),
            E::As => kel!(1090.15), // gray
            E::Se => kel!(453.15),  // vitreous
            E::Br => kel!(265.95),
            E::Kr => kel!(115.78),
            E::Rb => kel!(312.45),
            E::Sr => kel!(1050.15),
            E::Y => kel!(1795.15),
            E::Zr => kel!(2127.15),
            E::Nb => kel!(2750.15),
            E::Mo => kel!(2895.15),
            E::Tc => kel!(2430.15),
            E::Ru => kel!(2606.15),
            E::Rh => kel!(2236.15),
            E::Pd => kel!(1827.95),
            E::Ag => kel!(1234.93),
            E::Cd => kel!(594.219),
            E::In => kel!(429.7485),
            E::Sn => kel!(286.35),  // gray
            E::Sb => kel!(903.778), // gray
            E::Te => kel!(722.66),
            E::I => kel!(386.85),
            E::Xe => kel!(161.4),
            E::Cs => kel!(301.65),
            E::Ba => kel!(1000.15),
            E::La => kel!(1193.15),
            E::Ce => kel!(1072.15),
            E::Pr => kel!(1204.15),
            E::Nd => kel!(1289.15),
            E::Pm => kel!(1315.15),
            E::Sm => kel!(1345.15),
            E::Eu => kel!(1095.15),
            E::Gd => kel!(1586.15),
            E::Tb => kel!(1632.15),
            E::Dy => kel!(1685.15),
            E::Ho => kel!(1745.15),
            E::Er => kel!(1802.15),
            E::Tm => kel!(1818.15),
            E::Yb => kel!(1097.15),
            E::Lu => kel!(1936.15),
            E::Hf => kel!(2506.15),
            E::Ta => kel!(3290.15),
            E::W => kel!(3687.15),
            E::Re => kel!(3458.15),
            E::Os => kel!(3306.15),
            E::Ir => kel!(2719.15),
            E::Pt => kel!(2041.35),
            E::Au => kel!(1337.33),
            E::Hg => kel!(234.321),
            E::Tl => kel!(577.15),
            E::Pb => kel!(600.612),
            E::Bi => kel!(544.552),
            E::Po => kel!(527.15),
            E::At => kel!(575.15),
            E::Rn => kel!(202.15),
            E::Fr => kel!(294.15),
            E::Ra => kel!(969.15),
            E::Ac => kel!(1323.15),
            E::Th => kel!(2023.15),
            E::Pa => kel!(1845.15),
            E::U => kel!(1408.15),
            E::Np => kel!(917.15),
            E::Pu => kel!(913.15),
            E::Am => kel!(1449.15),
            E::Cm => kel!(1618.15),
            E::Bk => kel!(1259.15), // β form
            E::Cf => kel!(1173.15),
            E::Es => kel!(1133.15),
            E::Fm => kel!(1800.15),
            E::Md => kel!(1100.15),
            E::No => kel!(1100.15),
            E::Lr => kel!(1900.15),
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
