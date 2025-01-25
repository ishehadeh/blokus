use crate::bitboard::{BitBoard, BoardGeometry, Coordinates, Direction, DirectionSet, DynGeometry};

pub struct BlokusRuleset {}
pub trait BlokusPieceSet<const N: usize> {
    const PIECES: [BlokusPiece; N];

    const CORNER_COUNT: usize = {
        let mut corner_count: usize = 0;
        let mut i = 0;
        while i < Self::PIECES.len() {
            corner_count += Self::PIECES[i].count_corners();
            i += 1;
        }
    
        corner_count
    };
    
    const CORNER_OFFSETS: [usize; 21] = {
        let mut current_corner_count: usize = 0;
        let mut corner_offsets: [usize; 21] = [0; 21];
        let mut i = 0;
        while i < N {
            corner_offsets[i] = current_corner_count;
            current_corner_count += Self::PIECES[i].count_corners();
            i += 1;
        }
    
        corner_offsets
    };
    
    const CORNERS: [Coordinates; Self::CORNER_COUNT] = {
        let mut corner_coordinates = [Coordinates::zero(); Self::CORNER_COUNT];
        let mut piece_i = 0;
        while piece_i < Self::PIECES.len() {
            let mut local_corner_i = 0;
            while local_corner_i < Self::PIECES[piece_i].count_corners() {
                corner_coordinates[Self::CORNER_OFFSETS[piece_i] + local_corner_i] =
                    Self::PIECES[piece_i].nth_corner(local_corner_i);
                local_corner_i += 1;
            }
            piece_i += 1;
        }
    
        corner_coordinates
    } where [(); Self::CORNER_COUNT]:;
}

pub struct StandardBlokusPieceSet;
impl BlokusPieceSet<21> for StandardBlokusPieceSet {
    const PIECES: [BlokusPiece; 21] = [
        BlokusPiece::parse("x"),
        BlokusPiece::parse("xx"),
        BlokusPiece::parse("xxx"),
        BlokusPiece::parse("xxxx"),
        BlokusPiece::parse("xxxxx"),
        BlokusPiece::parse(
            r#"x x x x
               x"#,
        ),
        BlokusPiece::parse(
            r#"x x x
               x
               x"#,
        ),
        BlokusPiece::parse(
            r#"x x x
               x"#,
        ),
        BlokusPiece::parse(
            r#"x x
               x"#,
        ),
        BlokusPiece::parse(
            r#". x .
               x x x
               . x ."#,
        ),
        BlokusPiece::parse(
            r#". x .
               . x x
               x x ."#,
        ),
        BlokusPiece::parse(
            r#"x . .
               x x x
               x . ."#,
        ),
        BlokusPiece::parse(
            r#"x .
               x .
               x x
               x ."#,
        ),
        BlokusPiece::parse(
            r#"x .
               x x
               x ."#,
        ),
        BlokusPiece::parse(
            r#"x x
               x .
               x x"#,
        ),
        BlokusPiece::parse(
            r#"x x
               x x"#,
        ),
        BlokusPiece::parse(
            r#"x x x
               x x ."#,
        ),
        BlokusPiece::parse(
            r#". x x
               . x .
               x x ."#,
        ),
        BlokusPiece::parse(
            r#". x
               x x
               x .
               x"#,
        ),
        BlokusPiece::parse(
            r#". . x
               . x x
               x x ."#,
        ),
        BlokusPiece::parse(
            r#". x
               x x
               x ."#,
        ),
    ];
}

pub struct BlokusPiece {
    pub shape: BitBoard<DynGeometry, u32>,
}

impl BlokusPiece {
    pub const fn nth_corner(&self, n: usize) -> Coordinates {
        let mut corners = n;
        let mut coord = Coordinates::zero();
        while coord.x < self.shape.geometry().width() {
            while coord.y < self.shape.geometry().height() {
                let mut i = 0;
                while i < 4 {
                    let corner_mask = Direction::DIAGONAL[i].components().union_c(
                        DirectionSet::from_bits_truncate_c(
                            Direction::DIAGONAL[i] as u8,
                            DirectionSet::CONST_TOKEN,
                        ),
                    );
                    let set_blocks_around_corner = self
                        .shape
                        .are_adjacent_tiles_set(coord, corner_mask)
                        .bits_c();
                    if set_blocks_around_corner == 0 {
                        if corners == 0 {
                            return coord + Direction::DIAGONAL[i].as_coordinates();
                        } else {
                            corners -= 1;
                        }
                    }
                    i += 1;
                }
                coord.y += 1;
            }
            coord.y = 0;
            coord.x += 1;
        }
        panic!("invalid corner index");
    }

