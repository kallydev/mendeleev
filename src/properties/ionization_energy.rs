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
        use Element::*;
        match self {
            H => ev!(13.598),
            He => ev!(24.587),
            Li => ev!(5.392),
            Be => ev!(9.323),
            B => ev!(8.298),
            C => ev!(11.260),
            N => ev!(14.534),
            O => ev!(13.618),
            F => ev!(17.423),
            Ne => ev!(21.565),
            Na => ev!(5.139),
            Mg => ev!(7.646),
            Al => ev!(5.986),
            Si => ev!(8.152),
            P => ev!(10.487),
            S => ev!(10.360),
            Cl => ev!(12.968),
            Ar => ev!(15.760),
            K => ev!(4.341),
            Ca => ev!(6.113),
            Sc => ev!(6.561),
            Ti => ev!(6.828),
            V => ev!(6.746),
            Cr => ev!(6.767),
            Mn => ev!(7.434),
            Fe => ev!(7.902),
            Co => ev!(7.881),
            Ni => ev!(7.640),
            Cu => ev!(7.726),
            Zn => ev!(9.394),
            Ga => ev!(5.999),
            Ge => ev!(7.900),
            As => ev!(9.815),
            Se => ev!(9.752),
            Br => ev!(11.814),
            Kr => ev!(14.000),
            Rb => ev!(4.177),
            Sr => ev!(5.695),
            Y => ev!(6.217),
            Zr => ev!(6.634),
            Nb => ev!(6.759),
            Mo => ev!(7.092),
            Tc => ev!(7.28),
            Ru => ev!(7.361),
            Rh => ev!(7.459),
            Pd => ev!(8.337),
            Ag => ev!(7.576),
            Cd => ev!(8.994),
            In => ev!(5.786),
            Sn => ev!(7.344),
            Sb => ev!(8.64),
            Te => ev!(9.010),
            I => ev!(10.451),
            Xe => ev!(12.130),
            Cs => ev!(3.894),
            Ba => ev!(5.212),
            La => ev!(5.577),
            Ce => ev!(5.539),
            Pr => ev!(5.464),
            Nd => ev!(5.525),
            Pm => ev!(5.55),
            Sm => ev!(5.644),
            Eu => ev!(5.670),
            Gd => ev!(6.150),
            Tb => ev!(5.864),
            Dy => ev!(5.939),
            Ho => ev!(6.022),
            Er => ev!(6.108),
            Tm => ev!(6.184),
            Yb => ev!(6.254),
            Lu => ev!(5.426),
            Hf => ev!(6.825),
            Ta => ev!(7.89),
            W => ev!(7.98),
            Re => ev!(7.88),
            Os => ev!(8.7),
            Ir => ev!(9.1),
            Pt => ev!(9.0),
            Au => ev!(9.226),
            Hg => ev!(10.438),
            Tl => ev!(6.108),
            Pb => ev!(7.417),
            Bi => ev!(7.289),
            Po => ev!(8.417),
            At => ev!(9.5),
            Rn => ev!(10.745),
            Fr => ev!(3.9),
            Ra => ev!(5.279),
            Ac => ev!(5.17),
            Th => ev!(6.08),
            Pa => ev!(5.89),
            U => ev!(6.194),
            Np => ev!(6.266),
            Pu => ev!(6.06),
            Am => ev!(5.993),
            Cm => ev!(6.02),
            Bk => ev!(6.23),
            Cf => ev!(6.30),
            Es => ev!(6.42),
            Fm => ev!(6.50),
            Md => ev!(6.58),
            No => ev!(6.65),
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
