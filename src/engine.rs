use crate::map_element::MapElement;
use crate::player_utils::Radians;
use graphics::types::Color;
use graphics::Transformed;
use piston::input::{ButtonEvent, MouseRelativeEvent, RenderEvent, UpdateEvent};

cfg_if::cfg_if! {
    if #[cfg(test)]{
        use crate::object_generator::MockObjectGenerator as ObjectGenerator;
        use crate::player_utils::MockPlayer as Player;
        use crate::events_wrapper::MockEvents as Events;
        use crate::graphics_wrapper::MockGraphics as Graphics;
        use crate::test_utils::Window as GlutinWindow;
        use crate::test_utils::Graphics as GlGraphics;
    } else {
        use crate::events_wrapper::Events;
        use crate::graphics_wrapper::Graphics;
        use crate::map::Map;
        use crate::object_generator::ObjectGenerator;
        use crate::player_utils::Player;
        use crate::polygon_generator::PolygonGenerator;
        use crate::point_generator::PointGenerator;
        use glutin_window::GlutinWindow;
        use opengl_graphics::GlGraphics;
        use piston::AdvancedWindow;
        use piston::window::{Size, WindowSettings};
        use opengl_graphics::OpenGL;

        const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
    }
}

pub struct Engine {
    generator: ObjectGenerator,
    player: Player,
    window: GlutinWindow,
    events: Events,
    graphics: GlGraphics,
    map_elements: Vec<Box<dyn MapElement>>,
}

const BACKGROUND_COLOR: Color = [0.8, 0.8, 0.8, 1.0];
const WALL_COLOR: Color = [1.0, 0.0, 0.5, 1.0];

impl Engine {
    #[cfg(not(tarpaulin_include))]
    #[cfg(not(test))]
    pub fn new(
        resolution: Size,
        player: Player,
        vertical_angle_value: Radians,
        wall_height: f64,
        map_elements: Vec<Box<dyn MapElement>>,
        map: Map,
    ) -> Engine {
        let polygon_generator = PolygonGenerator {
            point_generator: PointGenerator::new(resolution, vertical_angle_value, wall_height),
        };
        Engine {
            generator: ObjectGenerator {
                rays: player.get_all_rays(),
                polygon_generator,
                map,
            },
            player,
            window: Self::create_window(resolution),
            events: Events::new(),
            graphics: GlGraphics::new(OPENGL_VERSION),
            map_elements,
        }
    }

    #[cfg(not(tarpaulin_include))]
    #[cfg(not(test))]
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

    pub(crate) fn cos() {}

