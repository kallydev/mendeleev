[![Latest version](https://img.shields.io/crates/v/mendeleev.svg)](https://crates.io/crates/mendeleev)
[![pipeline status](https://gitlab.com/ygor.souza/mendeleev/badges/main/pipeline.svg)](https://gitlab.com/ygor.souza/mendeleev/-/commits/main)
[![Documentation](https://docs.rs/mendeleev/badge.svg)](https://docs.rs/mendeleev)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/mendeleev.svg)](https://crates.io/crates/mendeleev)

# Mendeleev

Mendeleev is a crate containing all known chemical elements as an enum
and as a list, as well as methods that return some properties for each
of them.

### Example

Get data on a specific element

```rust
use mendeleev::Element;

let element = Element::Si;
assert_eq!(element.atomic_number(), 14);
assert_eq!(element.name(), "Silicon");
assert_eq!(format!("{}", element.electronic_configuration()), "[Ne] 3s² 3p²");
```

### Example

Search the list of elements

```rust
use mendeleev::{Element, Kelvin};

// Find the element with the highest value for a given property
let highest_melting_point = Element::iter().reduce(|acc, e| {
    core::cmp::max_by(acc, e, |e1, e2| {
        e1.melting_point()
            .unwrap_or(Kelvin(0.0))
            .total_cmp(&e2.melting_point().unwrap_or(Kelvin(0.0)))
    })
});
assert_eq!(highest_melting_point, Some(Element::C));

// Iterate through the elements with no known year of discovery
let mut ancient_elements = Element::iter()
    .filter(|e| matches!(e.year_discovered(), mendeleev::YearDiscovered::Ancient));
assert_eq!(ancient_elements.next(), Some(Element::C));
assert_eq!(ancient_elements.next(), Some(Element::Al));

// Find an element by name
let iron = Element::iter().find(|e| e.name().eq_ignore_ascii_case("iron"));
assert_eq!(iron, Some(Element::Fe));

```

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
- No required dependencies
- Optional support for `serde`
- All properties are `const` or `'static`
- Compatible with no-std
- Most types implement `Display`
- No `build.rs` file. All the data is directly in the code
- Each of the available properties has its own file that can be excluded from
  the build with a feature flag, to reduce binary size and compilation time
  when not all properties are needed
- Numeric properties are accompanied by constants providing the range
  of values they can take
- Documentation, tests, and examples

## Data sources

The data available in this crate comes from the following sources:

- Density, electron affinity, and ionization energy:
    - National Center for Biotechnology Information (2023). Periodic Table of
      Elements. Retrieved April 15, 2023 from
      <https://pubchem.ncbi.nlm.nih.gov/periodic-table/>.

- Other properties:
    - PyPI package mendeleev, version 0.17.0.
      <https://pypi.org/project/mendeleev/0.17.0/>.

For the data sources used by the PyPI package, see
<https://mendeleev.readthedocs.io/en/latest/data.html>

Note that this crate is not maintained by the same authors as the PyPI package.

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
