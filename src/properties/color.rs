#[cfg(feature = "std")]
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A 24-bit color value stored as R, G, and B bytes
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[cfg(feature = "std")]
impl Color {
    /// Returns the html string for this color.
    ///
    /// ```
    /// use mendeleev::{Element, Color};
    ///
    /// let color = Color{r: 0, g: 255, b: 255};
    /// assert_eq!(color.html(), "#00ffff");
    /// ```
    pub fn html(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[cfg(feature = "std")]
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.html())
    }
}

pub(crate) const fn color(r: u8, g: u8, b: u8) -> Option<Color> {
    Some(Color { r, g, b })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "atomic_number")]
    fn generates_html_string() {
        let cases = [
            (Color { r: 0, g: 0, b: 0 }, "#000000"),
            (Color { r: 255, g: 0, b: 0 }, "#ff0000"),
            (Color { r: 0, g: 255, b: 0 }, "#00ff00"),
            (Color { r: 0, g: 0, b: 255 }, "#0000ff"),
        ];
        for (cpk, html) in cases {
            assert_eq!(cpk.html(), html);
        }
    }
}
