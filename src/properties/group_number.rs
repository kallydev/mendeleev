use super::Group;

impl Group {
    /// Returns the group's number in the periodic table.
    ///
    /// ```
    /// use mendeleev::Group;
    /// assert_eq!(Group::IA.group_number(), 1);
    /// assert_eq!(Group::VIIIA.group_number(), 18);
    /// ```
    pub const fn group_number(&self) -> u32 {
        match self {
            Group::IA => 1,
            Group::IIA => 2,
            Group::IIIB => 3,
            Group::IVB => 4,
            Group::VB => 5,
            Group::VIB => 6,
            Group::VIIB => 7,
            Group::VIIIB8 => 8,
            Group::VIIIB9 => 9,
            Group::VIIIB10 => 10,
            Group::IB => 11,
            Group::IIB => 12,
            Group::IIIA => 13,
            Group::IVA => 14,
            Group::VA => 15,
            Group::VIA => 16,
            Group::VIIA => 17,
            Group::VIIIA => 18,
        }
    }
}