    pub const fn count_corners(&self) -> usize {
        let mut corners = 0;
        let mut coord = Coordinates::zero();
        while coord.x < self.shape.geometry().width() {
            while coord.y < self.shape.geometry().height() {
                let mut i = 0;
                while i < 4 {
                    let corner_mask = Direction::DIAGONAL[i].components().union_c(
                        DirectionSet::from_bits_truncate_c(
                            Direction::DIAGONAL[i] as u8,
                            DirectionSet::CONST_TOKEN,
                        ),
                    );
                    let set_blocks_around_corner = self
                        .shape
                        .are_adjacent_tiles_set(coord, corner_mask)
                        .bits_c();
                    if set_blocks_around_corner == 0 {
                        corners += 1;
                    }
                    i += 1;
                }
                coord.y += 1;
            }
            coord.y = 0;
            coord.x += 1;
        }
        corners
    }

    pub const fn parse(str: &'static str) -> BlokusPiece {
        // TODO: verify str is all ascii
        let str_b = str.as_bytes();

        let mut columns = 1;
        let mut rows = 1;

        // determine row and column counts
        {
            let mut i = 0;
            let mut current_col_count = 0;
            while i < str.len() {
                match str_b[i] {
                    b'|' | b'\n' => {
                        rows += 1;
                        if current_col_count > columns {
                            columns = current_col_count
                        }
                        current_col_count = 0;
                    }
                    b'x' | b'.' => {
                        current_col_count += 1;
                    }
                    b' ' => (),
                    _ => panic!("invalid character in blokus piece data"),
                }

                i += 1;
            }

            if current_col_count > columns {
                columns = current_col_count
            }
        }

        // set up board data
        let mut board_data = 0u32;
        {
            let mut i = 0;
            let mut x = 0;
            let mut y = 0;
            while i < str.len() {
                match str_b[i] {
                    b'|' | b'\n' => {
                        x = 0;
                        y += 1;
                    }
                    b'x' | b'.' => {
                        if str_b[i] == b'x' {
                            board_data |= 1u32 << (columns * y + x);
                        }
                        x += 1;
                    }
                    b' ' => (),
                    _ => panic!("invalid character in blokus piece data"),
                }

                i += 1;
            }
        }

        BlokusPiece {
            shape: BitBoard::new_with_data(DynGeometry::new(columns, rows), board_data),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard::BoardGeometry;

    use super::{BlokusPiece, BlokusPieceSet, StandardBlokusPieceSet};

    #[test]
    fn standard_blokus_pieces_have_at_most_5_tiles() {
        for (i, piece) in StandardBlokusPieceSet::PIECES.iter().enumerate() {
            let tile_count = piece.shape.data().count_ones();
            assert!(
                tile_count <= 5,
                "piece #{i} has {tile_count} tiles, expected at most 5",
            )
        }
    }

    #[test]
    fn standard_blokus_pieces_include_all_expected_dimensions() {
        let mut dimen = vec![
            (3, 3),
            (4, 2),
            (3, 2),
            (2, 2),
            (3, 3),
            (3, 3),
            (3, 3),
            (2, 3),
            (2, 4),
            (2, 3),
            (2, 2),
            (3, 2),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (3, 3),
            (2, 4),
            (3, 3),
            (2, 3),
        ];

        for (i, piece) in StandardBlokusPieceSet::PIECES.iter().enumerate() {
            let geom = piece.shape.geometry();
            let index = dimen
                .iter()
                .enumerate()
                .find(|(_, p)| p.0 == geom.width() && p.1 == geom.height())
                .map(|(i, _)| i);
            match index {
                Some(index) => {
                    dimen.remove(index);
                }
                None => panic!(
                    "piece #{i} is {w}x{h}, which is not in dimension list\n  dimen = {dimen:#?}",
                    w = geom.width(),
                    h = geom.height()
                ),
            }
        }
    }

    #[test]
    fn correctly_count_1x1_corners() {
        let _1x1 = BlokusPiece::parse("x");
        assert_eq!(_1x1.count_corners(), 4);
    }
}
