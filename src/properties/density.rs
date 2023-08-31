use super::Element;
use crate::GramPerCubicCentimeter;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum density across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, DENSITY_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.density());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, DENSITY_RANGE);
/// ```
pub const DENSITY_RANGE: core::ops::RangeInclusive<GramPerCubicCentimeter> =
    GramPerCubicCentimeter(0.00008988)..=GramPerCubicCentimeter(22.57);

macro_rules! gc {
    ($value:literal) => {
        Some(GramPerCubicCentimeter($value))
    };
}

impl Element {
    /// Returns the element's density in its most common form, if available.
    ///
    /// ```
    /// use mendeleev::{Element, GramPerCubicCentimeter};
    /// assert_eq!(Element::H.density(), Some(GramPerCubicCentimeter(0.00008988)));
    /// ```
    pub const fn density(&self) -> Option<GramPerCubicCentimeter> {
        use Element as E;
        match self {
            E::H => gc!(0.00008988),
            E::He => gc!(0.0001785),
            E::Li => gc!(0.534),
            E::Be => gc!(1.85),
            E::B => gc!(2.37),
            E::C => gc!(2.2670),
            E::N => gc!(0.0012506),
            E::O => gc!(0.001429),
            E::F => gc!(0.001696),
            E::Ne => gc!(0.0008999),
            E::Na => gc!(0.97),
            E::Mg => gc!(1.74),
            E::Al => gc!(2.70),
            E::Si => gc!(2.3296),
            E::P => gc!(1.82),
            E::S => gc!(2.067),
            E::Cl => gc!(0.003214),
            E::Ar => gc!(0.0017837),
            E::K => gc!(0.89),
            E::Ca => gc!(1.54),
            E::Sc => gc!(2.99),
            E::Ti => gc!(4.5),
            E::V => gc!(6.0),
            E::Cr => gc!(7.15),
            E::Mn => gc!(7.3),
            E::Fe => gc!(7.874),
            E::Co => gc!(8.86),
            E::Ni => gc!(8.912),
            E::Cu => gc!(8.933),
            E::Zn => gc!(7.134),
            E::Ga => gc!(5.91),
            E::Ge => gc!(5.323),
            E::As => gc!(5.776),
            E::Se => gc!(4.809),
            E::Br => gc!(3.11),
            E::Kr => gc!(0.003733),
            E::Rb => gc!(1.53),
            E::Sr => gc!(2.64),
            E::Y => gc!(4.47),
            E::Zr => gc!(6.52),
            E::Nb => gc!(8.57),
            E::Mo => gc!(10.2),
            E::Tc => gc!(11.0),
            E::Ru => gc!(12.1),
            E::Rh => gc!(12.4),
            E::Pd => gc!(12.0),
            E::Ag => gc!(10.501),
            E::Cd => gc!(8.69),
            E::In => gc!(7.31),
            E::Sn => gc!(7.287),
            E::Sb => gc!(6.685),
            E::Te => gc!(6.232),
            E::I => gc!(4.93),
            E::Xe => gc!(0.005887),
            E::Cs => gc!(1.93),
            E::Ba => gc!(3.62),
            E::La => gc!(6.15),
            E::Ce => gc!(6.770),
            E::Pr => gc!(6.77),
            E::Nd => gc!(7.01),
            E::Pm => gc!(7.26),
            E::Sm => gc!(7.52),
            E::Eu => gc!(5.24),
            E::Gd => gc!(7.90),
            E::Tb => gc!(8.23),
            E::Dy => gc!(8.55),
            E::Ho => gc!(8.80),
            E::Er => gc!(9.07),
            E::Tm => gc!(9.32),
            E::Yb => gc!(6.90),
            E::Lu => gc!(9.84),
            E::Hf => gc!(13.3),
            E::Ta => gc!(16.4),
            E::W => gc!(19.3),
            E::Re => gc!(20.8),
            E::Os => gc!(22.57),
            E::Ir => gc!(22.42),
            E::Pt => gc!(21.46),
            E::Au => gc!(19.282),
            E::Hg => gc!(13.5336),
            E::Tl => gc!(11.8),
            E::Pb => gc!(11.342),
            E::Bi => gc!(9.807),
            E::Po => gc!(9.32),
            E::At => gc!(7.0),
            E::Rn => gc!(0.00973),
            E::Fr => None,
            E::Ra => gc!(5.0),
            E::Ac => gc!(10.07),
            E::Th => gc!(11.72),
            E::Pa => gc!(15.37),
            E::U => gc!(18.95),
            E::Np => gc!(20.25),
            E::Pu => gc!(19.84),
            E::Am => gc!(13.69),
            E::Cm => gc!(13.51),
            E::Bk => gc!(14.0),
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
