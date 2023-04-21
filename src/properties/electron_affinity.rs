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
        use Element::*;
        match self {
            H => ev!(0.754),
            He => None,
            Li => ev!(0.618),
            Be => None,
            B => ev!(0.277),
            C => ev!(1.263),
            N => None,
            O => ev!(1.461),
            F => ev!(3.339),
            Ne => None,
            Na => ev!(0.548),
            Mg => None,
            Al => ev!(0.441),
            Si => ev!(1.385),
            P => ev!(0.746),
            S => ev!(2.077),
            Cl => ev!(3.617),
            Ar => None,
            K => ev!(0.501),
            Ca => None,
            Sc => ev!(0.188),
            Ti => ev!(0.079),
            V => ev!(0.525),
            Cr => ev!(0.666),
            Mn => None,
            Fe => ev!(0.163),
            Co => ev!(0.661),
            Ni => ev!(1.156),
            Cu => ev!(1.228),
            Zn => None,
            Ga => ev!(0.3),
            Ge => ev!(1.35),
            As => ev!(0.81),
            Se => ev!(2.021),
            Br => ev!(3.365),
            Kr => None,
            Rb => ev!(0.468),
            Sr => None,
            Y => ev!(0.307),
            Zr => ev!(0.426),
            Nb => ev!(0.893),
            Mo => ev!(0.746),
            Tc => ev!(0.55),
            Ru => ev!(1.05),
            Rh => ev!(1.137),
            Pd => ev!(0.557),
            Ag => ev!(1.302),
            Cd => None,
            In => ev!(0.3),
            Sn => ev!(1.2),
            Sb => ev!(1.07),
            Te => ev!(1.971),
            I => ev!(3.059),
            Xe => None,
            Cs => ev!(0.472),
            Ba => None,
            La => ev!(0.5),
            Ce => ev!(0.5),
            Pr => None,
            Nd => None,
            Pm => None,
            Sm => None,
            Eu => None,
            Gd => None,
            Tb => None,
            Dy => None,
            Ho => None,
            Er => None,
            Tm => None,
            Yb => None,
            Lu => None,
            Hf => None,
            Ta => ev!(0.322),
            W => ev!(0.815),
            Re => ev!(0.15),
            Os => ev!(1.1),
            Ir => ev!(1.565),
            Pt => ev!(2.128),
            Au => ev!(2.309),
            Hg => None,
            Tl => ev!(0.2),
            Pb => ev!(0.36),
            Bi => ev!(0.946),
            Po => ev!(1.9),
            At => ev!(2.8),
            Rn => None,
            Fr => ev!(0.47),
            Ra => None,
            Ac => None,
            Th => None,
            Pa => None,
            U => None,
            Np => None,
            Pu => None,
            Am => None,
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
