use super::Element;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Electron subshell type, based on the azimuthal quantum number ℓ
pub enum SubshellLabel {
    /// ℓ = 0, historical name "Sharp"
    S,
    /// ℓ = 1, historical name "Principal"
    P,
    /// ℓ = 2, historical name "Diffuse"
    D,
    /// ℓ = 3, historical name "Fundamental"
    F,
    /// ℓ = 4, no historical name
    G,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A subshell (s, p, d, or f) in the electronic configuration
pub struct Subshell {
    /// The shell's principal quantum number
    pub shell_number: u32,
    /// The subshell label letter, based on its azimuthal quantum number
    pub subshell_label: SubshellLabel,
    /// The number of electrons in this subshell for a particular atom
    pub number_of_electrons: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The electronic configuration of an atom
pub struct ElectronicConfiguration {
    /// The noble gas of the preceding period, if any
    pub noble_gas: Option<Element>,
    /// The subshells in the valence shell
    pub valence_subshells: &'static [Subshell],
}

impl core::fmt::Display for SubshellLabel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            SubshellLabel::S => "s",
            SubshellLabel::P => "p",
            SubshellLabel::D => "d",
            SubshellLabel::F => "f",
            SubshellLabel::G => "g",
        })
    }
}

impl core::fmt::Display for Subshell {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.number_of_electrons == 1 {
            formatter.write_fmt(format_args!("{}{}", self.shell_number, self.subshell_label))?;
        } else {
            formatter.write_fmt(format_args!(
                "{}{}{}",
                self.shell_number,
                self.subshell_label,
                crate::superscript::Superscript::new(self.number_of_electrons)
            ))?;
        }
        Ok(())
    }
}

#[cfg(feature = "symbol")]
impl core::fmt::Display for ElectronicConfiguration {
    /// Formats the electronic configuration according to the standard notation, with superscripts
    /// in utf8.
    ///
    /// ```
    /// use mendeleev::Element;
    ///
    /// assert_eq!(Element::H.electronic_configuration().to_string(), "1s");
    /// assert_eq!(Element::He.electronic_configuration().to_string(), "1s²");
    /// assert_eq!(Element::Si.electronic_configuration().to_string(), "[Ne] 3s² 3p²");
    /// ```
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        if let Some(element) = self.noble_gas {
            formatter.write_fmt(format_args!("[{}]", element.symbol()))?;
            first = false;
        }
        for subshell in self.valence_subshells {
            if !first {
                formatter.write_str(" ")?;
            }
            formatter.write_fmt(format_args!("{}", subshell,))?;
            first = false;
        }
        Ok(())
    }
}

macro_rules! ec {
    ($subshells:expr) => {
        ElectronicConfiguration {
            noble_gas: None,
            valence_subshells: &$subshells,
        }
    };
    ($gas:ident, $subshells:expr) => {
        ElectronicConfiguration {
            noble_gas: Some(Element::$gas),
            valence_subshells: &$subshells,
        }
    };
}

macro_rules! s {
    ($n:expr, $t:ident, $e:expr) => {
        Subshell {
            shell_number: $n,
            subshell_label: SubshellLabel::$t,
            number_of_electrons: $e,
        }
    };
}

