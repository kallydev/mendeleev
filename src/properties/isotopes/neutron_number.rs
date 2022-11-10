use super::Isotope;

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
