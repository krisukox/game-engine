use graphics::types::{Color, Matrix2d, Vec2d};
use graphics::DrawState;

cfg_if::cfg_if! {
    if #[cfg(test)]{
        use super::test_utils::Graphics as GlGraphics;
    } else {
        use opengl_graphics::GlGraphics;
    }
}

pub struct Graphics();

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(not(tarpaulin_include))]
#[allow(dead_code)]
#[allow(unused_variables)]
#[cfg_attr(test, automock)]
impl Graphics {
    pub fn clear(graphics: &mut GlGraphics, color: Color) {
        #[cfg(not(test))]
        graphics::clear(color, graphics);
    }

    pub fn draw_polygon(
        graphics: &mut GlGraphics,
        color: Color,
        polygon: [Vec2d; 4],
        draw_state: &DrawState,
        transform: Matrix2d,
    ) {
        #[cfg(not(test))]
        graphics::Polygon::new(color).draw(&polygon, draw_state, transform, graphics);
    }
}
