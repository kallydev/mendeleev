use mendeleev::Element;

/// Prints the electronic configuration for each element in the standard notation.
fn main() {
    for element in Element::list() {
        println!(
            "{}:\t{}",
            element.symbol(),
            element.electronic_configuration()
        );
    }
}
