use super::Element;

macro_rules! prop {
    ($mod:ident, $feature:expr) => {
        #[cfg(feature = $feature)]
        pub mod $mod;
        #[cfg(feature = $feature)]
        pub use $mod::*;
    };
}

prop!(atomic_number, "atomic_number");
prop!(atomic_radius, "atomic_radius");
prop!(atomic_weight, "atomic_weight");
prop!(color, "color");
prop!(cpk_color, "cpk_color");
prop!(jmol_color, "jmol_color");
prop!(name, "name");
prop!(symbol, "symbol");
prop!(year_discovered, "year_discovered");
