use mendeleev::{Element, Group, N_PERIODS};

/// Prints a simple periodic table to stdout, with only the element symbols
fn main() {
    for period in 1..=N_PERIODS {
        for group in Group::list() {
            let element = Element::list()
                .iter()
                .find(|e| e.period() == period && e.group() == Some(*group));
            match element {
                Some(element) => print!("{:<4}", element.symbol()),
                None => print!("    "),
            }
        }
        println!();
    }
    println!();
    for period in 6..=7 {
        print!("        ");
        for element in Element::list()
            .iter()
            .filter(|el| el.period() == period && el.group().is_none())
        {
            print!("{:<4}", element.symbol());
        }
        println!();
    }
}
