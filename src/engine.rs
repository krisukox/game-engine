use crate::map_element::MapElement;
use crate::player_utils::Radians;
use graphics::types::Color;
use graphics::Transformed;
use piston::input::{ButtonEvent, MouseRelativeEvent, RenderEvent, UpdateEvent};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

cfg_if::cfg_if! {
    if #[cfg(test)]{
        use crate::generator::MockObjectGenerator as ObjectGenerator;
        use crate::player_utils::MockPlayer as Player;
        use crate::wrapper::MockEvents as Events;
        use crate::wrapper::MockGraphics as Graphics;
        use crate::wrapper::test_utils::Window as GlutinWindow;
        use crate::wrapper::test_utils::GlGraphics;
    } else {
        use crate::wrapper::Events;
        use crate::wrapper::Graphics;
        use crate::graph::Walls;
        use crate::map::Map;
        use crate::generator::ObjectGenerator;
        use crate::player_utils::Player;
        use crate::generator::PolygonGenerator;
        use crate::generator::PointGenerator;
        use crate::render_thread::RenderThread;
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
    window: GlutinWindow,
    graphics: GlGraphics,
    events: Events,
    player: Arc<RwLock<Player>>,
    map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>>,
    start_render_notifiers: Vec<Sender<bool>>,
    render_threads: Vec<JoinHandle<()>>,
}

const BACKGROUND_COLOR: Color = [0.8, 0.8, 0.8, 1.0];

