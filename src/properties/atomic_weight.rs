use std::{fmt::Display, ops::RangeInclusive};

use super::Element;

#[derive(Clone, Debug, PartialEq)]
/// The Standard Atomic Weight as defined by the CIAAW
pub enum AtomicWeight {
    /// Value defined as an interval
    Interval {
        /// Interval of atomic weights
        range: RangeInclusive<f64>,
        /// The mean value conventionally used
        conventional: f64,
    },
    /// Value defined with uncertainty
    Uncertainty {
        /// The mean weight value
        weight: f64,
        /// The uncertainty in the weight
        uncertainty: f64,
    },
    /// Atomic weight not known, default to the mass number of the most stable isotope
    MassNumber {
        /// Mass number (number of protons + neutrons)
        number: u64,
    },
}

impl PartialOrd for AtomicWeight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        f64::from(self).partial_cmp(&f64::from(other))
    }
}

impl From<AtomicWeight> for f64 {
    fn from(weight: AtomicWeight) -> Self {
        weight.get_value()
    }
}

impl From<&AtomicWeight> for f64 {
    fn from(weight: &AtomicWeight) -> Self {
        weight.get_value()
    }
}

impl AtomicWeight {
    const fn get_value(&self) -> f64 {
        match self {
            AtomicWeight::Interval {
                range: _,
                conventional,
            } => *conventional,
            AtomicWeight::Uncertainty {
                weight,
                uncertainty: _,
            } => *weight,
            AtomicWeight::MassNumber { number } => *number as f64,
        }
    }
}

impl Display for AtomicWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AtomicWeight::Interval {
                range: _,
                conventional,
            } => format!("{}", conventional),
            AtomicWeight::Uncertainty {
                weight,
                uncertainty,
            } => {
                let (precision, digit) = get_precision(*uncertainty);
                format!("{w:.prec$}({u})", w = weight, prec = precision, u = digit)
            }
            AtomicWeight::MassNumber { number } => format!("[{}]", number),
        };
        f.write_str(&s)?;
        Ok(())
    }
}

fn get_precision(uncertainty: f64) -> (usize, u8) {
    let precision = uncertainty.log10().floor().abs();
    let digit = uncertainty * 10.0f64.powf(precision);
    (precision as usize, digit as u8)
}

const fn bounds(range: RangeInclusive<f64>, conventional: f64) -> AtomicWeight {
    AtomicWeight::Interval {
        range,
        conventional,
    }
}

const fn unc(weight: f64, uncertainty: f64) -> AtomicWeight {
    AtomicWeight::Uncertainty {
        weight,
        uncertainty,
    }
}
const fn mn(number: u64) -> AtomicWeight {
    AtomicWeight::MassNumber { number }
}

