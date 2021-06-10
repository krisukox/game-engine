use super::{Color, Point};

#[derive(Clone, PartialEq, Debug)]
pub struct ColoredPoint {
    pub point: Point,
    pub color: Color,
}

impl ColoredPoint {
    pub fn distance(&self, colored_point: &Self) -> f64 {
        self.point.distance(&colored_point.point)
    }
}
