use crate::graph::Coordinate;
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
use piston::input::{ButtonEvent, MouseCursorEvent, MouseRelativeEvent, RenderEvent};
use piston::window::{Size, WindowSettings};
use piston::AdvancedWindow;
// cfg_if::cfg_if! {
// if #[cfg(not(test))]{
// use crate::map::Map;
// use crate::point_generator::PointGenerator;
// use crate::polygon_generator::PolygonGenerator;

// }
// }

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
            point_generator: PointGenerator {
                resolution,
                half_vertical_angle_value: Radians(vertical_angle_value / 2.0),
                wall_height,
            },
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
        if let Result::Err(err) = window.ctx.window().grab_cursor(true) {
            println!("create window grab cursor Error: {}", err);
        }
        // window.ctx.window().hide_cursor(true);
        return window;
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new());
        let mut gl = GlGraphics::new(OPENGL_VERSION);
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    let transform = c.transform.flip_v().trans(0.0, -300.0);
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

            if let Some(args) = e.mouse_cursor_args() {
                // println!("naglik mouse_cursor_args {:?}", args);
            }

            if let Some(args) = e.mouse_relative_args() {
                self.player.rotate(Radians(args[0] / 1000.0));
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
