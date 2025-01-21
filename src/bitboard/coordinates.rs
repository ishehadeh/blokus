#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}


impl Coordinates {
     pub fn new(x: u16, y: u16) -> Coordinates {
        Coordinates {
            x,
            y,
        }
     }
}