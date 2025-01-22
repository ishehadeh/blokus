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

impl BitArray for u32 {
    fn new_zero(min_length: usize) -> Self {
        if min_length > u32::BITS as usize {
            panic!("cannot create a bit array of length {min_length} with a u32 as the backing store");
        }

        0u32
    }

    fn bit_get(&self, index: usize) -> bool {
        self & (1u32 << index) != 0
    }

    fn bit_set(&mut self, index: usize, value: bool) {
        if value {
            *self |= 1u32 << index;
        } else {
            *self &= !(1u32 << index);
        }
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