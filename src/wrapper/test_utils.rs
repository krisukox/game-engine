use graphics::Context;
use graphics::Viewport;

pub struct Window();

pub struct GlGraphics();

impl GlGraphics {
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
