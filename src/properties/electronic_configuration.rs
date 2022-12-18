use super::Element;
use super::Element::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// A subshell (s, p, d, or f) in the electronic configuration
pub struct Subshell {
    /// The shell's principal quantum number
    pub shell_number: u32,
    /// The subshell label letter, based on its azimuthal quantum number
    pub subshell_label: SubshellLabel,
    /// The number of electrons in this subshell for a particular atom
    pub number_of_electrons: u32,
}

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

#[cfg(feature = "std")]
impl core::fmt::Display for Subshell {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.number_of_electrons == 1 {
            formatter.write_fmt(format_args!("{}{}", self.shell_number, self.subshell_label))?;
        } else {
            formatter.write_fmt(format_args!(
                "{}{}{}",
                self.shell_number,
                self.subshell_label,
                crate::to_superscript(self.number_of_electrons)
            ))?;
        }
        Ok(())
    }
}

#[cfg(all(feature = "std", feature = "symbol"))]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            noble_gas: Some($gas),
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
        match self {
            H => ec!([s!(1, S, 1),]),
            He => ec!([s!(1, S, 2),]),
            Li => ec!(He, [s!(2, S, 1),]),
            Be => ec!(He, [s!(2, S, 2),]),
            B => ec!(He, [s!(2, S, 2), s!(2, P, 1),]),
            C => ec!(He, [s!(2, S, 2), s!(2, P, 2),]),
            N => ec!(He, [s!(2, S, 2), s!(2, P, 3),]),
            O => ec!(He, [s!(2, S, 2), s!(2, P, 4),]),
            F => ec!(He, [s!(2, S, 2), s!(2, P, 5),]),
            Ne => ec!(He, [s!(2, S, 2), s!(2, P, 6),]),
            Na => ec!(Ne, [s!(3, S, 1),]),
            Mg => ec!(Ne, [s!(3, S, 2),]),
            Al => ec!(Ne, [s!(3, S, 2), s!(3, P, 1),]),
            Si => ec!(Ne, [s!(3, S, 2), s!(3, P, 2),]),
            P => ec!(Ne, [s!(3, S, 2), s!(3, P, 3),]),
            S => ec!(Ne, [s!(3, S, 2), s!(3, P, 4),]),
            Cl => ec!(Ne, [s!(3, S, 2), s!(3, P, 5),]),
            Ar => ec!(Ne, [s!(3, S, 2), s!(3, P, 6),]),
            K => ec!(Ar, [s!(4, S, 1),]),
            Ca => ec!(Ar, [s!(4, S, 2),]),
            Sc => ec!(Ar, [s!(3, D, 1), s!(4, S, 2),]),
            Ti => ec!(Ar, [s!(3, D, 2), s!(4, S, 2),]),
            V => ec!(Ar, [s!(3, D, 3), s!(4, S, 2),]),
            Cr => ec!(Ar, [s!(3, D, 5), s!(4, S, 1),]),
            Mn => ec!(Ar, [s!(3, D, 5), s!(4, S, 2),]),
            Fe => ec!(Ar, [s!(3, D, 6), s!(4, S, 2),]),
            Co => ec!(Ar, [s!(3, D, 7), s!(4, S, 2),]),
            Ni => ec!(Ar, [s!(3, D, 8), s!(4, S, 2),]),
            Cu => ec!(Ar, [s!(3, D, 1), s!(4, S, 1),]),
            Zn => ec!(Ar, [s!(3, D, 1), s!(4, S, 2),]),
            Ga => ec!(Ar, [s!(3, D, 1), s!(4, S, 2), s!(4, P, 1),]),
            Ge => ec!(Ar, [s!(3, D, 1), s!(4, S, 2), s!(4, P, 2),]),
            As => ec!(Ar, [s!(3, D, 1), s!(4, S, 2), s!(4, P, 3),]),
            Se => ec!(Ar, [s!(3, D, 1), s!(4, S, 2), s!(4, P, 4),]),
            Br => ec!(Ar, [s!(3, D, 1), s!(4, S, 2), s!(4, P, 5),]),
            Kr => ec!(Ar, [s!(3, D, 1), s!(4, S, 2), s!(4, P, 6),]),
            Rb => ec!(Kr, [s!(5, S, 1),]),
            Sr => ec!(Kr, [s!(5, S, 2),]),
            Y => ec!(Kr, [s!(4, D, 1), s!(5, S, 2),]),
            Zr => ec!(Kr, [s!(4, D, 2), s!(5, S, 2),]),
            Nb => ec!(Kr, [s!(4, D, 4), s!(5, S, 1),]),
            Mo => ec!(Kr, [s!(4, D, 5), s!(5, S, 1),]),
            Tc => ec!(Kr, [s!(4, D, 5), s!(5, S, 2),]),
            Ru => ec!(Kr, [s!(4, D, 7), s!(5, S, 1),]),
            Rh => ec!(Kr, [s!(4, D, 8), s!(5, S, 1),]),
            Pd => ec!(Kr, [s!(4, D, 1),]),
            Ag => ec!(Kr, [s!(4, D, 1), s!(5, S, 1),]),
            Cd => ec!(Kr, [s!(4, D, 1), s!(5, S, 2),]),
            In => ec!(Kr, [s!(4, D, 1), s!(5, S, 2), s!(5, P, 1),]),
            Sn => ec!(Kr, [s!(4, D, 1), s!(5, S, 2), s!(5, P, 2),]),
            Sb => ec!(Kr, [s!(4, D, 1), s!(5, S, 2), s!(5, P, 3),]),
            Te => ec!(Kr, [s!(4, D, 1), s!(5, S, 2), s!(5, P, 4),]),
            I => ec!(Kr, [s!(4, D, 1), s!(5, S, 2), s!(5, P, 5),]),
            Xe => ec!(Kr, [s!(4, D, 1), s!(5, S, 2), s!(5, P, 6),]),
            Cs => ec!(Xe, [s!(6, S, 1),]),
            Ba => ec!(Xe, [s!(6, S, 2),]),
            La => ec!(Xe, [s!(5, D, 1), s!(6, S, 2),]),
            Ce => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2),]),
            Pr => ec!(Xe, [s!(4, F, 3), s!(6, S, 2),]),
            Nd => ec!(Xe, [s!(4, F, 4), s!(6, S, 2),]),
            Pm => ec!(Xe, [s!(4, F, 5), s!(6, S, 2),]),
            Sm => ec!(Xe, [s!(4, F, 6), s!(6, S, 2),]),
            Eu => ec!(Xe, [s!(4, F, 7), s!(6, S, 2),]),
            Gd => ec!(Xe, [s!(4, F, 7), s!(5, D, 1), s!(6, S, 2),]),
            Tb => ec!(Xe, [s!(4, F, 9), s!(6, S, 2),]),
            Dy => ec!(Xe, [s!(4, F, 1), s!(6, S, 2),]),
            Ho => ec!(Xe, [s!(4, F, 1), s!(6, S, 2),]),
            Er => ec!(Xe, [s!(4, F, 1), s!(6, S, 2),]),
            Tm => ec!(Xe, [s!(4, F, 1), s!(6, S, 2),]),
            Yb => ec!(Xe, [s!(4, F, 1), s!(6, S, 2),]),
            Lu => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2),]),
            Hf => ec!(Xe, [s!(4, F, 1), s!(5, D, 2), s!(6, S, 2),]),
            Ta => ec!(Xe, [s!(4, F, 1), s!(5, D, 3), s!(6, S, 2),]),
            W => ec!(Xe, [s!(4, F, 1), s!(5, D, 4), s!(6, S, 2),]),
            Re => ec!(Xe, [s!(4, F, 1), s!(5, D, 5), s!(6, S, 2),]),
            Os => ec!(Xe, [s!(4, F, 1), s!(5, D, 6), s!(6, S, 2),]),
            Ir => ec!(Xe, [s!(4, F, 1), s!(5, D, 7), s!(6, S, 2),]),
            Pt => ec!(Xe, [s!(4, F, 1), s!(5, D, 9), s!(6, S, 1),]),
            Au => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 1),]),
            Hg => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2),]),
            Tl => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2), s!(6, P, 1),]),
            Pb => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2), s!(6, P, 2),]),
            Bi => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2), s!(6, P, 3),]),
            Po => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2), s!(6, P, 4),]),
            At => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2), s!(6, P, 5),]),
            Rn => ec!(Xe, [s!(4, F, 1), s!(5, D, 1), s!(6, S, 2), s!(6, P, 6),]),
            Fr => ec!(Rn, [s!(7, S, 1),]),
            Ra => ec!(Rn, [s!(7, S, 2),]),
            Ac => ec!(Rn, [s!(6, D, 1), s!(7, S, 2),]),
            Th => ec!(Rn, [s!(6, D, 2), s!(7, S, 2),]),
            Pa => ec!(Rn, [s!(5, F, 2), s!(6, D, 1), s!(7, S, 2),]),
            U => ec!(Rn, [s!(5, F, 3), s!(6, D, 1), s!(7, S, 2),]),
            Np => ec!(Rn, [s!(5, F, 4), s!(6, D, 1), s!(7, S, 2),]),
            Pu => ec!(Rn, [s!(5, F, 6), s!(7, S, 2),]),
            Am => ec!(Rn, [s!(5, F, 7), s!(7, S, 2),]),
            Cm => ec!(Rn, [s!(5, F, 7), s!(6, D, 1), s!(7, S, 2),]),
            Bk => ec!(Rn, [s!(5, F, 9), s!(7, S, 2),]),
            Cf => ec!(Rn, [s!(5, F, 1), s!(7, S, 2),]),
            Es => ec!(Rn, [s!(5, F, 1), s!(7, S, 2),]),
            Fm => ec!(Rn, [s!(5, F, 1), s!(7, S, 2),]),
            Md => ec!(Rn, [s!(5, F, 1), s!(7, S, 2),]),
            No => ec!(Rn, [s!(5, F, 1), s!(7, S, 2),]),
            Lr => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2),]),
            Rf => ec!(Rn, [s!(5, F, 1), s!(6, D, 2), s!(7, S, 2),]),
            Db => ec!(Rn, [s!(5, F, 1), s!(6, D, 3), s!(7, S, 2),]),
            Sg => ec!(Rn, [s!(5, F, 1), s!(6, D, 4), s!(7, S, 2),]),
            Bh => ec!(Rn, [s!(5, F, 1), s!(6, D, 5), s!(7, S, 2),]),
            Hs => ec!(Rn, [s!(5, F, 1), s!(6, D, 6), s!(7, S, 2),]),
            Mt => ec!(Rn, [s!(5, F, 1), s!(6, D, 7), s!(7, S, 2),]),
            Ds => ec!(Rn, [s!(5, F, 1), s!(6, D, 9), s!(7, S, 1),]),
            Rg => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 1),]),
            Cn => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2),]),
            Nh => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2), s!(7, P, 1),]),
            Fl => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2), s!(7, P, 2),]),
            Mc => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2), s!(7, P, 3),]),
            Lv => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2), s!(7, P, 4),]),
            Ts => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2), s!(7, P, 5),]),
            Og => ec!(Rn, [s!(5, F, 1), s!(6, D, 1), s!(7, S, 2), s!(7, P, 6),]),
        }
    }
}
