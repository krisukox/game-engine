use graphics::types::Color;
use graphics::types::Matrix2d;
use graphics::types::Vec2d;
use graphics::Context;
use graphics::DrawState;
use graphics::Viewport;
use image::flat::Error;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct GraphicsWrapper();

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
impl GraphicsWrapper {
    // TODO consider to add two F types instead of two implementation of draw function
    #[cfg(test)]
    pub fn draw<F: FnOnce(graphics::Context, &mut GlGraphics) + 'static>(
        graphics: &mut GlGraphics,
        viewport: Viewport,
        f: F,
    ) {
        graphics.draw(viewport, f);
    }

    #[cfg(not(test))]
    pub fn draw<F: FnOnce(graphics::Context, &mut GlGraphics)>(
        graphics: &mut GlGraphics,
        viewport: Viewport,
        f: F,
    ) {
        graphics.draw(viewport, f);
    }

    pub fn clear(graphics: &mut GlGraphics, color: Color) {
        graphics::clear(color, graphics);
    }

    pub fn draw_polygon(
        graphics: &mut GlGraphics,
        color: Color,
        polygon: [Vec2d; 4],
        draw_state: &DrawState,
        transform: Matrix2d,
    ) {
        graphics::Polygon::new(color).draw(&polygon, draw_state, transform, graphics);
    }
}
