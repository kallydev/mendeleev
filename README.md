# Mendeleev

Mendeleev is a crate containing all known chemical elements as an enum
and as a list, as well as methods that return some properties for each
of them.

## Features

- No unsafe code
- No dependencies
- All properties are `const` or `'static`
- Most properties implement `Display`
- No `build.rs` file to parse JSON or CSV data, to improve IDE support.
- Element properties as method behind feature flags, to reduce binary
  size and compilation time when not all properties are needed
- Documentation, tests, and examples.

The data from this crate comes from the PyPI package
[mendeleev](https://pypi.org/project/mendeleev), and some of it was
verified from other sources. The end goal is to port all the data from
that package and more, and make it all available as compile-time
constants and selectable via feature flags.
