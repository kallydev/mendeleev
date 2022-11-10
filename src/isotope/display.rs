use super::Isotope;

impl Isotope {
    /// Returns the isotope represented as a string using the
    /// element's name in English and the mass number.
    ///
    /// ```
    /// use mendeleev::Isotope;
    /// assert_eq!(Isotope::H1.display_with_name(), "Hydrogen-1");
    /// ```
    pub fn display_with_name(&self) -> String {
        format!("{}-{}", self.element().name(), self.mass_number())
    }

    /// Returns the isotope represented as a string using the
    /// element's chemical symbol and the mass number.
    ///
    /// ```
    /// use mendeleev::Isotope;
    /// assert_eq!(Isotope::H1.display_with_symbol(), "H-1");
    /// ```
    pub fn display_with_symbol(&self) -> String {
        format!("{}-{}", self.element().symbol(), self.mass_number())
    }

    /// Returns the isotope represented as a string using the
    /// element's chemical symbol and the mass number as a UTF-8\
    /// superscript.
    ///
    /// ```
    /// use mendeleev::Isotope;
    /// assert_eq!(Isotope::H1.display_with_superscript(), "¹H");
    /// ```
    pub fn display_with_superscript(&self) -> String {
        format!(
            "{}{}",
            to_superscript(self.mass_number()),
            self.element().symbol()
        )
    }
}

fn to_superscript(number: u32) -> String {
    number
        .to_string()
        .chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            c => c,
        })
        .collect()
}
