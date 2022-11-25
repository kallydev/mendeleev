use super::Element;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum fusion heat across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, FUSION_HEAT_RANGE};
/// let all_values = Element::list().iter().flat_map(|e| e.fusion_heat());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, FUSION_HEAT_RANGE);
/// ```
pub const FUSION_HEAT_RANGE: std::ops::RangeInclusive<f64> = 0.117..=102.5;

impl Element {
    /// Returns the element's fusion heat in kJ/mol, if known.
    ///
    /// ```
    /// use mendeleev::Element;
    /// assert_eq!(Element::H.fusion_heat(), Some(0.117));
    /// assert_eq!(Element::Og.fusion_heat(), None);
    /// ```
    pub const fn fusion_heat(&self) -> Option<f64> {
        match self {
            Element::H => Some(0.117),
            Element::He => None,
            Element::Li => Some(2.89),
            Element::Be => Some(12.21),
            Element::B => Some(23.6),
            Element::C => None,
            Element::N => None,
            Element::O => None,
            Element::F => Some(0.51),
            Element::Ne => None,
            Element::Na => Some(2.64),
            Element::Mg => Some(9.2),
            Element::Al => Some(10.75),
            Element::Si => Some(50.6),
            Element::P => Some(2.51),
            Element::S => Some(1.23),
            Element::Cl => Some(6.41),
            Element::Ar => None,
            Element::K => Some(102.5),
            Element::Ca => Some(9.2),
            Element::Sc => Some(15.8),
            Element::Ti => Some(18.8),
            Element::V => Some(17.5),
            Element::Cr => Some(21.0),
            Element::Mn => Some(13.4),
            Element::Fe => Some(13.8),
            Element::Co => Some(15.48),
            Element::Ni => Some(17.61),
            Element::Cu => Some(13.01),
            Element::Zn => Some(7.28),
            Element::Ga => Some(5.59),
            Element::Ge => Some(36.8),
            Element::As => None,
            Element::Se => Some(5.23),
            Element::Br => Some(10.57),
            Element::Kr => None,
            Element::Rb => Some(2.2),
            Element::Sr => Some(9.2),
            Element::Y => Some(11.5),
            Element::Zr => Some(19.2),
            Element::Nb => Some(26.8),
            Element::Mo => Some(28.0),
            Element::Tc => Some(23.8),
            Element::Ru => Some(25.5),
            Element::Rh => Some(21.8),
            Element::Pd => Some(17.24),
            Element::Ag => Some(11.95),
            Element::Cd => Some(6.11),
            Element::In => Some(3.24),
            Element::Sn => Some(7.07),
            Element::Sb => Some(20.08),
            Element::Te => Some(17.91),
            Element::I => Some(15.52),
            Element::Xe => None,
            Element::Cs => Some(2.09),
            Element::Ba => Some(7.66),
            Element::La => Some(8.5),
            Element::Ce => Some(5.2),
            Element::Pr => Some(11.3),
            Element::Nd => Some(7.1),
            Element::Pm => None,
            Element::Sm => Some(8.9),
            Element::Eu => None,
            Element::Gd => None,
            Element::Tb => None,
            Element::Dy => None,
            Element::Ho => None,
            Element::Er => None,
            Element::Tm => None,
            Element::Yb => Some(3.35),
            Element::Lu => None,
            Element::Hf => Some(25.1),
            Element::Ta => Some(24.7),
            Element::W => Some(35.0),
            Element::Re => Some(34.0),
            Element::Os => Some(31.7),
            Element::Ir => Some(27.61),
            Element::Pt => Some(21.76),
            Element::Au => Some(12.68),
            Element::Hg => Some(2.295),
            Element::Tl => Some(4.31),
            Element::Pb => Some(4.77),
            Element::Bi => Some(11.0),
            Element::Po => Some(10.0),
            Element::At => None,
            Element::Rn => None,
            Element::Fr => Some(15.0),
            Element::Ra => Some(9.6),
            Element::Ac => Some(10.5),
            Element::Th => Some(16.11),
            Element::Pa => Some(16.7),
            Element::U => Some(12.6),
            Element::Np => Some(9.6),
            Element::Pu => Some(2.8),
            Element::Am => Some(10.0),
            Element::Cm => None,
            Element::Bk => None,
            Element::Cf => None,
            Element::Es => None,
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
