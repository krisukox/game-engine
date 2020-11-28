use graphics::types::Color;
use graphics::types::Matrix2d;
use graphics::DrawState;
use graphics::{clear, types};
// use graphics::{clear, Polygon};
use graphics::types::Vec2d;
use opengl_graphics::GlGraphics;

pub struct Painter();

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
impl Painter {
    pub fn draw_polygon(
        color: Color,
        polygon: [Vec2d; 4],
        draw_state: &DrawState,
        transform: Matrix2d,
        graphics: &mut GlGraphics,
    ) {
        graphics::Polygon::new(color).draw(&polygon, draw_state, transform, graphics);
    }
}

// #[cfg_attr(test, automock)]
// pub trait Painter {
//     fn draw_polygon(
//         color: Color,
//         polygon: [Vec2d; 4],
//         draw_state: &DrawState,
//         transform: Matrix2d,
//         graphics: &mut GlGraphics,
//     ) {
//         graphics::Polygon::new(color).draw(&polygon, draw_state, transform, graphics);
//     }
// }
