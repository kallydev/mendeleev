use super::Element;
use crate::Kelvin;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum boiling point across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, BOILING_POINT_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.boiling_point());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, BOILING_POINT_RANGE);
/// ```
pub const BOILING_POINT_RANGE: core::ops::RangeInclusive<Kelvin> = Kelvin(4.222)..=Kelvin(5863.15);

macro_rules! kel {
    ($value:literal) => {
        Some(Kelvin($value))
    };
}

impl Element {
    /// Returns the element's boiling point, if known.
    ///
    /// For elements that have multiple allotropes, one of them was chosen arbitrarily for the
    /// return value.
    ///
    /// ```
    /// use mendeleev::{Element, Kelvin};
    /// assert_eq!(Element::H.boiling_point(), Some(Kelvin(20.271)));
    /// // Graphite (sublimation point)
    /// assert_eq!(Element::C.boiling_point(), Some(Kelvin(4098.15)));
    /// // White phosphorus
    /// assert_eq!(Element::P.boiling_point(), Some(Kelvin(553.65)));
    /// assert_eq!(Element::Og.boiling_point(), None);
    /// ```
    pub const fn boiling_point(&self) -> Option<Kelvin> {
        use Element as E;
        match self {
            E::H => kel!(20.271),
            E::He => kel!(4.222),
            E::Li => kel!(1615.15),
            E::Be => kel!(2741.15),
            E::B => kel!(4273.15),
            E::C => kel!(4098.15), // graphite (sublimation)
            E::N => kel!(77.355),
            E::O => kel!(90.188),
            E::F => kel!(85.04),
            E::Ne => kel!(27.104),
            E::Na => kel!(1156.09),
            E::Mg => kel!(1363.15),
            E::Al => kel!(2792.15),
            E::Si => kel!(3538.15),
            E::P => kel!(553.65), // white
            E::S => kel!(717.76), // rhombic
            E::Cl => kel!(239.11),
            E::Ar => kel!(87.302),
            E::K => kel!(1032.15),
            E::Ca => kel!(1757.15),
            E::Sc => kel!(3109.15),
            E::Ti => kel!(3560.15),
            E::V => kel!(3680.15),
            E::Cr => kel!(2944.15),
            E::Mn => kel!(2334.15),
            E::Fe => kel!(3134.15),
            E::Co => kel!(3200.15),
            E::Ni => kel!(3186.15),
            E::Cu => kel!(2833.15),
            E::Zn => kel!(1180.15),
            E::Ga => kel!(2502.15),
            E::Ge => kel!(3106.15),
            E::As => kel!(889.15), // gray
            E::Se => kel!(958.15), // vitreous
            E::Br => kel!(331.95),
            E::Kr => kel!(119.735),
            E::Rb => kel!(961.15),
            E::Sr => kel!(1650.15),
            E::Y => kel!(3618.15),
            E::Zr => kel!(4679.15),
            E::Nb => kel!(5014.15),
            E::Mo => kel!(4912.15),
            E::Tc => kel!(4535.15),
            E::Ru => kel!(4420.15),
            E::Rh => kel!(3968.15),
            E::Pd => kel!(3236.15),
            E::Ag => kel!(2435.15),
            E::Cd => kel!(1040.15),
            E::In => kel!(2300.15),
            E::Sn => kel!(2859.15), // gray
            E::Sb => kel!(1860.15), // gray
            E::Te => kel!(1261.15),
            E::I => kel!(457.55),
            E::Xe => kel!(165.051),
            E::Cs => kel!(944.15),
            E::Ba => kel!(2118.15),
            E::La => kel!(3737.15),
            E::Ce => kel!(3716.15),
            E::Pr => kel!(3793.15),
            E::Nd => kel!(3347.15),
            E::Pm => None,
            E::Sm => kel!(2067.15),
            E::Eu => kel!(1802.15),
            E::Gd => kel!(3546.15),
            E::Tb => kel!(3503.15),
            E::Dy => kel!(2840.15),
            E::Ho => kel!(2973.15),
            E::Er => kel!(3141.15),
            E::Tm => kel!(2223.15),
            E::Yb => kel!(1469.15),
            E::Lu => kel!(3675.15),
            E::Hf => kel!(4873.15),
            E::Ta => kel!(5728.15),
            E::W => kel!(5828.15),
            E::Re => kel!(5863.15),
            E::Os => kel!(5281.15),
            E::Ir => kel!(4701.15),
            E::Pt => kel!(4098.15),
            E::Au => kel!(3109.15),
            E::Hg => kel!(629.769),
            E::Tl => kel!(1746.15),
            E::Pb => kel!(2022.15),
            E::Bi => kel!(1837.15),
            E::Po => kel!(1235.15),
            E::At => None,
            E::Rn => kel!(211.45),
            E::Fr => None,
            E::Ra => None,
            E::Ac => kel!(3473.15),
            E::Th => kel!(5058.15),
            E::Pa => None,
            E::U => kel!(4404.15),
            E::Np => None,
            E::Pu => kel!(3501.15),
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
