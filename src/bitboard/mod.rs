mod traits;
mod coordinates;

pub use coordinates::*;
pub use traits::*;


/// A 2D array of boolean values, typically used to store some information about a GridGame
pub struct BitBoard<GeomT: BoardGeometry, BitsT: BitArray> {
    data: BitsT,
    geometry: GeomT,
}

impl<GeomT: BoardGeometry, BitsT: BitArray> BitBoard<GeomT, BitsT> {

    pub fn new(geometry: GeomT) -> Self {
        Self {
            geometry,
            data: BitsT::new_zero()
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