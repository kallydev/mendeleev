use core::fmt::{Display, Formatter};

use super::Element;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum known year of discovery across all elements
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Element, YearDiscovered, YEAR_DISCOVERED_RANGE};
/// let all_values = Element::list().iter().flat_map(|e| match e.year_discovered() {
/// YearDiscovered::Known(year) => Some(year),
/// _ => None
/// });
/// let min = all_values.clone().min().unwrap();
/// let max = all_values.max().unwrap();
/// assert_eq!(min..=max, YEAR_DISCOVERED_RANGE);
/// ```
pub const YEAR_DISCOVERED_RANGE: core::ops::RangeInclusive<u16> = 1669..=2010;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The year in which an element was discovered, if known
pub enum YearDiscovered {
    /// Element known since ancient times, year of discovery not known
    Ancient,
    /// Common Era year in which the element was discovered
    Known(u16),
}
type Year = YearDiscovered;

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Year::Ancient => f.write_str("Ancient"),
            Year::Known(year) => year.fmt(f),
        }
    }
}

impl Element {
    /// The year in which the element was discovered, if known.
    ///
    /// ```
    /// use mendeleev::{Element, YearDiscovered};
    ///
    /// assert_eq!(Element::H.year_discovered(), YearDiscovered::Known(1766));
    /// assert_eq!(Element::Og.year_discovered(), YearDiscovered::Known(2002));
    /// assert_eq!(Element::Au.year_discovered(), YearDiscovered::Ancient);
    /// ```
    pub const fn year_discovered(&self) -> Year {
        match self {
            Element::H => Year::Known(1766),
            Element::He => Year::Known(1868),
            Element::Li => Year::Known(1817),
            Element::Be => Year::Known(1798),
            Element::B => Year::Known(1807),
            Element::C => Year::Ancient,
            Element::N => Year::Known(1772),
            Element::O => Year::Known(1774),
            Element::F => Year::Known(1670),
            Element::Ne => Year::Known(1898),
            Element::Na => Year::Known(1807),
            Element::Mg => Year::Known(1808),
            Element::Al => Year::Ancient,
            Element::Si => Year::Known(1854),
            Element::P => Year::Known(1669),
            Element::S => Year::Ancient,
            Element::Cl => Year::Known(1774),
            Element::Ar => Year::Known(1894),
            Element::K => Year::Known(1807),
            Element::Ca => Year::Ancient,
            Element::Sc => Year::Known(1876),
            Element::Ti => Year::Known(1791),
            Element::V => Year::Known(1803),
            Element::Cr => Year::Ancient,
            Element::Mn => Year::Known(1774),
            Element::Fe => Year::Ancient,
            Element::Co => Year::Ancient,
            Element::Ni => Year::Known(1751),
            Element::Cu => Year::Ancient,
            Element::Zn => Year::Known(1746),
            Element::Ga => Year::Known(1875),
            Element::Ge => Year::Known(1886),
            Element::As => Year::Ancient,
            Element::Se => Year::Known(1817),
            Element::Br => Year::Known(1826),
            Element::Kr => Year::Known(1898),
            Element::Rb => Year::Known(1861),
            Element::Sr => Year::Known(1790),
            Element::Y => Year::Known(1794),
            Element::Zr => Year::Known(1789),
            Element::Nb => Year::Known(1801),
            Element::Mo => Year::Known(1778),
            Element::Tc => Year::Known(1937),
            Element::Ru => Year::Known(1827),
            Element::Rh => Year::Known(1803),
            Element::Pd => Year::Known(1803),
            Element::Ag => Year::Ancient,
            Element::Cd => Year::Known(1817),
            Element::In => Year::Known(1863),
            Element::Sn => Year::Ancient,
            Element::Sb => Year::Ancient,
            Element::Te => Year::Known(1782),
            Element::I => Year::Known(1811),
            Element::Xe => Year::Known(1898),
            Element::Cs => Year::Known(1860),
            Element::Ba => Year::Known(1808),
            Element::La => Year::Known(1839),
            Element::Ce => Year::Known(1803),
            Element::Pr => Year::Known(1885),
            Element::Nd => Year::Known(1885),
            Element::Pm => Year::Known(1947),
            Element::Sm => Year::Known(1853),
            Element::Eu => Year::Known(1901),
            Element::Gd => Year::Known(1880),
            Element::Tb => Year::Known(1843),
            Element::Dy => Year::Known(1886),
            Element::Ho => Year::Known(1878),
            Element::Er => Year::Known(1842),
            Element::Tm => Year::Known(1879),
            Element::Yb => Year::Known(1878),
            Element::Lu => Year::Known(1907),
            Element::Hf => Year::Known(1923),
            Element::Ta => Year::Known(1802),
            Element::W => Year::Known(1783),
            Element::Re => Year::Known(1925),
            Element::Os => Year::Known(1803),
            Element::Ir => Year::Known(1803),
            Element::Pt => Year::Ancient,
            Element::Au => Year::Ancient,
            Element::Hg => Year::Ancient,
            Element::Tl => Year::Known(1861),
            Element::Pb => Year::Ancient,
            Element::Bi => Year::Ancient,
            Element::Po => Year::Known(1898),
            Element::At => Year::Known(1940),
            Element::Rn => Year::Known(1900),
            Element::Fr => Year::Known(1939),
            Element::Ra => Year::Known(1898),
            Element::Ac => Year::Known(1899),
            Element::Th => Year::Known(1828),
            Element::Pa => Year::Known(1913),
            Element::U => Year::Known(1789),
            Element::Np => Year::Known(1940),
            Element::Pu => Year::Known(1940),
            Element::Am => Year::Known(1944),
            Element::Cm => Year::Known(1944),
            Element::Bk => Year::Known(1949),
            Element::Cf => Year::Known(1950),
            Element::Es => Year::Known(1952),
            Element::Fm => Year::Known(1952),
            Element::Md => Year::Known(1955),
            Element::No => Year::Known(1957),
            Element::Lr => Year::Known(1961),
            Element::Rf => Year::Known(1969),
            Element::Db => Year::Known(1967),
            Element::Sg => Year::Known(1974),
            Element::Bh => Year::Known(1976),
            Element::Hs => Year::Known(1984),
            Element::Mt => Year::Known(1982),
            Element::Ds => Year::Known(1994),
            Element::Rg => Year::Known(1994),
            Element::Cn => Year::Known(1996),
            Element::Nh => Year::Known(2003),
            Element::Fl => Year::Known(1998),
            Element::Mc => Year::Known(2003),
            Element::Lv => Year::Known(2000),
            Element::Ts => Year::Known(2010),
            Element::Og => Year::Known(2002),
        }
    }
}
