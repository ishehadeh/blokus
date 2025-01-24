mod bitarray_impl;
mod coordinates;
mod geometry_impl;
mod traits;

#[allow(
    unused,
    reason = "currently there's nothing being re-exported, but that may change in the future"
)]
pub use bitarray_impl::*;
pub use coordinates::*;
pub use geometry_impl::*;
pub use traits::*;

const BITBOARD_VALIDATE_ALL_COORDINATES: bool = true;

/// A 2D array of boolean values, typically used to store some information about a GridGame
pub struct BitBoard<GeomT: const BoardGeometry, BitsT: const BitArray> {
    data: BitsT,
    geometry: GeomT,
}

impl<GeomT: const BoardGeometry, BitsT: const BitArray> BitBoard<GeomT, BitsT> {
    pub const fn new_with_data(geometry: GeomT, data: BitsT) -> Self {
        Self { data, geometry }
    }

    pub fn new(geometry: GeomT) -> Self {
        Self {
            data: BitsT::new_zero(geometry.width() as usize * geometry.height() as usize),
            geometry,
        }
    }

    pub const fn is_coordinate_in_bounds(&self, coord: Coordinates) -> bool {
        coord.y >= 0
            && coord.x >= 0
            && coord.x < self.geometry().width()
            && coord.y < self.geometry().height()
    }

    pub const fn coordinate_to_index(&self, coord: Coordinates) -> usize {
        if BITBOARD_VALIDATE_ALL_COORDINATES {
            assert!(self.is_coordinate_in_bounds(coord));
        }

        self.geometry().width() as usize * coord.y as usize + coord.x as usize
    }

    /// Get the underlying bit array where the board state is stored
    pub const fn data(&self) -> &BitsT {
        &self.data
    }
    
    /// Get the board's geomtry (i.e. height and width)
    pub const fn geometry(&self) -> &GeomT {
        &self.geometry
    }

    pub const fn get(&self, coord: Coordinates) -> bool {
        self.data().bit_get(self.coordinate_to_index(coord))
    }

    pub fn set(&mut self, coord: Coordinates, value: bool)  {
        self.data.bit_set(self.coordinate_to_index(coord), value)
    }

    /// Test the tiles surrounding `coord` in direction(s) `dir`.
    /// Any tested tiles set to `1` will have their direction, relative to `coord`, set in the the returned bitmap.
    pub const fn are_adjacent_tiles_set(
        &self,
        coord: Coordinates,
        dir: DirectionSet,
    ) -> DirectionSet {
        let mut result = 0u8;

        let all_dirs = Direction::all_as_array();
        let mut i = 0;
        while i < all_dirs.len() {
            if (dir.bits_c() & all_dirs[i] as u8) != 0 {
                let test_coord = coord + all_dirs[i].as_coordinates();
                if self.is_coordinate_in_bounds(test_coord) && self.get(test_coord) {
                    result |= all_dirs[i] as u8
                }
            }
            i += 1;
        }

        DirectionSet::from_bits_truncate_c(result, DirectionSet::CONST_TOKEN)
    }

    /// Test the tiles surrounding `coord` in direction(s) `dir`, treat tiles that are out of bounds as set.
    /// Any tested tiles set to `1` will have their direction, relative to `coord`, set in the the returned bitmap.
    pub const fn are_adjacent_tiles_set_or_out_of_bounds(
        &self,
        coord: Coordinates,
        dir: DirectionSet,
    ) -> DirectionSet {
        let mut result = 0u8;

        let all_dirs = Direction::all_as_array();
        let mut i = 0;
        while i < all_dirs.len() {
            if (dir.bits_c() & all_dirs[i] as u8) != 0 {
                let test_coord = coord + all_dirs[i].as_coordinates();
                if !self.is_coordinate_in_bounds(test_coord) || self.get(test_coord) {
                    result |= all_dirs[i] as u8
                }
            }
            i += 1;
        }

        DirectionSet::from_bits_truncate_c(result, DirectionSet::CONST_TOKEN)
    }
}


#[cfg(test)]
mod test {
    use enumflags2::make_bitflags;

    use crate::bitboard::Direction;

    use super::{BitBoard, Coordinates, DirectionSet, StaticGeometry};

    #[test]
    fn are_adjacent_tile_set_correct_for_1x1_in_center_of_3x3() {
        let mut board = BitBoard::new_with_data(StaticGeometry::<3, 3>, 0u16);
        let set_coord = Coordinates::new_with_column_first(1, 1);
        board.set(set_coord, true);

        assert_eq!(board.are_adjacent_tiles_set(set_coord, DirectionSet::all()), DirectionSet::EMPTY);
        assert_eq!(board.are_adjacent_tiles_set(set_coord, make_bitflags!(Direction::{East | West})), DirectionSet::EMPTY);

        board.set(set_coord + Direction::East.as_coordinates(), true);

        assert_eq!(board.are_adjacent_tiles_set(set_coord, DirectionSet::all()), make_bitflags!(Direction::{East}));
        assert_eq!(board.are_adjacent_tiles_set(set_coord, make_bitflags!(Direction::{East | West})), make_bitflags!(Direction::{East}));
        assert_eq!(board.are_adjacent_tiles_set(set_coord, make_bitflags!(Direction::{South | West})), DirectionSet::EMPTY);

    }
}