#[cfg(feature = "group_list")]
mod array;
#[cfg(feature = "group_list")]
pub use array::ALL_GROUPS;

/// The total number of groups in the periodic table
pub const N_GROUPS: usize = 18;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Each group in the periodic table
pub enum Group {
    /// Alkali metals
    IA,
    /// Alkaline earths
    IIA,
    #[doc(hidden)]
    IIIB,
    #[doc(hidden)]
    IVB,
    #[doc(hidden)]
    VB,
    #[doc(hidden)]
    VIB,
    #[doc(hidden)]
    VIIB,
    #[doc(hidden)]
    VIIIB8,
    #[doc(hidden)]
    VIIIB9,
    #[doc(hidden)]
    VIIIB10,
    #[doc(hidden)]
    IB,
    #[doc(hidden)]
    IIB,
    #[doc(hidden)]
    IIIA,
    #[doc(hidden)]
    IVA,
    #[doc(hidden)]
    VA,
    #[doc(hidden)]
    VIA,
    #[doc(hidden)]
    VIIA,
    #[doc(hidden)]
    VIIIA,
}
