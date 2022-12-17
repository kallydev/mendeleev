#[allow(unused_imports)]
use super::{Element, Isotope};

#[cfg(feature = "group")]
use super::Group;

macro_rules! prop {
    ($mod:ident, $feature:expr) => {
        #[cfg(feature = $feature)]
        mod $mod;
        #[cfg(feature = $feature)]
        pub use $mod::*;
    };
}

prop!(atomic_number, "atomic_number");
prop!(atomic_radius, "atomic_radius");
prop!(atomic_weight, "atomic_weight");
prop!(color, "color");
prop!(cpk_color, "cpk_color");
prop!(group, "group");
prop!(group_name, "group_name");
prop!(group_number, "group_number");
prop!(group_symbol, "group_symbol");
prop!(jmol_color, "jmol_color");
prop!(name, "name");
prop!(period, "period");
prop!(symbol, "symbol");
prop!(year_discovered, "year_discovered");
prop!(isotopes, "isotopes");
prop!(melting_point, "melting_point");
prop!(boiling_point, "boiling_point");
prop!(fusion_heat, "fusion_heat");
prop!(evaporation_heat, "evaporation_heat");
prop!(electronic_configuration, "electronic_configuration");
prop!(discoverers, "discoverers");
prop!(discovery_location, "discovery_location");
