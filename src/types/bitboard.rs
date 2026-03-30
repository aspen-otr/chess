use crate::types::Square;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const ZERO: Self = Self::new(0);
    pub const ONE: Self = Self::new(1);
    pub const ALL: Self = Self::new(!0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_inner(self) -> u64 {
        self.0
    }

    pub const fn more_than_one(self) -> bool {
        self.into_inner() & self.into_inner().wrapping_sub(1) > 0
    }

    pub fn lsb(self) -> Option<Square> {
        todo!()
    }

    pub unsafe fn lsb_unchecked(self) -> Square {
        todo!()
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() | rhs.into_inner())
    }
}
impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() & rhs.into_inner())
    }
}
impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() ^ rhs.into_inner())
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}
impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}
impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}
impl Neg for Bitboard {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(self.into_inner().wrapping_neg())
    }
}
impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::new(self.into_inner().not())
    }
}

impl<T> Shl<T> for Bitboard
where
    u64: Shl<T, Output = u64>,
{
    type Output = Self;
    fn shl(self, s: T) -> Self::Output {
        Self::new(self.into_inner() << s)
    }
}
impl<T> Shr<T> for Bitboard
where
    u64: Shr<T, Output = u64>,
{
    type Output = Self;
    fn shr(self, s: T) -> Self::Output {
        Self::new(self.into_inner() >> s)
    }
}

impl<T> ShlAssign<T> for Bitboard
where
    Bitboard: Shl<T, Output = Self>,
{
    fn shl_assign(&mut self, s: T) {
        *self = *self << s;
    }
}

impl<T> ShrAssign<T> for Bitboard
where
    Bitboard: Shr<T, Output = Self>,
{
    fn shr_assign(&mut self, s: T) {
        *self = *self >> s;
    }
}
