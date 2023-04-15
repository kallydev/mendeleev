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
        use Element::*;
        match self {
            H => gc!(0.00008988),
            He => gc!(0.0001785),
            Li => gc!(0.534),
            Be => gc!(1.85),
            B => gc!(2.37),
            C => gc!(2.2670),
            N => gc!(0.0012506),
            O => gc!(0.001429),
            F => gc!(0.001696),
            Ne => gc!(0.0008999),
            Na => gc!(0.97),
            Mg => gc!(1.74),
            Al => gc!(2.70),
            Si => gc!(2.3296),
            P => gc!(1.82),
            S => gc!(2.067),
            Cl => gc!(0.003214),
            Ar => gc!(0.0017837),
            K => gc!(0.89),
            Ca => gc!(1.54),
            Sc => gc!(2.99),
            Ti => gc!(4.5),
            V => gc!(6.0),
            Cr => gc!(7.15),
            Mn => gc!(7.3),
            Fe => gc!(7.874),
            Co => gc!(8.86),
            Ni => gc!(8.912),
            Cu => gc!(8.933),
            Zn => gc!(7.134),
            Ga => gc!(5.91),
            Ge => gc!(5.323),
            As => gc!(5.776),
            Se => gc!(4.809),
            Br => gc!(3.11),
            Kr => gc!(0.003733),
            Rb => gc!(1.53),
            Sr => gc!(2.64),
            Y => gc!(4.47),
            Zr => gc!(6.52),
            Nb => gc!(8.57),
            Mo => gc!(10.2),
            Tc => gc!(11.0),
            Ru => gc!(12.1),
            Rh => gc!(12.4),
            Pd => gc!(12.0),
            Ag => gc!(10.501),
            Cd => gc!(8.69),
            In => gc!(7.31),
            Sn => gc!(7.287),
            Sb => gc!(6.685),
            Te => gc!(6.232),
            I => gc!(4.93),
            Xe => gc!(0.005887),
            Cs => gc!(1.93),
            Ba => gc!(3.62),
            La => gc!(6.15),
            Ce => gc!(6.770),
            Pr => gc!(6.77),
            Nd => gc!(7.01),
            Pm => gc!(7.26),
            Sm => gc!(7.52),
            Eu => gc!(5.24),
            Gd => gc!(7.90),
            Tb => gc!(8.23),
            Dy => gc!(8.55),
            Ho => gc!(8.80),
            Er => gc!(9.07),
            Tm => gc!(9.32),
            Yb => gc!(6.90),
            Lu => gc!(9.84),
            Hf => gc!(13.3),
            Ta => gc!(16.4),
            W => gc!(19.3),
            Re => gc!(20.8),
            Os => gc!(22.57),
            Ir => gc!(22.42),
            Pt => gc!(21.46),
            Au => gc!(19.282),
            Hg => gc!(13.5336),
            Tl => gc!(11.8),
            Pb => gc!(11.342),
            Bi => gc!(9.807),
            Po => gc!(9.32),
            At => gc!(7.0),
            Rn => gc!(0.00973),
            Fr => None,
            Ra => gc!(5.0),
            Ac => gc!(10.07),
            Th => gc!(11.72),
            Pa => gc!(15.37),
            U => gc!(18.95),
            Np => gc!(20.25),
            Pu => gc!(19.84),
            Am => gc!(13.69),
            Cm => gc!(13.51),
            Bk => gc!(14.0),
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
