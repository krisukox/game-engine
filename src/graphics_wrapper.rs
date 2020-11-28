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
    // fn read(f: FTyp) {}
    // pub fn new() -> Self {
    //     Self(GlGraphics::new(OPENGL_VERSION))
    // }

    // #[cfg(not(test))]
    pub fn draw<F: FnOnce(graphics::Context, &mut GlGraphics) + 'static>(
        graphics: &mut GlGraphics,
        viewport: Viewport,
        f: F,
    ) {
        graphics.draw(viewport, f);
        // self.0.draw(viewport, |c, g| f(c, &mut GraphicsWrapper(g)));
    }

    // #[cfg(test)]
    // pub fn draw<F>(graphics: &mut GlGraphics, viewport: Viewport, f: F)
    // where
    //     F: FnOnce(Context, &mut GlGraphics) + 'static,
    // {
    //     graphics.draw(viewport, f);
    //     // self.0.draw(viewport, |c, g| f(c, &mut GraphicsWrapper(g)));
    // }

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

#[cfg(test)]
mod new_test {
    use super::*;
    type FTyp = Box<dyn FnOnce(Context) + 'static>;
    pub struct MyStruct();
    #[cfg_attr(test, automock)]
    impl MyStruct {
        fn read<F: FnOnce(graphics::Context) + 'static>(f: F) {}
        // where
        //     F: FnOnce(Context) + 'static;
    }

    #[test]
    fn read() {}
}

#[cfg_attr(test, automock)]
pub trait BlockProvider {
    fn size(&self) -> u64;
    fn read<F: FnOnce(Result<&[u8], std::io::Error>) + 'static>(
        &self,
        offset: u64,
        size: u32,
        handler: F,
    );
}

// #[cfg(test)]
// use mockall::{automock, predicate::*};

// #[cfg_attr(test, automock)]
// impl Graphics {
//     pub fn cos() {}
// }

// #[cfg(test)]
// impl MockGraphics {
//     pub fn draw<F>(&mut self, viewport: Viewport, f: F)
//     where
//         F: FnOnce(Context, &mut GlGraphics) + 'static,
//     {
//         f()
//     }
// }
