use super::Element;
use crate::Electronvolt;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum ionization energy across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, IONIZATION_ENERGY_RANGE};
/// let all_values = Element::iter().flat_map(|e| e.ionization_energy());
/// let min = all_values.clone().min_by(|a, b| a.total_cmp(&b)).unwrap();
/// let max = all_values.max_by(|a, b| a.total_cmp(&b)).unwrap();
/// assert_eq!(min..=max, IONIZATION_ENERGY_RANGE);
/// ```
pub const IONIZATION_ENERGY_RANGE: core::ops::RangeInclusive<Electronvolt> =
    Electronvolt(3.894)..=Electronvolt(24.587);

macro_rules! ev {
    ($value:literal) => {
        Some(Electronvolt($value))
    };
}

impl Element {
    /// Returns the element's ionization energy, if available.
    ///
    /// ```
    /// use mendeleev::{Element, Electronvolt};
    /// assert_eq!(Element::H.ionization_energy(), Some(Electronvolt(13.598)));
    /// ```
    pub const fn ionization_energy(&self) -> Option<Electronvolt> {
        use Element as E;
        match self {
            E::H => ev!(13.598),
            E::He => ev!(24.587),
            E::Li => ev!(5.392),
            E::Be => ev!(9.323),
            E::B => ev!(8.298),
            E::C => ev!(11.260),
            E::N => ev!(14.534),
            E::O => ev!(13.618),
            E::F => ev!(17.423),
            E::Ne => ev!(21.565),
            E::Na => ev!(5.139),
            E::Mg => ev!(7.646),
            E::Al => ev!(5.986),
            E::Si => ev!(8.152),
            E::P => ev!(10.487),
            E::S => ev!(10.360),
            E::Cl => ev!(12.968),
            E::Ar => ev!(15.760),
            E::K => ev!(4.341),
            E::Ca => ev!(6.113),
            E::Sc => ev!(6.561),
            E::Ti => ev!(6.828),
            E::V => ev!(6.746),
            E::Cr => ev!(6.767),
            E::Mn => ev!(7.434),
            E::Fe => ev!(7.902),
            E::Co => ev!(7.881),
            E::Ni => ev!(7.640),
            E::Cu => ev!(7.726),
            E::Zn => ev!(9.394),
            E::Ga => ev!(5.999),
            E::Ge => ev!(7.900),
            E::As => ev!(9.815),
            E::Se => ev!(9.752),
            E::Br => ev!(11.814),
            E::Kr => ev!(14.000),
            E::Rb => ev!(4.177),
            E::Sr => ev!(5.695),
            E::Y => ev!(6.217),
            E::Zr => ev!(6.634),
            E::Nb => ev!(6.759),
            E::Mo => ev!(7.092),
            E::Tc => ev!(7.28),
            E::Ru => ev!(7.361),
            E::Rh => ev!(7.459),
            E::Pd => ev!(8.337),
            E::Ag => ev!(7.576),
            E::Cd => ev!(8.994),
            E::In => ev!(5.786),
            E::Sn => ev!(7.344),
            E::Sb => ev!(8.64),
            E::Te => ev!(9.010),
            E::I => ev!(10.451),
            E::Xe => ev!(12.130),
            E::Cs => ev!(3.894),
            E::Ba => ev!(5.212),
            E::La => ev!(5.577),
            E::Ce => ev!(5.539),
            E::Pr => ev!(5.464),
            E::Nd => ev!(5.525),
            E::Pm => ev!(5.55),
            E::Sm => ev!(5.644),
            E::Eu => ev!(5.670),
            E::Gd => ev!(6.150),
            E::Tb => ev!(5.864),
            E::Dy => ev!(5.939),
            E::Ho => ev!(6.022),
            E::Er => ev!(6.108),
            E::Tm => ev!(6.184),
            E::Yb => ev!(6.254),
            E::Lu => ev!(5.426),
            E::Hf => ev!(6.825),
            E::Ta => ev!(7.89),
            E::W => ev!(7.98),
            E::Re => ev!(7.88),
            E::Os => ev!(8.7),
            E::Ir => ev!(9.1),
            E::Pt => ev!(9.0),
            E::Au => ev!(9.226),
            E::Hg => ev!(10.438),
            E::Tl => ev!(6.108),
            E::Pb => ev!(7.417),
            E::Bi => ev!(7.289),
            E::Po => ev!(8.417),
            E::At => ev!(9.5),
            E::Rn => ev!(10.745),
            E::Fr => ev!(3.9),
            E::Ra => ev!(5.279),
            E::Ac => ev!(5.17),
            E::Th => ev!(6.08),
            E::Pa => ev!(5.89),
            E::U => ev!(6.194),
            E::Np => ev!(6.266),
            E::Pu => ev!(6.06),
            E::Am => ev!(5.993),
            E::Cm => ev!(6.02),
            E::Bk => ev!(6.23),
            E::Cf => ev!(6.30),
            E::Es => ev!(6.42),
            E::Fm => ev!(6.50),
            E::Md => ev!(6.58),
            E::No => ev!(6.65),
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
