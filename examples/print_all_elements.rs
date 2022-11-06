use mendeleev::Element;

/// Prints all the elements and their properties to stdout, as a tab-separated table
fn main() {
    let columns: Vec<(&str, Box<dyn Fn(&Element) -> String>)> = vec![
        (
            "Number",
            Box::new(|e: &Element| e.atomic_number().to_string()),
        ),
        ("Symbol", Box::new(|e: &Element| e.symbol().to_string())),
        (
            "Name      ",
            Box::new(|e: &Element| format!("{:<10}", e.name())),
        ),
        (
            "Year",
            Box::new(|e: &Element| e.year_discovered().to_string()),
        ),
        (
            "CPK color",
            Box::new(|e: &Element| {
                format!(
                    "{:<10}",
                    e.cpk_color()
                        .map(|c| c.to_string())
                        .unwrap_or("".to_string()),
                )
            }),
        ),
        (
            "Jmol color",
            Box::new(|e: &Element| {
                format!(
                    "{:<10}",
                    e.jmol_color()
                        .map(|c| c.to_string())
                        .unwrap_or("".to_string()),
                )
            }),
        ),
        (
            "Atomic Weight",
            Box::new(|e: &Element| format!("{:<10}", e.atomic_weight().to_string())),
        ),
        (
            "Atomic Radius",
            Box::new(|e: &Element| {
                format!(
                    "{:10}",
                    e.atomic_radius()
                        .map(|r| r.to_string())
                        .unwrap_or("".to_string())
                )
            }),
        ),
        ("Period", Box::new(|e: &Element| e.period().to_string())),
        (
            "Group",
            Box::new(|e: &Element| {
                format!("{:<10}", e.group().map(|g| g.group_symbol()).unwrap_or(""))
            }),
        ),
    ];
    println!(
        "{}",
        columns
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join("\t")
    );
    for element in Element::list() {
        println!(
            "{}",
            columns
                .iter()
                .map(|(_, prop)| prop(&element))
                .collect::<Vec<_>>()
                .join("\t")
        );
    }
}
