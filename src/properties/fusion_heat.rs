use super::Element;
use crate::KiloJoulePerMole;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum fusion heat across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, FUSION_HEAT_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.fusion_heat());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, FUSION_HEAT_RANGE);
/// ```
pub const FUSION_HEAT_RANGE: core::ops::RangeInclusive<KiloJoulePerMole> =
    KiloJoulePerMole(0.117)..=KiloJoulePerMole(102.5);

macro_rules! kj {
    ($value:literal) => {
        Some(KiloJoulePerMole($value))
    };
}

impl Element {
    /// Returns the element's fusion heat, if known.
    ///
    /// ```
    /// use mendeleev::{Element, KiloJoulePerMole};
    /// assert_eq!(Element::H.fusion_heat(), Some(KiloJoulePerMole(0.117)));
    /// assert_eq!(Element::Og.fusion_heat(), None);
    /// ```
    pub const fn fusion_heat(&self) -> Option<KiloJoulePerMole> {
        use Element as E;
        match self {
            E::H => kj!(0.117),
            E::He => None,
            E::Li => kj!(2.89),
            E::Be => kj!(12.21),
            E::B => kj!(23.6),
            E::C => None,
            E::N => None,
            E::O => None,
            E::F => kj!(0.51),
            E::Ne => None,
            E::Na => kj!(2.64),
            E::Mg => kj!(9.2),
            E::Al => kj!(10.75),
            E::Si => kj!(50.6),
            E::P => kj!(2.51),
            E::S => kj!(1.23),
            E::Cl => kj!(6.41),
            E::Ar => None,
            E::K => kj!(102.5),
            E::Ca => kj!(9.2),
            E::Sc => kj!(15.8),
            E::Ti => kj!(18.8),
            E::V => kj!(17.5),
            E::Cr => kj!(21.0),
            E::Mn => kj!(13.4),
            E::Fe => kj!(13.8),
            E::Co => kj!(15.48),
            E::Ni => kj!(17.61),
            E::Cu => kj!(13.01),
            E::Zn => kj!(7.28),
            E::Ga => kj!(5.59),
            E::Ge => kj!(36.8),
            E::As => None,
            E::Se => kj!(5.23),
            E::Br => kj!(10.57),
            E::Kr => None,
            E::Rb => kj!(2.2),
            E::Sr => kj!(9.2),
            E::Y => kj!(11.5),
            E::Zr => kj!(19.2),
            E::Nb => kj!(26.8),
            E::Mo => kj!(28.0),
            E::Tc => kj!(23.8),
            E::Ru => kj!(25.5),
            E::Rh => kj!(21.8),
            E::Pd => kj!(17.24),
            E::Ag => kj!(11.95),
            E::Cd => kj!(6.11),
            E::In => kj!(3.24),
            E::Sn => kj!(7.07),
            E::Sb => kj!(20.08),
            E::Te => kj!(17.91),
            E::I => kj!(15.52),
            E::Xe => None,
            E::Cs => kj!(2.09),
            E::Ba => kj!(7.66),
            E::La => kj!(8.5),
            E::Ce => kj!(5.2),
            E::Pr => kj!(11.3),
            E::Nd => kj!(7.1),
            E::Pm => None,
            E::Sm => kj!(8.9),
            E::Eu => None,
            E::Gd => None,
            E::Tb => None,
            E::Dy => None,
            E::Ho => None,
            E::Er => None,
            E::Tm => None,
            E::Yb => kj!(3.35),
            E::Lu => None,
            E::Hf => kj!(25.1),
            E::Ta => kj!(24.7),
            E::W => kj!(35.0),
            E::Re => kj!(34.0),
            E::Os => kj!(31.7),
            E::Ir => kj!(27.61),
            E::Pt => kj!(21.76),
            E::Au => kj!(12.68),
            E::Hg => kj!(2.295),
            E::Tl => kj!(4.31),
            E::Pb => kj!(4.77),
            E::Bi => kj!(11.0),
            E::Po => kj!(10.0),
            E::At => None,
            E::Rn => None,
            E::Fr => kj!(15.0),
            E::Ra => kj!(9.6),
            E::Ac => kj!(10.5),
            E::Th => kj!(16.11),
            E::Pa => kj!(16.7),
            E::U => kj!(12.6),
            E::Np => kj!(9.6),
            E::Pu => kj!(2.8),
            E::Am => kj!(10.0),
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
