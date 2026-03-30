#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl std::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub struct ColorParseErr;

impl TryFrom<char> for Color {
    type Error = ColorParseErr;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            _ => Err(ColorParseErr)
        }
    }
}

impl std::str::FromStr for Color {
    type Err = ColorParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            Err(ColorParseErr)
        } else {
            Self::try_from(s.as_bytes()[0] as char)
        }
    }
}
