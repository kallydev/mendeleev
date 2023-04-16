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
        use Element::*;
        match self {
            H => kj!(0.117),
            He => None,
            Li => kj!(2.89),
            Be => kj!(12.21),
            B => kj!(23.6),
            C => None,
            N => None,
            O => None,
            F => kj!(0.51),
            Ne => None,
            Na => kj!(2.64),
            Mg => kj!(9.2),
            Al => kj!(10.75),
            Si => kj!(50.6),
            P => kj!(2.51),
            S => kj!(1.23),
            Cl => kj!(6.41),
            Ar => None,
            K => kj!(102.5),
            Ca => kj!(9.2),
            Sc => kj!(15.8),
            Ti => kj!(18.8),
            V => kj!(17.5),
            Cr => kj!(21.0),
            Mn => kj!(13.4),
            Fe => kj!(13.8),
            Co => kj!(15.48),
            Ni => kj!(17.61),
            Cu => kj!(13.01),
            Zn => kj!(7.28),
            Ga => kj!(5.59),
            Ge => kj!(36.8),
            As => None,
            Se => kj!(5.23),
            Br => kj!(10.57),
            Kr => None,
            Rb => kj!(2.2),
            Sr => kj!(9.2),
            Y => kj!(11.5),
            Zr => kj!(19.2),
            Nb => kj!(26.8),
            Mo => kj!(28.0),
            Tc => kj!(23.8),
            Ru => kj!(25.5),
            Rh => kj!(21.8),
            Pd => kj!(17.24),
            Ag => kj!(11.95),
            Cd => kj!(6.11),
            In => kj!(3.24),
            Sn => kj!(7.07),
            Sb => kj!(20.08),
            Te => kj!(17.91),
            I => kj!(15.52),
            Xe => None,
            Cs => kj!(2.09),
            Ba => kj!(7.66),
            La => kj!(8.5),
            Ce => kj!(5.2),
            Pr => kj!(11.3),
            Nd => kj!(7.1),
            Pm => None,
            Sm => kj!(8.9),
            Eu => None,
            Gd => None,
            Tb => None,
            Dy => None,
            Ho => None,
            Er => None,
            Tm => None,
            Yb => kj!(3.35),
            Lu => None,
            Hf => kj!(25.1),
            Ta => kj!(24.7),
            W => kj!(35.0),
            Re => kj!(34.0),
            Os => kj!(31.7),
            Ir => kj!(27.61),
            Pt => kj!(21.76),
            Au => kj!(12.68),
            Hg => kj!(2.295),
            Tl => kj!(4.31),
            Pb => kj!(4.77),
            Bi => kj!(11.0),
            Po => kj!(10.0),
            At => None,
            Rn => None,
            Fr => kj!(15.0),
            Ra => kj!(9.6),
            Ac => kj!(10.5),
            Th => kj!(16.11),
            Pa => kj!(16.7),
            U => kj!(12.6),
            Np => kj!(9.6),
            Pu => kj!(2.8),
            Am => kj!(10.0),
            Cm => None,
            Bk => None,
            Cf => None,
            Es => None,
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
