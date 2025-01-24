use super::BoardGeometry;


pub struct StaticGeometry<const WIDTH: i16, const HEIGHT: i16>;

impl<const WIDTH: i16, const HEIGHT: i16> const BoardGeometry for StaticGeometry<WIDTH,  HEIGHT> {
    fn width(&self) -> i16 {
        WIDTH
    }

    fn height(&self) -> i16 {
        HEIGHT
    }
}

pub struct DynGeometry {
    pub width: i16,
    pub height: i16,
}

impl DynGeometry {
    pub const fn new(width: i16, height: i16) -> DynGeometry {
        if width < 0 || height < 0 {
            panic!("invalid width and height")
        } 
        DynGeometry { width, height }
    }
}

impl const BoardGeometry for DynGeometry {
    fn width(&self) -> i16 {
        self.width
    }

    fn height(&self) -> i16 {
        self.height
    }
}