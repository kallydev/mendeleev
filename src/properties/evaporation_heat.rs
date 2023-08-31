use super::Element;
use crate::KiloJoulePerMole;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum evaporation heat across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, EVAPORATION_HEAT_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.evaporation_heat());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, EVAPORATION_HEAT_RANGE);
/// ```
pub const EVAPORATION_HEAT_RANGE: core::ops::RangeInclusive<KiloJoulePerMole> =
    KiloJoulePerMole(0.08)..=KiloJoulePerMole(824.0);

macro_rules! kj {
    ($value:literal) => {
        Some(KiloJoulePerMole($value))
    };
}

impl Element {
    /// Returns the element's evaporation heat, if known.
    ///
    /// ```
    /// use mendeleev::{Element, KiloJoulePerMole};
    /// assert_eq!(Element::H.evaporation_heat(), Some(KiloJoulePerMole(0.904)));
    /// ```
    pub const fn evaporation_heat(&self) -> Option<KiloJoulePerMole> {
        use Element as E;
        match self {
            E::H => kj!(0.904),
            E::He => kj!(0.08),
            E::Li => kj!(148.0),
            E::Be => kj!(309.0),
            E::B => kj!(504.5),
            E::C => None,
            E::N => None,
            E::O => None,
            E::F => kj!(6.54),
            E::Ne => kj!(1.74),
            E::Na => kj!(97.9),
            E::Mg => kj!(131.8),
            E::Al => kj!(284.1),
            E::Si => kj!(383.0),
            E::P => kj!(49.8),
            E::S => kj!(10.5),
            E::Cl => kj!(20.41),
            E::Ar => kj!(6.52),
            E::K => kj!(2.33),
            E::Ca => kj!(153.6),
            E::Sc => kj!(332.7),
            E::Ti => kj!(422.6),
            E::V => kj!(460.0),
            E::Cr => kj!(342.0),
            E::Mn => kj!(221.0),
            E::Fe => kj!(340.0),
            E::Co => kj!(389.1),
            E::Ni => kj!(378.6),
            E::Cu => kj!(304.6),
            E::Zn => kj!(114.8),
            E::Ga => kj!(270.3),
            E::Ge => kj!(328.0),
            E::As => kj!(32.4),
            E::Se => kj!(59.7),
            E::Br => kj!(29.56),
            E::Kr => kj!(9.05),
            E::Rb => kj!(75.8),
            E::Sr => kj!(144.0),
            E::Y => kj!(367.0),
            E::Zr => kj!(567.0),
            E::Nb => kj!(680.0),
            E::Mo => kj!(590.0),
            E::Tc => kj!(585.0),
            E::Ru => None,
            E::Rh => kj!(494.0),
            E::Pd => kj!(372.4),
            E::Ag => kj!(254.1),
            E::Cd => kj!(59.1),
            E::In => kj!(225.1),
            E::Sn => kj!(296.0),
            E::Sb => kj!(195.2),
            E::Te => kj!(49.8),
            E::I => kj!(41.95),
            E::Xe => kj!(12.65),
            E::Cs => kj!(68.3),
            E::Ba => kj!(142.0),
            E::La => kj!(402.0),
            E::Ce => kj!(398.0),
            E::Pr => kj!(331.0),
            E::Nd => kj!(289.0),
            E::Pm => None,
            E::Sm => kj!(165.0),
            E::Eu => kj!(176.0),
            E::Gd => kj!(398.0),
            E::Tb => kj!(389.0),
            E::Dy => kj!(291.0),
            E::Ho => kj!(301.0),
            E::Er => kj!(317.0),
            E::Tm => kj!(232.0),
            E::Yb => kj!(159.0),
            E::Lu => kj!(414.0),
            E::Hf => kj!(575.0),
            E::Ta => kj!(758.0),
            E::W => kj!(824.0),
            E::Re => kj!(704.0),
            E::Os => kj!(738.0),
            E::Ir => kj!(604.0),
            E::Pt => kj!(470.0),
            E::Au => kj!(340.0),
            E::Hg => kj!(58.5),
            E::Tl => kj!(162.4),
            E::Pb => kj!(177.8),
            E::Bi => kj!(172.0),
            E::Po => kj!(102.9),
            E::At => None,
            E::Rn => kj!(18.1),
            E::Fr => None,
            E::Ra => kj!(113.0),
            E::Ac => kj!(292.9),
            E::Th => kj!(513.7),
            E::Pa => kj!(481.2),
            E::U => kj!(417.0),
            E::Np => kj!(336.0),
            E::Pu => kj!(343.5),
            E::Am => kj!(238.5),
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
