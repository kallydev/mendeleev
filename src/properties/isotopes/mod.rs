use super::{Element, Isotope};

macro_rules! prop {
    ($mod:ident, $feature:expr) => {
        #[cfg(feature = $feature)]
        mod $mod;
        #[cfg(feature = $feature)]
        #[allow(unused_imports)]
        pub use $mod::*;
    };
}

prop!(natural_abundance, "isotope_natural_abundance");
prop!(element, "isotope_element");
prop!(mass_number, "isotope_mass_number");
prop!(neutron_number, "isotope_neutron_number");
