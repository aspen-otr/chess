use crate::types::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Square(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Square {
    const fn new_from_u8(value: u8) -> Self {
        Self(value)
    }

    const fn new(file: File, rank: Rank) -> Self {
        let f = file as u8;
        let r = rank as u8;
        Self::new_from_u8((r << 3) | f)
    }

    const fn from_str(s: &str) -> Option<Self> {
        if s.len() != 2 {
            None
        } else {
            let b = s.as_bytes();
            let r = b[1];
            let f = b[0];
            if f < b'a' || f > b'h' || r < b'1' || r > b'8' {
                return None;
            }
            unsafe {
                let r = std::mem::transmute(r - b'1');
                let f = std::mem::transmute(f - b'a');
                Some(Self::new(f, r))
            }
        }
    }
    const unsafe fn from_str_unchecked(s: &str) -> Self {
        let bytes = s.as_bytes();
        let f = unsafe { std::mem::transmute(bytes[0] - b'a') };
        let r = unsafe { std::mem::transmute(bytes[1] - b'1') };
        Self::new(f, r)
    }

    pub const fn into_inner(self) -> u8 {
        self.0
    }

    pub const fn is_ok(self) -> bool {
        self.into_inner() < 64
    }

    pub const fn rank(self) -> Rank {
        todo!()
    }
    pub const fn file(self) -> File {
        todo!()
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Bitboard {
        debug_assert!(square.is_ok());
        Bitboard::ONE << square.into_inner()
    }
}

impl From<Rank> for Bitboard {
    fn from(rank: Rank) -> Bitboard {
        let rank_one = Bitboard::new(0xff);
        let shift = (rank as u8) << 3;
        rank_one << shift
    }
}

impl From<File> for Bitboard {
    fn from(file: File) -> Bitboard {
        let a_file = Bitboard::new(0x0101010101010101);
        let shift = file as u8;
        a_file << shift
    }
}

pub struct SquareParseErr;
impl std::str::FromStr for Square {
    type Err = SquareParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(SquareParseErr)
    }
}

pub const A1: Square = Square::new(File::A, Rank::One);
pub const A2: Square = Square::new(File::A, Rank::Two);
pub const A3: Square = Square::new(File::A, Rank::Three);
pub const A4: Square = Square::new(File::A, Rank::Four);
pub const A5: Square = Square::new(File::A, Rank::Five);
pub const A6: Square = Square::new(File::A, Rank::Six);
pub const A7: Square = Square::new(File::A, Rank::Seven);
pub const A8: Square = Square::new(File::A, Rank::Eight);
pub const B1: Square = Square::new(File::B, Rank::One);
pub const B2: Square = Square::new(File::B, Rank::Two);
pub const B3: Square = Square::new(File::B, Rank::Three);
pub const B4: Square = Square::new(File::B, Rank::Four);
pub const B5: Square = Square::new(File::B, Rank::Five);
pub const B6: Square = Square::new(File::B, Rank::Six);
pub const B7: Square = Square::new(File::B, Rank::Seven);
pub const B8: Square = Square::new(File::B, Rank::Eight);
pub const C1: Square = Square::new(File::C, Rank::One);
pub const C2: Square = Square::new(File::C, Rank::Two);
pub const C3: Square = Square::new(File::C, Rank::Three);
pub const C4: Square = Square::new(File::C, Rank::Four);
pub const C5: Square = Square::new(File::C, Rank::Five);
pub const C6: Square = Square::new(File::C, Rank::Six);
pub const C7: Square = Square::new(File::C, Rank::Seven);
pub const C8: Square = Square::new(File::C, Rank::Eight);
pub const D1: Square = Square::new(File::D, Rank::One);
pub const D2: Square = Square::new(File::D, Rank::Two);
pub const D3: Square = Square::new(File::D, Rank::Three);
pub const D4: Square = Square::new(File::D, Rank::Four);
pub const D5: Square = Square::new(File::D, Rank::Five);
pub const D6: Square = Square::new(File::D, Rank::Six);
pub const D7: Square = Square::new(File::D, Rank::Seven);
pub const D8: Square = Square::new(File::D, Rank::Eight);
pub const E1: Square = Square::new(File::E, Rank::One);
pub const E2: Square = Square::new(File::E, Rank::Two);
pub const E3: Square = Square::new(File::E, Rank::Three);
pub const E4: Square = Square::new(File::E, Rank::Four);
pub const E5: Square = Square::new(File::E, Rank::Five);
pub const E6: Square = Square::new(File::E, Rank::Six);
pub const E7: Square = Square::new(File::E, Rank::Seven);
pub const E8: Square = Square::new(File::E, Rank::Eight);
pub const F1: Square = Square::new(File::F, Rank::One);
pub const F2: Square = Square::new(File::F, Rank::Two);
pub const F3: Square = Square::new(File::F, Rank::Three);
pub const F4: Square = Square::new(File::F, Rank::Four);
pub const F5: Square = Square::new(File::F, Rank::Five);
pub const F6: Square = Square::new(File::F, Rank::Six);
pub const F7: Square = Square::new(File::F, Rank::Seven);
pub const F8: Square = Square::new(File::F, Rank::Eight);
pub const G1: Square = Square::new(File::G, Rank::One);
pub const G2: Square = Square::new(File::G, Rank::Two);
pub const G3: Square = Square::new(File::G, Rank::Three);
pub const G4: Square = Square::new(File::G, Rank::Four);
pub const G5: Square = Square::new(File::G, Rank::Five);
pub const G6: Square = Square::new(File::G, Rank::Six);
pub const G7: Square = Square::new(File::G, Rank::Seven);
pub const G8: Square = Square::new(File::G, Rank::Eight);
pub const H1: Square = Square::new(File::H, Rank::One);
pub const H2: Square = Square::new(File::H, Rank::Two);
pub const H3: Square = Square::new(File::H, Rank::Three);
pub const H4: Square = Square::new(File::H, Rank::Four);
pub const H5: Square = Square::new(File::H, Rank::Five);
pub const H6: Square = Square::new(File::H, Rank::Six);
pub const H7: Square = Square::new(File::H, Rank::Seven);
pub const H8: Square = Square::new(File::H, Rank::Eight);
