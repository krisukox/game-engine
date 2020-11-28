use crate::map::Map;
use crate::object_generator::ObjectGenerator;
use crate::player_utils::Player;
use crate::player_utils::Radians;
use crate::point_generator::PointGenerator;
use crate::polygon_generator::PolygonGenerator;
use glutin_window::GlutinWindow;
use graphics::{Polygon, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::EventSettings;
use piston::input::{ButtonEvent, MouseRelativeEvent, RenderEvent};
use piston::window::{Size, WindowSettings};
use piston::AdvancedWindow;
use piston::{Event, Window};
use std::time::Duration;

use graphics::Graphics;

use opengl_graphics::Texture;

use graphics::types::Color;

use graphics::DrawState;

use crate::painter::Painter;

use crate::graphics_wrapper::GraphicsWrapper;

cfg_if::cfg_if! {
    if #[cfg(test)]{
        use crate::events::MockEvents as Events;
    } else {
        use crate::events::Events;
    }
}

pub struct Engine {
    generator: ObjectGenerator,
    player: Player,
    window: GlutinWindow,
    events: Events,
}

#[cfg(test)]
use mockall::{mock, predicate::*};

// #[cfg(test)]
// mock! {
//     MyWindow {}
//     trait Window {
//         fn set_should_close(&mut self, value: bool);
//         fn should_close(&self) -> bool;
//         fn size(&self) -> Size;
//         fn swap_buffers(&mut self);
//         fn wait_event(&mut self) -> Event;
//         fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Event>;
//         fn poll_event(&mut self) -> Option<Event>;
//         fn draw_size(&self) -> Size;
//     }
// }

// trait Cos {
//     type Texture;
//     fn clear_color(&mut self, color: Color);
//     fn clear_stencil(&mut self, value: u8);
//     fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], f: F)
//     where
//         F: FnMut(&mut dyn FnMut(&[[f32; 2]]));
//     fn tri_list_c<F>(&mut self, draw_state: &DrawState, f: F)
//     where
//         F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 4]]));
//     fn tri_list_uv<F>(
//         &mut self,
//         draw_state: &DrawState,
//         color: &[f32; 4],
//         texture: &<Self as Cos>::Texture,
//         f: F,
//     ) where
//         F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]]));
//     fn tri_list_uv_c<F>(&mut self, draw_state: &DrawState, texture: &<Self as Cos>::Texture, f: F)
//     where
//         F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]], &[[f32; 4]]));
// }

// #[cfg(test)]
// mock! {
//     MyGraphics {}
//     trait Graphics {
//         type Texture = Texture;
//         fn clear_color(&mut self, color: Color);
//     fn clear_stencil(&mut self, value: u8);
//     fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], f: F)
//         where F: FnMut(&mut dyn FnMut(&[[f32; 2]]));
//     fn tri_list_c<F>(&mut self, draw_state: &DrawState, f: F)
//         where F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 4]]));
//     fn tri_list_uv<F>(&mut self,
//                       draw_state: &DrawState,
//                       color: &[f32; 4],
//                       texture: &Texture,
//                       f: F)
//         where F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]]));
//     fn tri_list_uv_c<F>(&mut self,
//                       draw_state: &DrawState,
//                       texture: &Texture,
//                       f: F)
//         where F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]], &[[f32; 4]]));
//     }
// }

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

// #[cfg(not(test))]
// impl Engine<GlutinWindow> {
//     pub fn new(
//         path_to_map: &str,
//         resolution: Size,
//         player: Player,
//         vertical_angle_value: Radians,
//         wall_height: f64,
//     ) -> Result<Engine<GlutinWindow>, image::ImageError> {
//         let map = Map::new(path_to_map)?;
//         let polygon_generator = PolygonGenerator {
//             point_generator: PointGenerator::new(resolution, vertical_angle_value, wall_height),
//         };
//         Result::Ok(Engine {
//             generator: ObjectGenerator {
//                 map,
//                 rays: player.get_all_rays(),
//                 polygon_generator,
//             },
//             player,
//             window: Self::create_window(resolution),
//         })
//     }

//     fn create_window(resolution: Size) -> GlutinWindow {
//         let mut window: GlutinWindow = WindowSettings::new("game", resolution)
//             .graphics_api(OPENGL_VERSION)
//             .fullscreen(false)
//             .exit_on_esc(true)
//             .build()
//             .unwrap();
//         window.ctx.window().set_maximized(false);
//         window.set_capture_cursor(true);
//         return window;
//     }
// }

// fn cos(window: &mut GlGraphics) {}

use graphics::Context;

impl Engine
// where
//     W: Window,
{
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
            events: crate::events::Events::new(),
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
        let mut gl = GlGraphics::new(OPENGL_VERSION);
        while let Some(e) = self.events.next_event(&mut self.window) {
            if let Some(args) = e.render_args() {
                let polygons = self.generator.generate_polygons(&self.player);
                GraphicsWrapper::draw(&mut gl, args.viewport(), |c, g| {
                    let transform = c
                        .transform
                        .flip_v()
                        .trans(0.0, -(c.viewport.unwrap().draw_size[1] as f64 / 2.0));
                    GraphicsWrapper::clear(g, [0.8, 0.8, 0.8, 1.0]);
                    for polygon_ in polygons {
                        GraphicsWrapper::draw_polygon(
                            g,
                            [1.0, 0.0, 0.5, 1.0],
                            polygon_,
                            &c.draw_state,
                            transform,
                        );
                    }
                });
            }

            if let Some(args) = e.mouse_relative_args() {
                if args[0] > 0.0 {
                    self.player.rotate_left(Radians::new(args[0] / 1000.0));
                } else {
                    self.player
                        .rotate_right(Radians::new((args[0] / 1000.0).abs()));
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::Coordinate;
    // use crate::graphics_wrapper::MockGraphics;
    use crate::player_utils::{Angle, Player, Radians};
    use crate::polygon_generator::MockPolygonGenerator;

    #[test]
    fn start() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            // let my_cos = MockGraphics::default();
            // my_cos.c();

            let resolution = Size {
                width: 1280.0,
                height: 720.0,
            };
            let vertical_angle_value = Radians::new(std::f64::consts::PI * 0.375);
            let wall_height = 5.0;
            let polygon_generator = MockPolygonGenerator::new();
            let number_of_rays = 2000;
            let player = Player::new(
                Angle {
                    start: Radians::new(std::f64::consts::PI),
                    end: Radians::new(3.0 * std::f64::consts::PI / 2.0),
                },
                Coordinate { x: 27.0, y: 9.0 },
                number_of_rays,
            );

            let generator = ObjectGenerator {
                map,
                rays: player.get_all_rays(),
                polygon_generator,
            };

            let events = crate::events::MockEvents::default();

            let window_mock = Engine::create_window(resolution);
            let engine = Engine {
                generator,
                player,
                window: window_mock,
                events,
            };
        }
    }
}
