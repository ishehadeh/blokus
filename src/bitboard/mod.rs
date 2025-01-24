mod traits;
mod coordinates;
mod geometry_impl;
mod bitarray_impl;

pub use coordinates::*;
pub use traits::*;
pub use geometry_impl::*;
#[allow(unused, reason="currently there's nothing being re-exported, but that may change in the future")]
pub use bitarray_impl::*;

/// A 2D array of boolean values, typically used to store some information about a GridGame
pub struct BitBoard<GeomT: BoardGeometry, BitsT: BitArray> {
    data: BitsT,
    geometry: GeomT,
}

impl<GeomT: BoardGeometry, BitsT: BitArray> BitBoard<GeomT, BitsT> {
    pub const fn new_with_data(geometry: GeomT, data: BitsT) -> Self {
        Self { data, geometry }
    }

    pub fn new(geometry: GeomT) -> Self {
        Self {
            data: BitsT::new_zero(geometry.width() as usize * geometry.height() as usize),
            geometry,
        }
    }

    /// Get the underlying bit array where the board state is stored
    pub fn data(&self) -> &BitsT {
        &self.data
    }

    /// Get the board's geomtry (i.e. height and width)
    pub fn geometry(&self) -> &GeomT {
        &self.geometry
    }
}
