use crate::types::{Bitboard, File};
use std::ops::Shl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
    All,
}

impl Shl<Direction> for Bitboard {
    type Output = Bitboard;
    fn shl(self, d: Direction) -> Self::Output {
        match d {
            Direction::North => self << 8,
            Direction::South => self >> 8,
            Direction::East => (self << 1) & !Bitboard::from(File::A),
            Direction::West => (self >> 1) & !Bitboard::from(File::H),
            Direction::NorthEast => (self << 9) & !Bitboard::from(File::A),
            Direction::NorthWest => (self << 7) & !Bitboard::from(File::H),
            Direction::SouthEast => (self >> 7) & !Bitboard::from(File::A),
            Direction::SouthWest => (self >> 9) & !Bitboard::from(File::H),
            Direction::All => panic!("Cannot shift by `Direction::All`"),
        }
    }
}