impl Engine {
    #[cfg(not(tarpaulin_include))]
    #[cfg(not(test))]
    pub fn new(
        resolution: Size,
        vertical_angle_value: Radians,
        wall_height: f64,
        map: Map,
        player: Player,
        map_elements: Vec<Box<dyn MapElement>>,
        render_threads_amount: i64,
    ) -> Engine {
        let polygon_generator = PolygonGenerator {
            point_generator: PointGenerator::new(resolution, vertical_angle_value, wall_height),
        };
        let map = Arc::new(map);
        let rays = Arc::new(player.get_all_rays());
        let player = Arc::new(RwLock::new(player));
        let map_elements = Arc::new(RwLock::new(map_elements));
        let mut start_render_notifiers = vec![];
        let render_threads_amount = Self::limit_threads_amount(render_threads_amount);
        let mut render_threads = Vec::with_capacity(render_threads_amount);

        let (sender_walls, receiver_walls) = channel::<(Walls, usize)>();

        for thread_index in 0..render_threads_amount {
            let (start_render_notifier, start_render_receiver) = channel::<bool>();
            let render_thread = RenderThread {
                map_elements: Arc::clone(&map_elements),
                player: Arc::clone(&player),
                map: Arc::clone(&map),
                rays: Arc::clone(&rays),
                start_render_receiver,
                sender_walls: sender_walls.clone(),
                thread_index,
                threads_amount: render_threads_amount,
            };
            render_threads.push(RenderThread::start_thread(render_thread));
            start_render_notifiers.push(start_render_notifier);
        }

        Engine {
            generator: ObjectGenerator {
                polygon_generator,
                receiver_walls,
                render_threads_amount,
            },
            window: Self::create_window(resolution),
            graphics: GlGraphics::new(OPENGL_VERSION),
            events: Events::new(),
            player,
            map_elements,
            start_render_notifiers,
            render_threads,
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
        window.ctx.window().set_resizable(false);
        window.ctx.window().set_maximized(false);
        window.set_capture_cursor(true);
        return window;
    }

    fn limit_threads_amount(render_threads_amount: i64) -> usize {
        if render_threads_amount <= 1 {
            return 1;
        } else if render_threads_amount >= 4 {
            return 4;
        }
        return render_threads_amount as usize;
    }

    pub fn start(&mut self) {
        while let Some(e) = self.events.next_event(&mut self.window) {
            if let Some(args) = e.render_args() {
                for start_render_notifier in &self.start_render_notifiers {
                    start_render_notifier.send(true).unwrap();
                }
                let polygons = self.generator.generate_polygons(&self.player);
                self.graphics.draw(args.viewport(), |c, g| {
                    let transform = c
                        .transform
                        .flip_v()
                        .trans(0.0, -(c.viewport.unwrap().draw_size[1] as f64 / 2.0));
                    Graphics::clear(g, BACKGROUND_COLOR);
                    for polygon in polygons {
                        Graphics::draw_polygon(
                            g,
                            polygon.color.into(),
                            polygon.area,
                            &c.draw_state,
                            transform,
                        );
                    }
                });
            }

            if let Some(args) = e.mouse_relative_args() {
                let mut player = self.player.write().unwrap();
                if args[0] > 0.0 {
                    player.rotate_left(Radians::new(args[0] / 1000.0));
                } else {
                    player.rotate_right(Radians::new((args[0] / 1000.0).abs()));
                }
            }

            if let Some(args) = e.button_args() {
                if let piston::input::Button::Keyboard(key) = args.button {
                    let mut player = self.player.write().unwrap();
                    match key {
                        piston::input::Key::W => {
                            player.move_forward(into_bool(args.state));
                        }
                        piston::input::Key::S => {
                            player.move_backward(into_bool(args.state));
                        }
                        piston::input::Key::A => {
                            player.move_left(into_bool(args.state));
                        }
                        piston::input::Key::D => {
                            player.move_right(into_bool(args.state));
                        }
                        _ => {}
                    }
                }
            }

            if let Some(args) = e.update_args() {
                let mut map_elements = self.map_elements.write().unwrap();
                let mut player = self.player.write().unwrap();
                if player.update() {
                    for map_element in &mut *map_elements {
                        map_element.as_mut().on_position_update(player.position());
                    }
                }
                for map_element in &mut *map_elements {
                    map_element.as_mut().update(args.dt);
                }
            }
        }

        for sender_notifier in &self.start_render_notifiers {
            sender_notifier.send(false).unwrap();
        }

        for thread in self.render_threads.drain(0..).collect::<Vec<_>>() {
            thread.join().unwrap();
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
    use crate::generator::MockObjectGenerator;
    use crate::generator::Polygon;
    use crate::graph::Coordinate;
    use crate::map_element::Color;
    use crate::map_element::MockMapElement;
    use crate::player_utils::{MockPlayer, Radians};
    use crate::wrapper::test_utils::GlGraphics;
    use crate::wrapper::test_utils::Window;
    use crate::wrapper::MockEvents;
    use crate::wrapper::MockGraphics;
    use mockall::*;
    use piston::input::*;
    use piston::*;

    const RENDER_THREADS_AMOUNT: usize = 2;

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
    fn limit_threads_amount() {
        assert_eq!(Engine::limit_threads_amount(3), 3);
        assert_eq!(Engine::limit_threads_amount(6), 4);
        assert_eq!(Engine::limit_threads_amount(0), 1);
    }

    #[test]
    fn start_render_event() {
        let mut seq = Sequence::new();

        let mut generator = MockObjectGenerator::new();
        let window = Window {};
        let graphics = GlGraphics {};
        let mut events = MockEvents::default();
        let player = Arc::new(RwLock::new(MockPlayer::default()));
        let map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>> =
            Arc::new(RwLock::new(vec![Box::new(MockMapElement::new())]));

        let mut start_render_notifiers = vec![];
        let mut start_render_receivers = vec![];

        for _ in 0..RENDER_THREADS_AMOUNT {
            let (start_render_notifier, start_render_receiver) = channel::<bool>();
            start_render_notifiers.push(start_render_notifier);
            start_render_receivers.push(start_render_receiver);
        }

        let clear_ctx = MockGraphics::clear_context();
        let draw_polygon_ctx = MockGraphics::draw_polygon_context();

        let polygons = vec![
            Polygon {
                area: [[0.0, 1.0], [2.0, 3.0], [4.0, 5.0], [6.0, 7.0]],
                color: Color::Red,
            },
            Polygon {
                area: [[8.0, 9.0], [10.0, 11.0], [12.0, 13.0], [14.0, 15.0]],
                color: Color::Yellow,
            },
        ];

        let event = piston::Event::Loop(Loop::Render(RenderArgs {
            ext_dt: 1.0,
            window_size: [2.0, 3.0],
            draw_size: [1, 2],
        }));
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

        for polygon in polygons.into_iter() {
            draw_polygon_ctx
                .expect()
                .times(1)
                .withf(move |_, color, polygon_, _, _| {
                    *color == Into::<[f32; 4]>::into(polygon.color.clone())
                        && *polygon_ == polygon.area
                })
                .return_const(())
                .in_sequence(&mut seq);
        }

        call_none_event(&mut events, &mut seq);

        let mut engine = Engine {
            generator,
            window,
            graphics,
            events,
            player,
            map_elements,
            start_render_notifiers,
            render_threads: vec![],
        };

        engine.start();

        for start_render_receiver in &start_render_receivers {
            assert_eq!(start_render_receiver.recv().unwrap(), true);
        }
    }

    #[test]
    fn start_mouse_event() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let window = Window {};
        let graphics = GlGraphics {};
        let mut events = MockEvents::default();
        let player = Arc::new(RwLock::new(MockPlayer::default()));
        let map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>> =
            Arc::new(RwLock::new(vec![Box::new(MockMapElement::new())]));

        static motion_left: [f64; 2] = [3.0, 5.0];
        static motion_right: [f64; 2] = [-7.0, 9.0];

        {
            let mut player_write = player.write().unwrap();

            call_move_event(&mut events, &mut seq, motion_left);
            player_write
                .expect_rotate_left()
                .times(1)
                .withf(|radians| *radians == Radians::new(motion_left[0] / 1000.0))
                .return_const(())
                .in_sequence(&mut seq);

            call_move_event(&mut events, &mut seq, motion_right);
            player_write
                .expect_rotate_right()
                .times(1)
                .withf(|radians| *radians == Radians::new(motion_right[0].abs() / 1000.0))
                .return_const(())
                .in_sequence(&mut seq);

            call_none_event(&mut events, &mut seq);
        }

        let mut engine = Engine {
            generator,
            window,
            graphics,
            events,
            player,
            map_elements,
            start_render_notifiers: vec![],
            render_threads: vec![],
        };

        engine.start();
    }

    #[test]
    fn start_key_event() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let player = Arc::new(RwLock::new(MockPlayer::default()));
        let window = Window {};
        let mut events = MockEvents::default();
        let graphics = GlGraphics {};
        let map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>> =
            Arc::new(RwLock::new(vec![Box::new(MockMapElement::new())]));
        {
            let mut player_write = player.write().unwrap();

            call_key_event(&mut events, &mut seq, input::Key::W, ButtonState::Press);
            expect_move_forward(&mut player_write, &mut seq, true);

            call_key_event(&mut events, &mut seq, input::Key::W, ButtonState::Release);
            expect_move_forward(&mut player_write, &mut seq, false);

            call_key_event(&mut events, &mut seq, input::Key::S, ButtonState::Press);
            expect_move_backward(&mut player_write, &mut seq, true);

            call_key_event(&mut events, &mut seq, input::Key::S, ButtonState::Release);
            expect_move_backward(&mut player_write, &mut seq, false);

            call_key_event(&mut events, &mut seq, input::Key::A, ButtonState::Press);
            expect_move_left(&mut player_write, &mut seq, true);

            call_key_event(&mut events, &mut seq, input::Key::A, ButtonState::Release);
            expect_move_left(&mut player_write, &mut seq, false);

            call_key_event(&mut events, &mut seq, input::Key::D, ButtonState::Press);
            expect_move_right(&mut player_write, &mut seq, true);

            call_key_event(&mut events, &mut seq, input::Key::D, ButtonState::Release);
            expect_move_right(&mut player_write, &mut seq, false);

            call_none_event(&mut events, &mut seq);
        }

        let mut engine = Engine {
            generator,
            window,
            graphics,
            events,
            player,
            map_elements,
            start_render_notifiers: vec![],
            render_threads: vec![],
        };

        engine.start();
    }

    #[test]
    fn start_update_event_position_not_updated() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let window = Window {};
        let graphics = GlGraphics {};
        let mut events = MockEvents::default();
        let player = Arc::new(RwLock::new(MockPlayer::default()));

        let mut map_element = Box::new(MockMapElement::new());

        let delta_time = 2.0;

        {
            let mut player_write = player.write().unwrap();
            events
                .expect_next_event()
                .times(1)
                .return_const(Some(piston::Event::Loop(piston::Loop::Update(
                    UpdateArgs {
                        dt: delta_time.clone(),
                    },
                ))))
                .in_sequence(&mut seq);
            player_write
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
        }

        let map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>> =
            Arc::new(RwLock::new(vec![map_element]));
        let mut engine = Engine {
            generator,
            window,
            graphics,
            events,
            player,
            map_elements,
            start_render_notifiers: vec![],
            render_threads: vec![],
        };

        engine.start();
    }

