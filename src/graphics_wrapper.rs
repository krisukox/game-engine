use graphics::types::{Color, Matrix2d, Vec2d};
use graphics::Context;
use graphics::{DrawState, Viewport};
// use opengl_graphics::GlGraphics;

pub struct Graphics {}

impl Graphics {
    pub fn draw<F: FnOnce(graphics::Context, &mut Self) + 'static>(
        &mut self,
        viewport: Viewport,
        f: F,
    ) {
        let context = Context {
            viewport: Some(viewport),
            view: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            transform: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            draw_state: graphics::DrawState {
                scissor: None,
                stencil: None,
                blend: None,
            },
        };
        f(context, self);
    }
}

cfg_if::cfg_if! {
    if #[cfg(test)]{
        use Graphics as GlGraphics;
    } else {
        use opengl_graphics::GlGraphics;
    }
}

pub struct GraphicsWrapper();

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
impl GraphicsWrapper {
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
