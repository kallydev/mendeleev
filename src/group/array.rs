use super::Group;
use super::N_GROUPS;

/// Array containing all periodic table groups, ordered by group number
///
/// ```
/// use mendeleev::Group;
/// use mendeleev::ALL_GROUPS;
/// use mendeleev::N_GROUPS;
///
/// for n in 1..=N_GROUPS {
///     assert_eq!(ALL_GROUPS[n - 1].group_number() as usize, n);
/// }
/// ```
pub const ALL_GROUPS: [Group; N_GROUPS] = [
    Group::IA,
    Group::IIA,
    Group::IIIB,
    Group::IVB,
    Group::VB,
    Group::VIB,
    Group::VIIB,
    Group::VIIIB8,
    Group::VIIIB9,
    Group::VIIIB10,
    Group::IB,
    Group::IIB,
    Group::IIIA,
    Group::IVA,
    Group::VA,
    Group::VIA,
    Group::VIIA,
    Group::VIIIA,
];

impl Group {
    /// Slice containing all groups, ordered by group number
    ///
    /// ```
    /// use mendeleev::Group;
    /// use mendeleev::N_GROUPS;
    ///
    /// for n in 1..=N_GROUPS {
    ///     assert_eq!(Group::list()[n - 1].group_number() as usize, n);
    /// }
    /// ```
    pub const fn list() -> &'static [Self] {
        &ALL_GROUPS
    }
}
