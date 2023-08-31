use super::Element;
use crate::Electronvolt;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum electron affinity across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, ELECTRON_AFFINITY_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.electron_affinity());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, ELECTRON_AFFINITY_RANGE);
/// ```
pub const ELECTRON_AFFINITY_RANGE: core::ops::RangeInclusive<Electronvolt> =
    Electronvolt(0.079)..=Electronvolt(3.617);

macro_rules! ev {
    ($value:literal) => {
        Some(Electronvolt($value))
    };
}

impl Element {
    /// Returns the element's electron affinity, if available.
    ///
    /// ```
    /// use mendeleev::{Element, Electronvolt};
    /// assert_eq!(Element::H.electron_affinity(), Some(Electronvolt(0.754)));
    /// ```
    pub const fn electron_affinity(&self) -> Option<Electronvolt> {
        use Element as E;
        match self {
            E::H => ev!(0.754),
            E::He => None,
            E::Li => ev!(0.618),
            E::Be => None,
            E::B => ev!(0.277),
            E::C => ev!(1.263),
            E::N => None,
            E::O => ev!(1.461),
            E::F => ev!(3.339),
            E::Ne => None,
            E::Na => ev!(0.548),
            E::Mg => None,
            E::Al => ev!(0.441),
            E::Si => ev!(1.385),
            E::P => ev!(0.746),
            E::S => ev!(2.077),
            E::Cl => ev!(3.617),
            E::Ar => None,
            E::K => ev!(0.501),
            E::Ca => None,
            E::Sc => ev!(0.188),
            E::Ti => ev!(0.079),
            E::V => ev!(0.525),
            E::Cr => ev!(0.666),
            E::Mn => None,
            E::Fe => ev!(0.163),
            E::Co => ev!(0.661),
            E::Ni => ev!(1.156),
            E::Cu => ev!(1.228),
            E::Zn => None,
            E::Ga => ev!(0.3),
            E::Ge => ev!(1.35),
            E::As => ev!(0.81),
            E::Se => ev!(2.021),
            E::Br => ev!(3.365),
            E::Kr => None,
            E::Rb => ev!(0.468),
            E::Sr => None,
            E::Y => ev!(0.307),
            E::Zr => ev!(0.426),
            E::Nb => ev!(0.893),
            E::Mo => ev!(0.746),
            E::Tc => ev!(0.55),
            E::Ru => ev!(1.05),
            E::Rh => ev!(1.137),
            E::Pd => ev!(0.557),
            E::Ag => ev!(1.302),
            E::Cd => None,
            E::In => ev!(0.3),
            E::Sn => ev!(1.2),
            E::Sb => ev!(1.07),
            E::Te => ev!(1.971),
            E::I => ev!(3.059),
            E::Xe => None,
            E::Cs => ev!(0.472),
            E::Ba => None,
            E::La => ev!(0.5),
            E::Ce => ev!(0.5),
            E::Pr => None,
            E::Nd => None,
            E::Pm => None,
            E::Sm => None,
            E::Eu => None,
            E::Gd => None,
            E::Tb => None,
            E::Dy => None,
            E::Ho => None,
            E::Er => None,
            E::Tm => None,
            E::Yb => None,
            E::Lu => None,
            E::Hf => None,
            E::Ta => ev!(0.322),
            E::W => ev!(0.815),
            E::Re => ev!(0.15),
            E::Os => ev!(1.1),
            E::Ir => ev!(1.565),
            E::Pt => ev!(2.128),
            E::Au => ev!(2.309),
            E::Hg => None,
            E::Tl => ev!(0.2),
            E::Pb => ev!(0.36),
            E::Bi => ev!(0.946),
            E::Po => ev!(1.9),
            E::At => ev!(2.8),
            E::Rn => None,
            E::Fr => ev!(0.47),
            E::Ra => None,
            E::Ac => None,
            E::Th => None,
            E::Pa => None,
            E::U => None,
            E::Np => None,
            E::Pu => None,
            E::Am => None,
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
