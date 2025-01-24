use super::BoardGeometry;


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