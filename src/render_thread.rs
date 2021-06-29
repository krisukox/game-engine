use crate::graph::{Coordinate, Walls};
use crate::map_element::MapElement;
use mockall_double::double;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::{thread, thread::JoinHandle};

#[double]
use crate::graph::LinearGraph;
#[double]
use crate::graph::Rays;
#[double]
use crate::map::Map;
#[double]
use crate::player_utils::Player;

pub struct RenderThread {
    pub map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>>,
    pub player: Arc<RwLock<Player>>,
    pub map: Arc<Map>,
    pub rays: Arc<Rays>,
    pub start_render_receiver: Receiver<bool>,
    pub sender_walls: Sender<(Walls, usize)>,
    pub thread_index: usize,
    pub threads_amount: usize,
}

impl RenderThread {
    pub fn start_thread(render_thread: RenderThread) -> JoinHandle<()> {
        thread::spawn(move || {
            render_thread.start();
        })
    }

    fn start(&self) {
        while let Ok(true) = self.start_render_receiver.recv() {
            let map_elements = self.map_elements.read().unwrap();
            let player = self.player.read().unwrap();
            let walls = self.get_walls_in_sight(
                player.position(),
                self.rays
                    .iter(player.angle(), self.thread_index, self.threads_amount),
                &map_elements,
            );
            self.sender_walls.send((walls, self.thread_index)).unwrap();
        }
    }

    fn get_walls_in_sight<'a>(
        &self,
        position: &Coordinate,
        rays_iter: impl Iterator<Item = &'a LinearGraph>,
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Walls {
        let mut walls_in_sight = Walls(vec![]);
        for ray in rays_iter {
            let option_wall = self.map.cast_ray(position, ray, map_elements);
            if let Some(wall) = option_wall {
                walls_in_sight.try_extend_last_wall(wall)
            }
        }
        return walls_in_sight;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::*;
    use crate::graph::{MockLinearGraph, MockRays, MockRaysIterator, Wall};
    use crate::map::MockMap;
    use crate::map_element::{Color, Point};
    use crate::player_utils::{Angle, MockPlayer, Radians};
    use mockall::*;
    use std::sync::mpsc::channel;

    #[test]
    fn render_thread_start() {
        let mut seq = Sequence::new();

        let map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>> = Arc::new(RwLock::new(vec![]));
        let player = Arc::new(RwLock::new(MockPlayer::default()));
        let mut map = MockMap::default();
        let mut rays = MockRays::new();
        lazy_static! {
            static ref ray: MockLinearGraph = MockLinearGraph::new();
        }

        let (start_render_sender, start_render_receiver) = channel::<bool>();
        let (sender_walls, receiver_walls) = channel::<(Walls, usize)>();
        static thread_index: usize = 3;
        static threads_amount: usize = 4;

        static player_position: Coordinate = Coordinate { x: 10.0, y: 20.0 };
        static player_angle: Angle = Angle {
            start: Radians::PI,
            end: Radians::PI_2,
        };
        let walls = vec![
            Wall::new(Point { x: 1, y: 4 }, Point { x: 2, y: 4 }, Color::Red),
            Wall::new(Point { x: 2, y: 4 }, Point { x: 2, y: 3 }, Color::Red),
            Wall::new(Point { x: 2, y: 4 }, Point { x: 2, y: 3 }, Color::Red),
            Wall::new(Point { x: 2, y: 3 }, Point { x: 3, y: 3 }, Color::Red),
            Wall::new(Point { x: 3, y: 4 }, Point { x: 4, y: 4 }, Color::Red),
            Wall::new(Point { x: 4, y: 5 }, Point { x: 5, y: 5 }, Color::Red),
            Wall::new(Point { x: 5, y: 6 }, Point { x: 6, y: 6 }, Color::Red),
            Wall::new(Point { x: 6, y: 6 }, Point { x: 7, y: 6 }, Color::Red),
            Wall::new(Point { x: 7, y: 6 }, Point { x: 8, y: 6 }, Color::Green),
        ];
        let walls_in_sight = Walls(vec![
            Wall::new(Point { x: 1, y: 4 }, Point { x: 2, y: 4 }, Color::Red),
            Wall::new(Point { x: 2, y: 4 }, Point { x: 2, y: 3 }, Color::Red),
            Wall::new(Point { x: 2, y: 3 }, Point { x: 3, y: 3 }, Color::Red),
            Wall::new(Point { x: 3, y: 4 }, Point { x: 4, y: 4 }, Color::Red),
            Wall::new(Point { x: 4, y: 5 }, Point { x: 5, y: 5 }, Color::Red),
            Wall::new(Point { x: 5, y: 6 }, Point { x: 7, y: 6 }, Color::Red),
            Wall::new(Point { x: 7, y: 6 }, Point { x: 8, y: 6 }, Color::Green),
        ]);

        {
            let mut player_write = player.write().unwrap();
            player_write
                .expect_position()
                .times(1)
                .return_const(player_position.clone())
                .in_sequence(&mut seq);
            player_write
                .expect_angle()
                .times(1)
                .return_const(player_angle.clone())
                .in_sequence(&mut seq);
        }

        rays.expect_iter()
            .times(1)
            .withf(|angle, _thread_index, _threads_amount| {
                *angle == player_angle
                    && *_thread_index == thread_index
                    && *_threads_amount == threads_amount
            })
            .returning(|_, _, _| {
                let mut rays_iterator = Box::new(MockRaysIterator::default());
                rays_iterator
                    .expect_next()
                    .times(9)
                    .returning(|| Some(&ray));
                rays_iterator.expect_next().times(1).return_const(None);
                return rays_iterator;
            });

        for wall in walls {
            map.expect_cast_ray()
                .times(1)
                .withf(|position, _, _| *position == player_position)
                .return_const(wall);
        }

        let render_thread = RenderThread {
            map_elements,
            player,
            map: Arc::new(map),
            rays: Arc::new(rays),
            start_render_receiver,
            sender_walls,
            thread_index,
            threads_amount,
        };
        start_render_sender.send(true).unwrap();
        start_render_sender.send(false).unwrap();

        render_thread.start();

        assert_eq!(
            receiver_walls.recv().unwrap(),
            (walls_in_sight, thread_index)
        );
    }
}