impl Element {
    /// Returns the element's electronic configuration.
    ///
    /// ```
    /// use mendeleev::{Element, SubshellLabel};
    ///
    /// let configuration = Element::Li.electronic_configuration();
    /// assert_eq!(configuration.noble_gas, Some(Element::He));
    /// assert_eq!(configuration.valence_subshells[0].shell_number, 2);
    /// assert_eq!(configuration.valence_subshells[0].subshell_label, SubshellLabel::S);
    /// assert_eq!(configuration.valence_subshells[0].number_of_electrons, 1);
    /// ```
    pub const fn electronic_configuration(&self) -> ElectronicConfiguration {
        use Element as E;
        match self {
            E::H => ec!([s!(1, S, 1),]),
            E::He => ec!([s!(1, S, 2),]),
            E::Li => ec!(He, [s!(2, S, 1),]),
            E::Be => ec!(He, [s!(2, S, 2),]),
            E::B => ec!(He, [s!(2, S, 2), s!(2, P, 1),]),
            E::C => ec!(He, [s!(2, S, 2), s!(2, P, 2),]),
            E::N => ec!(He, [s!(2, S, 2), s!(2, P, 3),]),
            E::O => ec!(He, [s!(2, S, 2), s!(2, P, 4),]),
            E::F => ec!(He, [s!(2, S, 2), s!(2, P, 5),]),
            E::Ne => ec!(He, [s!(2, S, 2), s!(2, P, 6),]),
            E::Na => ec!(Ne, [s!(3, S, 1),]),
            E::Mg => ec!(Ne, [s!(3, S, 2),]),
            E::Al => ec!(Ne, [s!(3, S, 2), s!(3, P, 1),]),
            E::Si => ec!(Ne, [s!(3, S, 2), s!(3, P, 2),]),
            E::P => ec!(Ne, [s!(3, S, 2), s!(3, P, 3),]),
            E::S => ec!(Ne, [s!(3, S, 2), s!(3, P, 4),]),
            E::Cl => ec!(Ne, [s!(3, S, 2), s!(3, P, 5),]),
            E::Ar => ec!(Ne, [s!(3, S, 2), s!(3, P, 6),]),
            E::K => ec!(Ar, [s!(4, S, 1),]),
            E::Ca => ec!(Ar, [s!(4, S, 2),]),
            E::Sc => ec!(Ar, [s!(3, D, 1), s!(4, S, 2),]),
            E::Ti => ec!(Ar, [s!(3, D, 2), s!(4, S, 2),]),
            E::V => ec!(Ar, [s!(3, D, 3), s!(4, S, 2),]),
            E::Cr => ec!(Ar, [s!(3, D, 5), s!(4, S, 1),]),
            E::Mn => ec!(Ar, [s!(3, D, 5), s!(4, S, 2),]),
            E::Fe => ec!(Ar, [s!(3, D, 6), s!(4, S, 2),]),
            E::Co => ec!(Ar, [s!(3, D, 7), s!(4, S, 2),]),
            E::Ni => ec!(Ar, [s!(3, D, 8), s!(4, S, 2),]),
            E::Cu => ec!(Ar, [s!(3, D, 10), s!(4, S, 1),]),
            E::Zn => ec!(Ar, [s!(3, D, 10), s!(4, S, 2),]),
            E::Ga => ec!(Ar, [s!(3, D, 10), s!(4, S, 2), s!(4, P, 1),]),
            E::Ge => ec!(Ar, [s!(3, D, 10), s!(4, S, 2), s!(4, P, 2),]),
            E::As => ec!(Ar, [s!(3, D, 10), s!(4, S, 2), s!(4, P, 3),]),
            E::Se => ec!(Ar, [s!(3, D, 10), s!(4, S, 2), s!(4, P, 4),]),
            E::Br => ec!(Ar, [s!(3, D, 10), s!(4, S, 2), s!(4, P, 5),]),
            E::Kr => ec!(Ar, [s!(3, D, 10), s!(4, S, 2), s!(4, P, 6),]),
            E::Rb => ec!(Kr, [s!(5, S, 1),]),
            E::Sr => ec!(Kr, [s!(5, S, 2),]),
            E::Y => ec!(Kr, [s!(4, D, 1), s!(5, S, 2),]),
            E::Zr => ec!(Kr, [s!(4, D, 2), s!(5, S, 2),]),
            E::Nb => ec!(Kr, [s!(4, D, 4), s!(5, S, 1),]),
            E::Mo => ec!(Kr, [s!(4, D, 5), s!(5, S, 1),]),
            E::Tc => ec!(Kr, [s!(4, D, 5), s!(5, S, 2),]),
            E::Ru => ec!(Kr, [s!(4, D, 7), s!(5, S, 1),]),
            E::Rh => ec!(Kr, [s!(4, D, 8), s!(5, S, 1),]),
            E::Pd => ec!(Kr, [s!(4, D, 10),]),
            E::Ag => ec!(Kr, [s!(4, D, 10), s!(5, S, 1),]),
            E::Cd => ec!(Kr, [s!(4, D, 10), s!(5, S, 2),]),
            E::In => ec!(Kr, [s!(4, D, 10), s!(5, S, 2), s!(5, P, 1),]),
            E::Sn => ec!(Kr, [s!(4, D, 10), s!(5, S, 2), s!(5, P, 2),]),
            E::Sb => ec!(Kr, [s!(4, D, 10), s!(5, S, 2), s!(5, P, 3),]),
            E::Te => ec!(Kr, [s!(4, D, 10), s!(5, S, 2), s!(5, P, 4),]),
            E::I => ec!(Kr, [s!(4, D, 10), s!(5, S, 2), s!(5, P, 5),]),
            E::Xe => ec!(Kr, [s!(4, D, 10), s!(5, S, 2), s!(5, P, 6),]),
            E::Cs => ec!(Xe, [s!(6, S, 1),]),
            E::Ba => ec!(Xe, [s!(6, S, 2),]),
            E::La => ec!(Xe, [s!(5, D, 1), s!(6, S, 2),]),
            E::Ce => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2),]),
            E::Pr => ec!(Xe, [s!(4, F, 3), s!(6, S, 2),]),
            E::Nd => ec!(Xe, [s!(4, F, 4), s!(6, S, 2),]),
            E::Pm => ec!(Xe, [s!(4, F, 5), s!(6, S, 2),]),
            E::Sm => ec!(Xe, [s!(4, F, 6), s!(6, S, 2),]),
            E::Eu => ec!(Xe, [s!(4, F, 7), s!(6, S, 2),]),
            E::Gd => ec!(Xe, [s!(4, F, 7), s!(5, D, 1), s!(6, S, 2),]),
            E::Tb => ec!(Xe, [s!(4, F, 9), s!(6, S, 2),]),
            E::Dy => ec!(Xe, [s!(4, F, 10), s!(6, S, 2),]),
            E::Ho => ec!(Xe, [s!(4, F, 11), s!(6, S, 2),]),
            E::Er => ec!(Xe, [s!(4, F, 12), s!(6, S, 2),]),
            E::Tm => ec!(Xe, [s!(4, F, 13), s!(6, S, 2),]),
            E::Yb => ec!(Xe, [s!(4, F, 14), s!(6, S, 2),]),
            E::Lu => ec!(Xe, [s!(4, F, 14), s!(5, D, 1), s!(6, S, 2),]),
            E::Hf => ec!(Xe, [s!(4, F, 14), s!(5, D, 2), s!(6, S, 2),]),
            E::Ta => ec!(Xe, [s!(4, F, 14), s!(5, D, 3), s!(6, S, 2),]),
            E::W => ec!(Xe, [s!(4, F, 14), s!(5, D, 4), s!(6, S, 2),]),
            E::Re => ec!(Xe, [s!(4, F, 14), s!(5, D, 5), s!(6, S, 2),]),
            E::Os => ec!(Xe, [s!(4, F, 14), s!(5, D, 6), s!(6, S, 2),]),
            E::Ir => ec!(Xe, [s!(4, F, 14), s!(5, D, 7), s!(6, S, 2),]),
            E::Pt => ec!(Xe, [s!(4, F, 14), s!(5, D, 9), s!(6, S, 1),]),
            E::Au => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 1),]),
            E::Hg => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 2),]),
            E::Tl => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 2), s!(6, P, 1),]),
            E::Pb => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 2), s!(6, P, 2),]),
            E::Bi => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 2), s!(6, P, 3),]),
            E::Po => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 2), s!(6, P, 4),]),
            E::At => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 2), s!(6, P, 5),]),
            E::Rn => ec!(Xe, [s!(4, F, 14), s!(5, D, 10), s!(6, S, 2), s!(6, P, 6),]),
            E::Fr => ec!(Rn, [s!(7, S, 1),]),
            E::Ra => ec!(Rn, [s!(7, S, 2),]),
            E::Ac => ec!(Rn, [s!(6, D, 1), s!(7, S, 2),]),
            E::Th => ec!(Rn, [s!(6, D, 2), s!(7, S, 2),]),
            E::Pa => ec!(Rn, [s!(5, F, 2), s!(6, D, 1), s!(7, S, 2),]),
            E::U => ec!(Rn, [s!(5, F, 3), s!(6, D, 1), s!(7, S, 2),]),
            E::Np => ec!(Rn, [s!(5, F, 4), s!(6, D, 1), s!(7, S, 2),]),
            E::Pu => ec!(Rn, [s!(5, F, 6), s!(7, S, 2),]),
            E::Am => ec!(Rn, [s!(5, F, 7), s!(7, S, 2),]),
            E::Cm => ec!(Rn, [s!(5, F, 7), s!(6, D, 1), s!(7, S, 2),]),
            E::Bk => ec!(Rn, [s!(5, F, 9), s!(7, S, 2),]),
            E::Cf => ec!(Rn, [s!(5, F, 10), s!(7, S, 2),]),
            E::Es => ec!(Rn, [s!(5, F, 11), s!(7, S, 2),]),
            E::Fm => ec!(Rn, [s!(5, F, 12), s!(7, S, 2),]),
            E::Md => ec!(Rn, [s!(5, F, 13), s!(7, S, 2),]),
            E::No => ec!(Rn, [s!(5, F, 14), s!(7, S, 2),]),
            E::Lr => ec!(Rn, [s!(5, F, 14), s!(7, S, 2), s!(7, P, 1),]),
            E::Rf => ec!(Rn, [s!(5, F, 14), s!(6, D, 2), s!(7, S, 2),]),
            E::Db => ec!(Rn, [s!(5, F, 14), s!(6, D, 3), s!(7, S, 2),]),
            E::Sg => ec!(Rn, [s!(5, F, 14), s!(6, D, 4), s!(7, S, 2),]),
            E::Bh => ec!(Rn, [s!(5, F, 14), s!(6, D, 5), s!(7, S, 2),]),
            E::Hs => ec!(Rn, [s!(5, F, 14), s!(6, D, 6), s!(7, S, 2),]),
            E::Mt => ec!(Rn, [s!(5, F, 14), s!(6, D, 7), s!(7, S, 2),]),
            E::Ds => ec!(Rn, [s!(5, F, 14), s!(6, D, 9), s!(7, S, 1),]),
            E::Rg => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 1),]),
            E::Cn => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 2),]),
            E::Nh => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 2), s!(7, P, 1),]),
            E::Fl => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 2), s!(7, P, 2),]),
            E::Mc => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 2), s!(7, P, 3),]),
            E::Lv => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 2), s!(7, P, 4),]),
            E::Ts => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 2), s!(7, P, 5),]),
            E::Og => ec!(Rn, [s!(5, F, 14), s!(6, D, 10), s!(7, S, 2), s!(7, P, 6),]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ElectronicConfiguration, Element};

    fn electron_count(element: Element) -> u32 {
        let ElectronicConfiguration {
            noble_gas,
            valence_subshells,
        } = element.electronic_configuration();
        valence_subshells
            .iter()
            .map(|s| s.number_of_electrons)
            .sum::<u32>()
            + noble_gas.map_or(0, electron_count)
    }
    #[test]
    fn electron_count_matches_atomic_number() {
        for element in Element::iter() {
            assert_eq!(element.atomic_number(), electron_count(element));
        }
    }
}
