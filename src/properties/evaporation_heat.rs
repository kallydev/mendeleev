use super::Element;
use crate::KiloJoulePerMol;

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
pub const EVAPORATION_HEAT_RANGE: core::ops::RangeInclusive<KiloJoulePerMol> =
    KiloJoulePerMol(0.08)..=KiloJoulePerMol(824.0);

macro_rules! kj {
    ($value:literal) => {
        Some(KiloJoulePerMol($value))
    };
}

impl Element {
    /// Returns the element's evaporation heat, if known.
    ///
    /// ```
    /// use mendeleev::{Element, KiloJoulePerMol};
    /// assert_eq!(Element::H.evaporation_heat(), Some(KiloJoulePerMol(0.904)));
    /// ```
    pub const fn evaporation_heat(&self) -> Option<KiloJoulePerMol> {
        use Element::*;
        match self {
            H => kj!(0.904),
            He => kj!(0.08),
            Li => kj!(148.0),
            Be => kj!(309.0),
            B => kj!(504.5),
            C => None,
            N => None,
            O => None,
            F => kj!(6.54),
            Ne => kj!(1.74),
            Na => kj!(97.9),
            Mg => kj!(131.8),
            Al => kj!(284.1),
            Si => kj!(383.0),
            P => kj!(49.8),
            S => kj!(10.5),
            Cl => kj!(20.41),
            Ar => kj!(6.52),
            K => kj!(2.33),
            Ca => kj!(153.6),
            Sc => kj!(332.7),
            Ti => kj!(422.6),
            V => kj!(460.0),
            Cr => kj!(342.0),
            Mn => kj!(221.0),
            Fe => kj!(340.0),
            Co => kj!(389.1),
            Ni => kj!(378.6),
            Cu => kj!(304.6),
            Zn => kj!(114.8),
            Ga => kj!(270.3),
            Ge => kj!(328.0),
            As => kj!(32.4),
            Se => kj!(59.7),
            Br => kj!(29.56),
            Kr => kj!(9.05),
            Rb => kj!(75.8),
            Sr => kj!(144.0),
            Y => kj!(367.0),
            Zr => kj!(567.0),
            Nb => kj!(680.0),
            Mo => kj!(590.0),
            Tc => kj!(585.0),
            Ru => None,
            Rh => kj!(494.0),
            Pd => kj!(372.4),
            Ag => kj!(254.1),
            Cd => kj!(59.1),
            In => kj!(225.1),
            Sn => kj!(296.0),
            Sb => kj!(195.2),
            Te => kj!(49.8),
            I => kj!(41.95),
            Xe => kj!(12.65),
            Cs => kj!(68.3),
            Ba => kj!(142.0),
            La => kj!(402.0),
            Ce => kj!(398.0),
            Pr => kj!(331.0),
            Nd => kj!(289.0),
            Pm => None,
            Sm => kj!(165.0),
            Eu => kj!(176.0),
            Gd => kj!(398.0),
            Tb => kj!(389.0),
            Dy => kj!(291.0),
            Ho => kj!(301.0),
            Er => kj!(317.0),
            Tm => kj!(232.0),
            Yb => kj!(159.0),
            Lu => kj!(414.0),
            Hf => kj!(575.0),
            Ta => kj!(758.0),
            W => kj!(824.0),
            Re => kj!(704.0),
            Os => kj!(738.0),
            Ir => kj!(604.0),
            Pt => kj!(470.0),
            Au => kj!(340.0),
            Hg => kj!(58.5),
            Tl => kj!(162.4),
            Pb => kj!(177.8),
            Bi => kj!(172.0),
            Po => kj!(102.9),
            At => None,
            Rn => kj!(18.1),
            Fr => None,
            Ra => kj!(113.0),
            Ac => kj!(292.9),
            Th => kj!(513.7),
            Pa => kj!(481.2),
            U => kj!(417.0),
            Np => kj!(336.0),
            Pu => kj!(343.5),
            Am => kj!(238.5),
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
