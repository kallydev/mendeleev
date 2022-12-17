use super::Element;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum boiling point across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, BOILING_POINT_RANGE};
/// let all_values = Element::list().iter().flat_map(|e| e.boiling_point());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, BOILING_POINT_RANGE);
/// ```
pub const BOILING_POINT_RANGE: core::ops::RangeInclusive<f64> = 4.216..=5930.0;

impl Element {
    /// Returns the element's boiling point in Kelvin, if known.
    ///
    /// ```
    /// use mendeleev::Element;
    /// assert_eq!(Element::H.boiling_point(), Some(20.28));
    /// assert_eq!(Element::C.boiling_point(), Some(5100.0));
    /// assert_eq!(Element::Og.boiling_point(), None);
    /// ```
    pub const fn boiling_point(&self) -> Option<f64> {
        match self {
            Element::H => Some(20.28),
            Element::He => Some(4.216),
            Element::Li => Some(1118.15),
            Element::Be => Some(3243.0),
            Element::B => Some(3931.0),
            Element::C => Some(5100.0),
            Element::N => Some(77.4),
            Element::O => Some(90.19),
            Element::F => Some(85.01),
            Element::Ne => Some(27.1),
            Element::Na => Some(1156.1),
            Element::Mg => Some(1363.0),
            Element::Al => Some(2740.0),
            Element::Si => Some(2628.0),
            Element::P => Some(553.0),
            Element::S => Some(717.824),
            Element::Cl => Some(238.6),
            Element::Ar => Some(87.3),
            Element::K => Some(1047.0),
            Element::Ca => Some(1757.0),
            Element::Sc => Some(3104.0),
            Element::Ti => Some(3560.0),
            Element::V => Some(3650.0),
            Element::Cr => Some(2945.0),
            Element::Mn => Some(2235.0),
            Element::Fe => Some(3023.0),
            Element::Co => Some(3143.0),
            Element::Ni => Some(3005.0),
            Element::Cu => Some(2840.0),
            Element::Zn => Some(1180.0),
            Element::Ga => Some(2676.0),
            Element::Ge => Some(3103.0),
            Element::As => Some(876.0),
            Element::Se => Some(958.1),
            Element::Br => Some(331.9),
            Element::Kr => Some(120.85),
            Element::Rb => Some(961.0),
            Element::Sr => Some(1657.0),
            Element::Y => Some(3611.0),
            Element::Zr => Some(4650.0),
            Element::Nb => Some(5015.0),
            Element::Mo => Some(4885.0),
            Element::Tc => Some(5150.0),
            Element::Ru => Some(4173.0),
            Element::Rh => Some(4000.0),
            Element::Pd => Some(3413.0),
            Element::Ag => Some(2485.0),
            Element::Cd => Some(1038.0),
            Element::In => Some(2353.0),
            Element::Sn => Some(2543.0),
            Element::Sb => Some(1908.0),
            Element::Te => Some(1263.0),
            Element::I => Some(457.5),
            Element::Xe => Some(166.1),
            Element::Cs => Some(951.6),
            Element::Ba => Some(1910.0),
            Element::La => Some(3730.0),
            Element::Ce => Some(3699.0),
            Element::Pr => Some(3785.0),
            Element::Nd => Some(3341.0),
            Element::Pm => Some(3000.0),
            Element::Sm => Some(2064.0),
            Element::Eu => Some(1870.0),
            Element::Gd => Some(3539.0),
            Element::Tb => Some(3296.0),
            Element::Dy => Some(2835.0),
            Element::Ho => Some(2968.0),
            Element::Er => Some(3136.0),
            Element::Tm => Some(2220.0),
            Element::Yb => Some(1466.0),
            Element::Lu => Some(3668.0),
            Element::Hf => Some(5470.0),
            Element::Ta => Some(5698.0),
            Element::W => Some(5930.0),
            Element::Re => Some(5900.0),
            Element::Os => Some(5300.0),
            Element::Ir => Some(4403.0),
            Element::Pt => Some(4100.0),
            Element::Au => Some(3080.0),
            Element::Hg => Some(629.73),
            Element::Tl => Some(1730.0),
            Element::Pb => Some(2013.0),
            Element::Bi => Some(1883.0),
            Element::Po => Some(1235.0),
            Element::At => Some(610.0),
            Element::Rn => Some(211.4),
            Element::Fr => Some(950.0),
            Element::Ra => Some(1413.0),
            Element::Ac => Some(3470.0),
            Element::Th => Some(5060.0),
            Element::Pa => Some(4300.0),
            Element::U => Some(4018.0),
            Element::Np => Some(4175.0),
            Element::Pu => Some(3505.0),
            Element::Am => Some(2880.0),
            Element::Cm => None,
            Element::Bk => None,
            Element::Cf => None,
            Element::Es => Some(1130.0),
            Element::Fm => None,
            Element::Md => None,
            Element::No => None,
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
