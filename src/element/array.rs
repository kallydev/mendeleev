use super::Element;
use super::N_ELEMENTS;

/// Array containing all elements, ordered by atomic number
///
/// ```
/// use mendeleev::Element;
/// use mendeleev::ALL_ELEMENTS;
/// use mendeleev::N_ELEMENTS;
///
/// for n in 1..=N_ELEMENTS {
///     assert_eq!(ALL_ELEMENTS[n - 1].atomic_number() as usize, n);
/// }
/// ```
pub const ALL_ELEMENTS: [Element; N_ELEMENTS] = [
    Element::H,
    Element::He,
    Element::Li,
    Element::Be,
    Element::B,
    Element::C,
    Element::N,
    Element::O,
    Element::F,
    Element::Ne,
    Element::Na,
    Element::Mg,
    Element::Al,
    Element::Si,
    Element::P,
    Element::S,
    Element::Cl,
    Element::Ar,
    Element::K,
    Element::Ca,
    Element::Sc,
    Element::Ti,
    Element::V,
    Element::Cr,
    Element::Mn,
    Element::Fe,
    Element::Co,
    Element::Ni,
    Element::Cu,
    Element::Zn,
    Element::Ga,
    Element::Ge,
    Element::As,
    Element::Se,
    Element::Br,
    Element::Kr,
    Element::Rb,
    Element::Sr,
    Element::Y,
    Element::Zr,
    Element::Nb,
    Element::Mo,
    Element::Tc,
    Element::Ru,
    Element::Rh,
    Element::Pd,
    Element::Ag,
    Element::Cd,
    Element::In,
    Element::Sn,
    Element::Sb,
    Element::Te,
    Element::I,
    Element::Xe,
    Element::Cs,
    Element::Ba,
    Element::La,
    Element::Ce,
    Element::Pr,
    Element::Nd,
    Element::Pm,
    Element::Sm,
    Element::Eu,
    Element::Gd,
    Element::Tb,
    Element::Dy,
    Element::Ho,
    Element::Er,
    Element::Tm,
    Element::Yb,
    Element::Lu,
    Element::Hf,
    Element::Ta,
    Element::W,
    Element::Re,
    Element::Os,
    Element::Ir,
    Element::Pt,
    Element::Au,
    Element::Hg,
    Element::Tl,
    Element::Pb,
    Element::Bi,
    Element::Po,
    Element::At,
    Element::Rn,
    Element::Fr,
    Element::Ra,
    Element::Ac,
    Element::Th,
    Element::Pa,
    Element::U,
    Element::Np,
    Element::Pu,
    Element::Am,
    Element::Cm,
    Element::Bk,
    Element::Cf,
    Element::Es,
    Element::Fm,
    Element::Md,
    Element::No,
    Element::Lr,
    Element::Rf,
    Element::Db,
    Element::Sg,
    Element::Bh,
    Element::Hs,
    Element::Mt,
    Element::Ds,
    Element::Rg,
    Element::Cn,
    Element::Nh,
    Element::Fl,
    Element::Mc,
    Element::Lv,
    Element::Ts,
    Element::Og,
];

impl Element {
    /// Slice containing all elements, ordered by atomic number
    ///
    /// ```
    /// use mendeleev::Element;
    /// use mendeleev::N_ELEMENTS;
    ///
    /// for n in 1..=N_ELEMENTS {
    ///     assert_eq!(Element::list()[n - 1].atomic_number() as usize, n);
    /// }
    /// ```
    pub const fn list() -> &'static [Self] {
        &ALL_ELEMENTS
    }

    /// Returns an iterator that yields all the elements by value, ordered by atomic number
    ///
    /// ```
    /// use mendeleev::Element;
    ///
    /// assert_eq!(Element::iter().next(), Some(Element::H));
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> + Clone {
        ALL_ELEMENTS.into_iter()
    }
}
