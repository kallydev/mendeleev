use super::Isotope;

#[cfg(feature = "ranges")]
/// Range from the minimum to the maximum neutron number across all isotopes
///
/// Convenience constant to avoid writing the code below when this range is needed:
///
/// ```
/// use mendeleev::{Isotope, ISOTOPE_NEUTRON_NUMBER_RANGE};
/// let all_values = Isotope::list().iter().map(|e| e.neutron_number());
/// let min = all_values.clone().min().unwrap();
/// let max = all_values.max().unwrap();
/// assert_eq!(min..=max, ISOTOPE_NEUTRON_NUMBER_RANGE);
/// ```
pub const ISOTOPE_NEUTRON_NUMBER_RANGE: std::ops::RangeInclusive<u32> = 0..=177;

impl Isotope {
    /// Returns the neutron number of the isotope,
    /// i.e., the number of neutrons in its nucleus
    ///
    /// ```
    /// use mendeleev::Isotope;
    /// assert_eq!(Isotope::H1.neutron_number(), 0);
    /// assert_eq!(Isotope::Og295.neutron_number(), 177);
    /// ```
    pub const fn neutron_number(&self) -> u32 {
        self.mass_number()
            .saturating_sub(self.element().atomic_number())
    }
}
