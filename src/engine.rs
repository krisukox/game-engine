use crate::graph::{Coordinate, LinearGraph};
use crate::map::Map;
use crate::object_generator::ObjectGenerator;
use crate::player_utils::Player;
use crate::player_utils::Radians;
use crate::point_generator::PointGenerator;
use crate::polygon_generator::PolygonGenerator;
use glutin_window::GlutinWindow as Window;
use graphics::{Polygon, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{Size, WindowSettings};

pub struct Engine {
    generator: ObjectGenerator,
    player: Player,
    window: Window,
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
        let window: Window = WindowSettings::new("game", resolution)
            .graphics_api(OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .unwrap();
        Result::Ok(Engine {
            generator: ObjectGenerator {
                map,
                rays: player.get_all_rays(),
                polygon_generator,
            },
            player,
            window,
        })
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new());
        let mut gl = GlGraphics::new(OPENGL_VERSION);

        while let Some(e) = events.next(&mut self.window) {
            println!("Naglik !!!!!!");
            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    let transform = c.transform.flip_v().trans(0.0, -300.0);
                    graphics::clear([0.8, 0.8, 0.8, 1.0], g);
                    let polygons = self.generator.generate_polygons(&self.player);
                    for polygon_ in polygons {
                        Polygon::new([1.0, 0.0, 0.0, 1.0]).draw(
                            &polygon_,
                            &c.draw_state,
                            transform,
                            g,
                        );
                    }
                });
            }
        }
    }
}
