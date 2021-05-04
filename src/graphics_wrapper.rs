use graphics::types::{Color, Matrix2d, Vec2d};
use graphics::Context;
use graphics::{DrawState, Viewport};
use opengl_graphics::GlGraphics;

pub struct GraphicsWrapper();

pub struct Graphics {}

impl Graphics {
    fn draw<F: FnOnce(graphics::Context, &mut Graphics) + 'static>(
        &mut self,
        viewport: Viewport,
        f: F,
    ) {
        let context = Context {
            viewport: None,
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

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
impl GraphicsWrapper {
    #[cfg(not(test))]
    pub fn draw<F: FnOnce(graphics::Context, &mut GlGraphics)>(
        graphics: &mut GlGraphics,
        viewport: Viewport,
        f: F,
    ) {
        println!("NAGLIK draw!!!!!!!!!!");
        graphics.draw(viewport, f);
    }

    // TODO consider to add two F types instead of two implementation of draw function
    #[cfg(test)]
    pub fn draw<F: FnOnce(graphics::Context, &mut Graphics) + 'static>(
        graphics: &mut Graphics,
        viewport: Viewport,
        f: F,
    ) {
        // println!("NAGLIK draw!!!!!!!!!!");
        // let context = Context {
        //     viewport: None,
        //     view: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        //     transform: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        //     draw_state: graphics::DrawState {
        //         scissor: None,
        //         stencil: None,
        //         blend: None,
        //     },
        // };
        // f(context, graphics);
        graphics.draw(viewport, f);
    }
    #[cfg(not(test))]
    pub fn clear(graphics: &mut GlGraphics, color: Color) {
        graphics::clear(color, graphics);
    }
    #[cfg(test)]
    pub fn clear(graphics: &mut Graphics, color: Color) {}

    #[cfg(not(test))]
    pub fn draw_polygon(
        graphics: &mut GlGraphics,
        color: Color,
        polygon: [Vec2d; 4],
        draw_state: &DrawState,
        transform: Matrix2d,
    ) {
        graphics::Polygon::new(color).draw(&polygon, draw_state, transform, graphics);
    }
    #[cfg(test)]
    pub fn draw_polygon(
        graphics: &mut Graphics,
        color: Color,
        polygon: [Vec2d; 4],
        draw_state: &DrawState,
        transform: Matrix2d,
    ) {
        println!("NAGLIK!!!!!!!!!!");
    }
}
