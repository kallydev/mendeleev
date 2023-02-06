use core::fmt::{Display, Error, Formatter, Result, Write};
use core::iter::Iterator;

pub struct Superscript(u32);

impl Superscript {
    pub fn new(number: u32) -> Self {
        Self(number)
    }
}

impl Display for Superscript {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Self(number) = self;
        for digit in Digits::new(*number) {
            let c = to_superscript_utf8(digit).ok_or(Error)?;
            f.write_char(c)?;
        }
        Ok(())
    }
}

fn to_superscript_utf8(digit: u32) -> Option<char> {
    match digit {
        0 => '⁰',
        1 => '¹',
        2 => '²',
        3 => '³',
        4 => '⁴',
        5 => '⁵',
        6 => '⁶',
        7 => '⁷',
        8 => '⁸',
        9 => '⁹',
        _ => None?,
    }
    .into()
}

struct Digits {
    number: u32,
    divider: u32,
    radix: u32,
}

impl Digits {
    fn new(number: u32) -> Self {
        let radix = 10;
        let mut divider = 1;
        while divider <= number {
            divider = divider.saturating_mul(radix);
        }
        divider = divider.saturating_div(radix).max(1);
        Self {
            number,
            divider,
            radix,
        }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let digit = self.number.checked_div(self.divider);
        self.number = self.number.checked_rem(self.divider).unwrap_or(0);
        self.divider = self.divider.saturating_div(self.radix);
        digit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::iter::Iterator;

    macro_rules! compare {
        ($digits:expr, $expected:expr) => {
            let mut digits = $digits;
            let digits = core::array::from_fn(|_| digits.next().unwrap());
            assert_eq!(digits, $expected);
        };
    }

    #[test]
    fn iterate_digits() {
        compare!(Digits::new(0), [0]);
        compare!(Digits::new(1), [1]);
        compare!(Digits::new(5), [5]);
        compare!(Digits::new(10), [1, 0]);
        compare!(Digits::new(99), [9, 9]);
        compare!(Digits::new(100), [1, 0, 0]);
        compare!(Digits::new(101), [1, 0, 1]);
        compare!(Digits::new(1234), [1, 2, 3, 4]);
    }
}
