use crate::bitboard::{BitBoard, DynGeometry, StaticGeometry};

pub struct BlokusRuleset {}

const STANDARD_BLOKUS_PIECES: [BlokusPiece; 3] = [
    BlokusPiece::parse("x"),
    BlokusPiece::parse("xxx"),
    BlokusPiece::parse(".x.|xxx"),
];

pub struct BlokusPiece {
    shape: BitBoard<DynGeometry, u32>,
}

impl BlokusPiece {
    pub const fn parse(str: &'static str) -> BlokusPiece {
        // TODO: verify str is all ascii
        let str_b = str.as_bytes();

        let mut columns = 0;
        let mut rows = 0;

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
                    _ => panic!("invalid character in blokus piece data"),
                }

                i += 1;
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
                    _ => panic!("invalid character in blokus piece data"),
                }

                i += 1;
            }
        }

        BlokusPiece {
            shape: BitBoard::new_with_data(DynGeometry::new(columns as u16, rows as u16), board_data)
        }
    }
}
