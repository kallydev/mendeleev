use super::Group;

impl Group {
    /// The group's trivial name, if any
    ///
    /// ```
    /// use mendeleev::Group;
    /// assert_eq!(Group::IA.group_name(), Some("Alkali metals"));
    /// assert_eq!(Group::VIIIA.group_name(), Some("Noble gases"));
    /// assert_eq!(Group::VIIIB8.group_name(), None);
    /// ```
    pub const fn group_name(&self) -> Option<&'static str> {
        match self {
            Group::IA => Some("Alkali metals"),
            Group::IIA => Some("Alkaline earths"),
            Group::IIIB => None,
            Group::IVB => None,
            Group::VB => None,
            Group::VIB => None,
            Group::VIIB => None,
            Group::VIIIB8 => None,
            Group::VIIIB9 => None,
            Group::VIIIB10 => None,
            Group::IB => Some("Coinage metals"),
            Group::IIB => None,
            Group::IIIA => Some("Boron group"),
            Group::IVA => Some("Carbon group"),
            Group::VA => Some("Pnictogens"),
            Group::VIA => Some("Chalcogens"),
            Group::VIIA => Some("Halogens"),
            Group::VIIIA => Some("Noble gases"),
        }
    }
}
