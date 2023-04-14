use mendeleev::{Element, Isotope};

/// For each element, print all its naturally occurring isotopes and their abundance
fn main() {
    for element in Element::list() {
        print!("{}: ", element.name());
        let isotope_count = Isotope::list()
            .iter()
            .filter(|isotope| &isotope.element() == element)
            .flat_map(|isotope| {
                let abundance = isotope.natural_abundance()?;
                print!("{}: {}, ", isotope.display_with_superscript(), abundance);
                Some(())
            })
            .count();
        if isotope_count == 0 {
            print!("No naturally occurring isotopes");
        }
        println!();
    }
}
