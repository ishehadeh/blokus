use crate::bitboard::{BitBoard, DynGeometry};

pub struct BlokusRuleset {}

pub const STANDARD_BLOKUS_PIECES: [BlokusPiece; 21] = [
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

pub struct BlokusPiece {
    pub shape: BitBoard<DynGeometry, u32>,
}

impl BlokusPiece {
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
            shape: BitBoard::new_with_data(
                DynGeometry::new(columns as u16, rows as u16),
                board_data,
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard::BoardGeometry;

    use super::STANDARD_BLOKUS_PIECES;

    #[test]
    fn standard_blokus_pieces_have_at_most_5_tiles() {
        for (i, piece) in STANDARD_BLOKUS_PIECES.iter().enumerate() {
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

        for (i, piece) in STANDARD_BLOKUS_PIECES.iter().enumerate() {
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
}
