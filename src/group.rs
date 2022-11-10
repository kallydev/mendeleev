#[cfg(feature = "group_list")]
pub mod array;
#[cfg(feature = "group_list")]
pub use array::ALL_GROUPS;

/// The total number of known chemical elements
pub const N_GROUPS: usize = 18;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Each group in the periodic table
pub enum Group {
    IA,
    IIA,
    IIIB,
    IVB,
    VB,
    VIB,
    VIIB,
    VIIIB8,
    VIIIB9,
    VIIIB10,
    IB,
    IIB,
    IIIA,
    IVA,
    VA,
    VIA,
    VIIA,
    VIIIA,
}