impl Element {
    /// Returns the element's Standard Atomic Weight, if applicable,
    /// or its mass number otherwise.
    ///
    /// ```
    /// use mendeleev::{Element, AtomicWeight};
    ///
    /// assert_eq!(Element::H.atomic_weight(),
    /// AtomicWeight::Interval{range: 1.0078..=1.0082, conventional: 1.008});
    /// assert_eq!(Element::Og.atomic_weight(), AtomicWeight::MassNumber{number: 294});
    /// ```
    pub const fn atomic_weight(&self) -> AtomicWeight {
        match self {
            Element::H => bounds(1.0078..=1.0082, 1.008),
            Element::He => unc(4.002602, 0.000002),
            Element::Li => bounds(6.938..=6.997, 6.94),
            Element::Be => unc(9.0121831, 0.0000005),
            Element::B => bounds(10.806..=10.821, 10.81),
            Element::C => bounds(12.009..=12.012, 12.011),
            Element::N => bounds(14.006..=14.008, 14.007),
            Element::O => bounds(15.999..=16.000, 15.999),
            Element::F => unc(18.998403163, 0.000000006),
            Element::Ne => unc(20.1797, 0.0006),
            Element::Na => unc(22.98976928, 0.00000002),
            Element::Mg => bounds(24.304..=24.307, 24.305),
            Element::Al => unc(26.9815385, 0.0000007),
            Element::Si => bounds(28.084..=28.086, 28.085),
            Element::P => unc(30.973761998, 0.000000005),
            Element::S => bounds(32.059..=32.076, 32.06),
            Element::Cl => bounds(35.446..=35.457, 35.45),
            Element::Ar => unc(39.948, 0.001),
            Element::K => unc(39.0983, 0.0001),
            Element::Ca => unc(40.078, 0.004),
            Element::Sc => unc(44.955908, 0.000005),
            Element::Ti => unc(47.867, 0.001),
            Element::V => unc(50.9415, 0.0001),
            Element::Cr => unc(51.9961, 0.0006),
            Element::Mn => unc(54.938044, 0.000003),
            Element::Fe => unc(55.845, 0.002),
            Element::Co => unc(58.933194, 0.000004),
            Element::Ni => unc(58.6934, 0.0004),
            Element::Cu => unc(63.546, 0.003),
            Element::Zn => unc(65.38, 0.02),
            Element::Ga => unc(69.723, 0.001),
            Element::Ge => unc(72.63, 0.008),
            Element::As => unc(74.921595, 0.000006),
            Element::Se => unc(78.971, 0.008),
            Element::Br => bounds(79.901..=79.907, 79.904),
            Element::Kr => unc(83.798, 0.002),
            Element::Rb => unc(85.4678, 0.0003),
            Element::Sr => unc(87.62, 0.01),
            Element::Y => unc(88.90584, 0.00002),
            Element::Zr => unc(91.224, 0.002),
            Element::Nb => unc(92.90637, 0.00002),
            Element::Mo => unc(95.95, 0.01),
            Element::Tc => unc(97.90721, 0.00003),
            Element::Ru => unc(101.07, 0.02),
            Element::Rh => unc(102.9055, 0.00002),
            Element::Pd => unc(106.42, 0.01),
            Element::Ag => unc(107.8682, 0.0002),
            Element::Cd => unc(112.414, 0.004),
            Element::In => unc(114.818, 0.001),
            Element::Sn => unc(118.71, 0.007),
            Element::Sb => unc(121.76, 0.001),
            Element::Te => unc(127.6, 0.03),
            Element::I => unc(126.90447, 0.00003),
            Element::Xe => unc(131.293, 0.006),
            Element::Cs => unc(132.90545196, 0.00000006),
            Element::Ba => unc(137.327, 0.007),
            Element::La => unc(138.90547, 0.00007),
            Element::Ce => unc(140.116, 0.001),
            Element::Pr => unc(140.90766, 0.00002),
            Element::Nd => unc(144.242, 0.003),
            Element::Pm => unc(144.91276, 0.00002),
            Element::Sm => unc(150.36, 0.02),
            Element::Eu => unc(151.964, 0.001),
            Element::Gd => unc(157.25, 0.03),
            Element::Tb => unc(158.92535, 0.00002),
            Element::Dy => unc(162.5, 0.001),
            Element::Ho => unc(164.93033, 0.00002),
            Element::Er => unc(167.259, 0.003),
            Element::Tm => unc(168.93422, 0.00002),
            Element::Yb => unc(173.045, 0.01),
            Element::Lu => unc(174.9668, 0.0001),
            Element::Hf => unc(178.49, 0.02),
            Element::Ta => unc(180.94788, 0.00002),
            Element::W => unc(183.84, 0.01),
            Element::Re => unc(186.207, 0.001),
            Element::Os => unc(190.23, 0.03),
            Element::Ir => unc(192.217, 0.003),
            Element::Pt => unc(195.084, 0.009),
            Element::Au => unc(196.966569, 0.000005),
            Element::Hg => unc(200.592, 0.003),
            Element::Tl => bounds(204.38..=204.39, 204.38),
            Element::Pb => unc(207.2, 0.1),
            Element::Bi => unc(208.9804, 0.00001),
            Element::Po => mn(209),
            Element::At => mn(210),
            Element::Rn => mn(222),
            Element::Fr => mn(223),
            Element::Ra => mn(226),
            Element::Ac => mn(227),
            Element::Th => unc(232.0377, 0.0004),
            Element::Pa => unc(231.03588, 0.00002),
            Element::U => unc(238.02891, 0.00003),
            Element::Np => mn(237),
            Element::Pu => mn(244),
            Element::Am => mn(243),
            Element::Cm => mn(247),
            Element::Bk => mn(247),
            Element::Cf => mn(251),
            Element::Es => mn(252),
            Element::Fm => mn(257),
            Element::Md => mn(258),
            Element::No => mn(259),
            Element::Lr => mn(262),
            Element::Rf => mn(267),
            Element::Db => mn(268),
            Element::Sg => mn(271),
            Element::Bh => mn(274),
            Element::Hs => mn(269),
            Element::Mt => mn(276),
            Element::Ds => mn(281),
            Element::Rg => mn(281),
            Element::Cn => mn(285),
            Element::Nh => mn(286),
            Element::Fl => mn(289),
            Element::Mc => mn(288),
            Element::Lv => mn(293),
            Element::Ts => mn(294),
            Element::Og => mn(294),
        }
    }
}
