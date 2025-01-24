mod traits;
mod coordinates;
pub mod bitarray_impl;

pub use coordinates::*;
pub use traits::*;


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

pub struct StaticGeometry<const WIDTH: u16, const HEIGHT: u16>;

impl<const WIDTH: u16, const HEIGHT: u16> BoardGeometry for StaticGeometry<WIDTH,  HEIGHT> {
    fn width(&self) -> u16 {
        WIDTH
    }

    fn height(&self) -> u16 {
        HEIGHT
    }
}

pub struct DynGeometry {
    pub width: u16,
    pub height: u16,
}

impl DynGeometry {
    pub const fn new(width: u16, height: u16) -> DynGeometry {
        DynGeometry { width, height }
    }
}

impl BoardGeometry for DynGeometry {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}