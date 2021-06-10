use crate::map_element::Color;
use graphics::types::Vec2d;

#[derive(Clone, PartialEq, Debug)]
pub struct Polygon {
    pub area: [Vec2d; 4],
    pub color: Color,
}
