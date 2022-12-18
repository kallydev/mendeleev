[![Latest version](https://img.shields.io/crates/v/mendeleev.svg)](https://crates.io/crates/mendeleev)
[![pipeline status](https://gitlab.com/ygor.souza/mendeleev/badges/main/pipeline.svg)](https://gitlab.com/ygor.souza/mendeleev/-/commits/main)
[![Documentation](https://docs.rs/mendeleev/badge.svg)](https://docs.rs/mendeleev)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/mendeleev.svg)](https://crates.io/crates/mendeleev)

# Mendeleev

Mendeleev is a crate containing all known chemical elements as an enum
and as a list, as well as methods that return some properties for each
of them.

It also contains most of the known isotopes for each element (naturally
occurring, synthetic, or theoretical), accessible via a similar API as the
elements themselves.

## Available data

### Elements

- Symbol
- Name (in American English)
- Atomic number
- Atomic radius
- Atomic weight
- RGB color in the CPK and Jmol conventions
- Position in the periodic table (period and group)
- Melting and boiling point
- Heat of fusion and evaporation
- Electronic configuration
- Discovery data (year, location, discoverers)

### Isotopes

- Corresponding element
- Relative natural abundance on Earth
- Mass number
- Neutron number

## Features

- No unsafe code
- No dependencies
- All properties are `const` or `'static`
- Compatible with no-std
- Most properties implement `Display` (requires std)
- No `build.rs` file. All the data is directly in the code
- Each of the available properties has its own file that can be excluded from
  the build with a feature flag, to reduce binary size and compilation time
  when not all properties are needed,
- Numeric properties are accompanied by constants providing the range
  of values they can take
- Documentation, tests, and examples

The data from this crate comes from the PyPI package
[mendeleev](https://pypi.org/project/mendeleev), and some of it was
verified from other sources. The end goal is to port all the data from
that package and more, and make it all available as compile-time
constants and selectable via feature flags.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as below, without any additional terms or conditions.

### License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <https://opensource.org/licenses/MIT>)

at your option.
