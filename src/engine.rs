use crate::map::Map;
use crate::object_generator::ObjectGenerator;
use crate::player_utils::Player;
use crate::player_utils::Radians;
use crate::point_generator::PointGenerator;
use crate::polygon_generator::PolygonGenerator;
use glutin_window::GlutinWindow;
use graphics::{Polygon, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, MouseRelativeEvent, RenderEvent};
use piston::window::{Size, WindowSettings};
use piston::AdvancedWindow;

pub struct Engine {
    generator: ObjectGenerator,
    player: Player,
    window: GlutinWindow,
}

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

impl Engine {
    #[cfg(not(test))]
    pub fn new(
        path_to_map: &str,
        resolution: Size,
        player: Player,
        vertical_angle_value: Radians,
        wall_height: f64,
    ) -> Result<Engine, image::ImageError> {
        let map = Map::new(path_to_map)?;
        let polygon_generator = PolygonGenerator {
            point_generator: PointGenerator::new(resolution, vertical_angle_value, wall_height),
        };
        Result::Ok(Engine {
            generator: ObjectGenerator {
                map,
                rays: player.get_all_rays(),
                polygon_generator,
            },
            player,
            window: Self::create_window(resolution),
        })
    }

    fn create_window(resolution: Size) -> GlutinWindow {
        let mut window: GlutinWindow = WindowSettings::new("game", resolution)
            .graphics_api(OPENGL_VERSION)
            .fullscreen(false)
            .exit_on_esc(true)
            .build()
            .unwrap();
        window.ctx.window().set_maximized(false);
        window.set_capture_cursor(true);
        return window;
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new());
        let mut gl = GlGraphics::new(OPENGL_VERSION);
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    let transform = c
                        .transform
                        .flip_v()
                        .trans(0.0, -(c.viewport.unwrap().draw_size[1] as f64 / 2.0));
                    graphics::clear([0.8, 0.8, 0.8, 1.0], g);
                    let polygons = self.generator.generate_polygons(&self.player);
                    for polygon_ in polygons {
                        Polygon::new([1.0, 0.0, 0.5, 1.0]).draw(
                            &polygon_,
                            &c.draw_state,
                            transform,
                            g,
                        );
                    }
                });
            }

            if let Some(args) = e.mouse_relative_args() {
                if args[0] > 0.0 {
                    self.player.rotate_left(Radians::new(args[0] / 1000.0));
                } else {
                    self.player.rotate_right(Radians::new(args[0] / 1000.0));
                }
            }

            if let Some(args) = e.button_args() {
                if piston::input::ButtonState::Press == args.state {
                    if let piston::input::Button::Keyboard(key) = args.button {
                        match key {
                            piston::input::Key::W => {
                                self.player.move_forward_backward(0.5);
                            }
                            piston::input::Key::S => {
                                self.player.move_forward_backward(-0.5);
                            }
                            piston::input::Key::A => {
                                self.player.move_right_left(0.5);
                            }
                            piston::input::Key::D => {
                                self.player.move_right_left(-0.5);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
