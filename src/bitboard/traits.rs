use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

/// The backing store for a bit board
#[const_trait]
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
    fn new_zero(min_length: usize) -> Self;

    fn bit_get(&self, index: usize) -> bool;
    fn bit_set(&mut self, index: usize, value: bool);
}


/// Describes the size of a rectangular board

#[const_trait]
pub trait BoardGeometry {
    fn width(&self) -> i16;
    fn height(&self) -> i16;
}