    pub fn start(&mut self) {
        Self::cos();
        while let Some(e) = self.events.next_event(&mut self.window) {
            if let Some(args) = e.render_args() {
                let polygons = self
                    .generator
                    .generate_polygons(&self.player, &self.map_elements);
                self.graphics.draw(args.viewport(), |c, g| {
                    let transform = c
                        .transform
                        .flip_v()
                        .trans(0.0, -(c.viewport.unwrap().draw_size[1] as f64 / 2.0));
                    Graphics::clear(g, BACKGROUND_COLOR);
                    for polygon_ in polygons {
                        Graphics::draw_polygon(g, WALL_COLOR, polygon_, &c.draw_state, transform);
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
                if let piston::input::Button::Keyboard(key) = args.button {
                    match key {
                        piston::input::Key::W => {
                            self.player.move_forward(into_bool(args.state));
                        }
                        piston::input::Key::S => {
                            self.player.move_backward(into_bool(args.state));
                        }
                        piston::input::Key::A => {
                            self.player.move_left(into_bool(args.state));
                        }
                        piston::input::Key::D => {
                            self.player.move_right(into_bool(args.state));
                        }
                        _ => {}
                    }
                }
            }

            if let Some(args) = e.update_args() {
                if self.player.update() {
                    for map_element in &mut self.map_elements {
                        map_element
                            .as_mut()
                            .on_position_update(&self.player.position());
                    }
                }
                for map_element in &mut self.map_elements {
                    map_element.as_mut().update(args.dt);
                }
            }
        }
    }
}

fn into_bool(state: piston::input::ButtonState) -> bool {
    if state == piston::input::ButtonState::Press {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::*;
    use crate::events_wrapper::MockEvents;
    use crate::graph::Coordinate;
    use crate::graphics_wrapper::MockGraphics;
    use crate::map_element::MockMapElement;
    use crate::object_generator::MockObjectGenerator;
    use crate::player_utils::{MockPlayer, Radians};
    use crate::test_utils::Graphics;
    use crate::test_utils::Window;
    use graphics::types::Vec2d;
    use mockall::*;
    use piston::input::*;
    use piston::*;

    fn call_none_event(events: &mut MockEvents, seq: &mut Sequence) {
        events
            .expect_next_event()
            .times(1)
            .return_const(None)
            .in_sequence(seq);
    }

    fn call_move_event(events: &mut MockEvents, seq: &mut Sequence, motion_value: [f64; 2]) {
        events
            .expect_next_event()
            .times(1)
            .return_const(Some(piston::Event::Input(
                Input::Move(Motion::MouseRelative(motion_value)),
                None,
            )))
            .in_sequence(seq);
    }

    fn call_key_event(
        events: &mut MockEvents,
        seq: &mut Sequence,
        key: input::Key,
        state: ButtonState,
    ) {
        events
            .expect_next_event()
            .times(1)
            .return_const(Some(piston::Event::Input(
                Input::Button(ButtonArgs {
                    state,
                    button: Button::Keyboard(key),
                    scancode: None,
                }),
                None,
            )))
            .in_sequence(seq);
    }

    fn expect_move_right(player: &mut MockPlayer, seq: &mut Sequence, is_move: bool) {
        player
            .expect_move_right()
            .times(1)
            .withf(move |_is_move| *_is_move == is_move)
            .return_const(())
            .in_sequence(seq);
    }

    fn expect_move_left(player: &mut MockPlayer, seq: &mut Sequence, is_move: bool) {
        player
            .expect_move_left()
            .times(1)
            .withf(move |_is_move| *_is_move == is_move)
            .return_const(())
            .in_sequence(seq);
    }

    fn expect_move_forward(player: &mut MockPlayer, seq: &mut Sequence, is_move: bool) {
        player
            .expect_move_forward()
            .times(1)
            .withf(move |_is_move| *_is_move == is_move)
            .return_const(())
            .in_sequence(seq);
    }

    fn expect_move_backward(player: &mut MockPlayer, seq: &mut Sequence, is_move: bool) {
        player
            .expect_move_backward()
            .times(1)
            .withf(move |_is_move| *_is_move == is_move)
            .return_const(())
            .in_sequence(seq);
    }

    #[test]
    fn start_render_event() {
        let mut seq = Sequence::new();

        let mut generator = MockObjectGenerator::new();
        let player = MockPlayer::default();
        let window = crate::test_utils::Window {};
        let mut events = MockEvents::default();
        let graphics = Graphics {};
        let map_elements: Vec<Box<dyn MapElement>> = vec![Box::new(MockMapElement::new())];

        let clear_ctx = MockGraphics::clear_context();
        let draw_polygon_ctx = MockGraphics::draw_polygon_context();

        lazy_static! {
            static ref polygons: Vec<[Vec2d; 4]> = vec![
                [[0.0, 1.0], [2.0, 3.0], [4.0, 5.0], [6.0, 7.0]],
                [[8.0, 9.0], [10.0, 11.0], [12.0, 13.0], [14.0, 15.0]]
            ];
        }
        static graphic_context: graphics::Context = graphics::Context {
            viewport: Some(graphics::Viewport {
                rect: [1, 2, 3, 4],
                draw_size: [1, 2],
                window_size: [1.0, 2.0],
            }),
            view: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
            transform: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
            draw_state: graphics::DrawState {
                scissor: None,
                stencil: None,
                blend: None,
            },
        };
        static render_args: RenderArgs = RenderArgs {
            ext_dt: 1.0,
            window_size: [2.0, 3.0],
            draw_size: [1, 2],
        };
        let event = piston::Event::Loop(Loop::Render(render_args));

        events
            .expect_next_event()
            .times(1)
            .return_const(Some(event))
            .in_sequence(&mut seq);

        generator
            .expect_generate_polygons()
            .times(1)
            .return_const(polygons.clone())
            .in_sequence(&mut seq);

        clear_ctx
            .expect()
            .times(1)
            .withf(|_, color| *color == BACKGROUND_COLOR)
            .return_const(())
            .in_sequence(&mut seq);

        for _polygon in polygons.iter() {
            draw_polygon_ctx
                .expect()
                .times(1)
                .withf(move |_, color, polygon, draw_state, _| {
                    *color == WALL_COLOR
                        && *polygon == *_polygon
                        && *draw_state == graphic_context.draw_state
                })
                .return_const(())
                .in_sequence(&mut seq);
        }

        call_none_event(&mut events, &mut seq);

        let mut engine = Engine {
            generator,
            player,
            window,
            events,
            graphics,
            map_elements,
        };

        engine.start();
    }

    #[test]
    fn start_mouse_event() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let mut player = MockPlayer::default();
        let window = Window {};
        let mut events = MockEvents::default();
        let graphics = Graphics {};
        let map_elements: Vec<Box<dyn MapElement>> = vec![Box::new(MockMapElement::new())];

        static motion_left: [f64; 2] = [3.0, 5.0];
        static motion_right: [f64; 2] = [-7.0, 9.0];

        call_move_event(&mut events, &mut seq, motion_left);
        player
            .expect_rotate_left()
            .times(1)
            .withf(|radians| *radians == Radians::new(motion_left[0] / 1000.0))
            .return_const(())
            .in_sequence(&mut seq);

        call_move_event(&mut events, &mut seq, motion_right);
        player
            .expect_rotate_right()
            .times(1)
            .withf(|radians| *radians == Radians::new(motion_right[0].abs() / 1000.0))
            .return_const(())
            .in_sequence(&mut seq);

        call_none_event(&mut events, &mut seq);

        let mut engine = Engine {
            generator,
            player,
            window,
            events,
            graphics,
            map_elements,
        };

        engine.start();
    }

    #[test]
    fn start_key_event() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let mut player = MockPlayer::default();
        let window = Window {};
        let mut events = MockEvents::default();
        let graphics = Graphics {};
        let map_elements: Vec<Box<dyn MapElement>> = vec![Box::new(MockMapElement::new())];

        call_key_event(&mut events, &mut seq, input::Key::W, ButtonState::Press);
        expect_move_forward(&mut player, &mut seq, true);

        call_key_event(&mut events, &mut seq, input::Key::W, ButtonState::Release);
        expect_move_forward(&mut player, &mut seq, false);

        call_key_event(&mut events, &mut seq, input::Key::S, ButtonState::Press);
        expect_move_backward(&mut player, &mut seq, true);

        call_key_event(&mut events, &mut seq, input::Key::S, ButtonState::Release);
        expect_move_backward(&mut player, &mut seq, false);

        call_key_event(&mut events, &mut seq, input::Key::A, ButtonState::Press);
        expect_move_left(&mut player, &mut seq, true);

        call_key_event(&mut events, &mut seq, input::Key::A, ButtonState::Release);
        expect_move_left(&mut player, &mut seq, false);

        call_key_event(&mut events, &mut seq, input::Key::D, ButtonState::Press);
        expect_move_right(&mut player, &mut seq, true);

        call_key_event(&mut events, &mut seq, input::Key::D, ButtonState::Release);
        expect_move_right(&mut player, &mut seq, false);

        call_none_event(&mut events, &mut seq);

        let mut engine = Engine {
            generator,
            player,
            window,
            events,
            graphics,
            map_elements,
        };

        engine.start();
    }

    #[test]
    fn start_update_event_position_not_updated() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let mut player = MockPlayer::default();
        let window = Window {};
        let mut events = MockEvents::default();
        let graphics = Graphics {};

        let mut map_element = Box::new(MockMapElement::new());

        let delta_time = 2.0;

        events
            .expect_next_event()
            .times(1)
            .return_const(Some(piston::Event::Loop(piston::Loop::Update(
                UpdateArgs {
                    dt: delta_time.clone(),
                },
            ))))
            .in_sequence(&mut seq);
        player
            .expect_update()
            .times(1)
            .return_const(false)
            .in_sequence(&mut seq);
        map_element
            .expect_update()
            .times(1)
            .withf(move |time_elapsed| *time_elapsed == delta_time)
            .return_const(())
            .in_sequence(&mut seq);

        call_none_event(&mut events, &mut seq);

        let map_elements: Vec<Box<dyn MapElement>> = vec![map_element];
        let mut engine = Engine {
            generator,
            player,
            window,
            events,
            graphics,
            map_elements,
        };

        engine.start();
    }

    #[test]
    fn start_update_event_position_updated() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let mut player = MockPlayer::default();
        let window = Window {};
        let mut events = MockEvents::default();
        let graphics = Graphics {};

        let mut map_element = Box::new(MockMapElement::new());

        let delta_time = 2.0;

        events
            .expect_next_event()
            .times(1)
            .return_const(Some(piston::Event::Loop(piston::Loop::Update(
                UpdateArgs {
                    dt: delta_time.clone(),
                },
            ))))
            .in_sequence(&mut seq);
        player
            .expect_update()
            .times(1)
            .return_const(true)
            .in_sequence(&mut seq);
        player
            .expect_position()
            .times(1)
            .return_const(Coordinate { x: 10.0, y: 20.0 })
            .in_sequence(&mut seq);
        map_element
            .expect_on_position_update()
            .times(1)
            .withf(|position| *position == Coordinate { x: 10.0, y: 20.0 })
            .return_const(())
            .in_sequence(&mut seq);
        map_element
            .expect_update()
            .times(1)
            .withf(move |time_elapsed| *time_elapsed == delta_time)
            .return_const(())
            .in_sequence(&mut seq);

        call_none_event(&mut events, &mut seq);

        let map_elements: Vec<Box<dyn MapElement>> = vec![map_element];
        let mut engine = Engine {
            generator,
            player,
            window,
            events,
            graphics,
            map_elements,
        };

        engine.start();
    }
}