    #[test]
    fn start_update_event_position_updated() {
        let mut seq = Sequence::new();

        let generator = MockObjectGenerator::new();
        let window = Window {};
        let graphics = GlGraphics {};
        let mut events = MockEvents::default();
        let player = Arc::new(RwLock::new(MockPlayer::default()));

        let mut map_element = Box::new(MockMapElement::new());

        let delta_time = 2.0;
        let position = Coordinate { x: 10.0, y: 20.0 };

        {
            let mut player_write = player.write().unwrap();
            events
                .expect_next_event()
                .times(1)
                .return_const(Some(piston::Event::Loop(piston::Loop::Update(
                    UpdateArgs {
                        dt: delta_time.clone(),
                    },
                ))))
                .in_sequence(&mut seq);
            player_write
                .expect_update()
                .times(1)
                .return_const(true)
                .in_sequence(&mut seq);
            player_write
                .expect_position()
                .times(1)
                .return_const(position.clone())
                .in_sequence(&mut seq);
            map_element
                .expect_on_position_update()
                .times(1)
                .withf(move |position_| *position_ == position)
                .return_const(())
                .in_sequence(&mut seq);
            map_element
                .expect_update()
                .times(1)
                .withf(move |time_elapsed| *time_elapsed == delta_time)
                .return_const(())
                .in_sequence(&mut seq);
        }

        call_none_event(&mut events, &mut seq);

        let map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>> =
            Arc::new(RwLock::new(vec![map_element]));

        let mut engine = Engine {
            generator,
            window,
            graphics,
            events,
            player,
            map_elements,
            start_render_notifiers: vec![],
            render_threads: vec![],
        };

        engine.start();
    }
}
