use core::fmt::{Display, Formatter};
#[cfg(feature = "std")]
use std::string::{String, ToString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A 24-bit color value stored as R, G, and B bytes
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
}

#[cfg(feature = "std")]
#[deprecated(since = "0.7.0", note = "Use color.to_string() instead")]
impl Color {
    /// Returns the html string for this color.
    ///
    /// ```
    /// use mendeleev::{Element, Color};
    ///
    /// let color = Color{r: 0, g: 255, b: 255};
    /// assert_eq!(color.html(), "#00ffff");
    /// let color = Color{r: 48, g: 64, b: 80};
    /// assert_eq!(color.html(), "#304050");
    /// ```
    pub fn html(&self) -> String {
        self.to_string()
    }
}

/// Displays the color as an HTML string.
///
/// ```
/// use mendeleev::{Element, Color};
///
/// let color = Color{r: 0, g: 255, b: 255};
/// assert_eq!(format!("{}", color), "#00ffff");
/// let color = Color{r: 48, g: 64, b: 80};
/// assert_eq!(format!("{}", color), "#304050");
/// ```
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

pub(crate) const fn color(r: u8, g: u8, b: u8) -> Option<Color> {
    Some(Color { r, g, b })
}
