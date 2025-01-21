use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

/// The backing store for a bit board
pub trait BitArray:
    ShlAssign
    + ShrAssign
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + Shl<Self>
    + Shr<Self>
    + BitAnd<Self>
    + BitOr<Self>
    + BitXor<Self>
    + Sized
{
    /// Create a new instance of the bit array with all bits set to 0
    fn new_zero() -> Self;

    fn bit_get(&self, index: usize) -> bool;
    fn bit_set(&mut self, index: usize, value: bool);
}


/// Describes the size of a rectangular board
pub trait BoardGeometry {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}
