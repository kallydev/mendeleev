# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2023-04-21

### Added
- Optional support for serde
- New type `Kelvin` for temperature (melting/boiling point)
- New type `KiloJoulePerMole` for heat of fusion/evaporation
- New type `Picometer` for atomic radius
- New type `Percent` for isotope abundance
- New type `GramPerCubicCentimeter` for density
- New type `Electronvolt` for electron affinity and ionization energy
- Density property (from PubChem)
- Electron affinity property (from PubChem)
- Ionization energy property (from PubChem)


### Changed
- `Element::atomic_radius()` return type changed to `Option<Picometer>`
- `Element::melting_point()` return type changed to `Option<Kelvin>`
- `Element::boiling_point()` return type changed to `Option<Kelvin>`
- `Element::fusion_heat()` return type changed to `Option<KiloJoulePerMole>`
- `Element::evaporation_heat()` return type changed to `Option<KiloJoulePerMole>`
- `Isotope::natural_abundance()` return type changed to `Option<Percent>`

## [0.7.0] - 2023-02-06

### Added
- iter() method to iterate over elements, isotopes, and groups directly
- Oxidation states property

### Changed
- Display for YearDiscovered, Color, and Atomic Weight no longer need the std feature

### Deprecated
- Color::html(). It is a trivial method, and the Display implementation already does the same thing

## [0.6.0] - 2022-12-18

### Added
- Support for no-std
- Electronic configuration property
- Discoverers property
- Discovery location property

## [0.5.0] - 2022-11-27

### Added
- Constants for the ranges of numeric properties

## [0.4.0] - 2022-11-22

### Added
- State of matter properties: melting/boiling point, fusion/evaporation heat

## [0.3.0] - 2022-11-10

### Added
- Isotope enum
- Iterable list of isotopes
- Mass number and Neutron number properties
- Relative abundance property
- Methods to display isotope as a string
- Example that prints the relative abundances of all naturally occurring isotopes

### Changed
- Redefined some features

## [0.2.0] - 2022-11-06

### Added
- Period property
- Group property
- Group symbol property
- Group trivial name property
- Example that prints a simple periodic table to the console

### Changed
- Element name and string return static strings

## [0.1.0] - 2022-11-05

Initial release.

### Added
- Element enum
- Iterable list of elements
- Name and symbol properties
- Year discovered property
- Atomic weight property
- Atomic radius property
- CPK and Jmol color properties
- Example that prints all elements and properties to the console

