use super::Group;

impl Group {
    /// The group's symbol in the CAS system
    ///
    /// ```
    /// use mendeleev::Group;
    /// assert_eq!(Group::IA.group_symbol(), "IA");
    /// assert_eq!(Group::VIIIA.group_symbol(), "VIIIA");
    /// assert_eq!(Group::VIIIB8.group_symbol(), "VIIIB");
    /// ```
    pub const fn group_symbol(&self) -> &'static str {
        match self {
            Group::IA => "IA",
            Group::IIA => "IIA",
            Group::IIIB => "IIIB",
            Group::IVB => "IVB",
            Group::VB => "VB",
            Group::VIB => "VIB",
            Group::VIIB => "VIIB",
            Group::VIIIB8 => "VIIIB",
            Group::VIIIB9 => "VIIIB",
            Group::VIIIB10 => "VIIIB",
            Group::IB => "IB",
            Group::IIB => "IIB",
            Group::IIIA => "IIIA",
            Group::IVA => "IVA",
            Group::VA => "VA",
            Group::VIA => "VIA",
            Group::VIIA => "VIIA",
            Group::VIIIA => "VIIIA",
        }
    }
}
