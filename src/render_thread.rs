use crate::graph::{Coordinate, LinearGraph, Walls};
use crate::map_element::MapElement;
use mockall_double::double;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::{thread, thread::JoinHandle};

#[double]
use crate::graph::GraphMethods;
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

macro_rules! check_next_ray {
    ($next_ray:expr, $rays_iter:expr, $walls_in_sight:expr) => {
        $next_ray = $rays_iter.next();
        if $next_ray.is_none() {
            return $walls_in_sight;
        }
    };
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
        mut rays_iter: impl Iterator<Item = &'a LinearGraph>,
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Walls {
        let mut walls_in_sight = Walls(vec![]);
        let mut next_ray: Option<&LinearGraph> = None;
        let mut current_ray: LinearGraph;
        loop {
            if next_ray.is_none() {
                check_next_ray!(next_ray, rays_iter, walls_in_sight);
            }
            current_ray = next_ray.unwrap().clone();
            next_ray = None;
            loop {
                if let Some((wall, ray_ret)) =
                    self.map.cast_ray(position, &current_ray, map_elements)
                {
                    if walls_in_sight.is_wall_in_object(&wall) {
                        break;
                    }
                    if !walls_in_sight.is_wall_connected(&wall) && next_ray.is_some() {
                        break;
                    } else {
                        walls_in_sight.try_extend_last_wall(wall);
                    }
                    if next_ray.is_none() {
                        check_next_ray!(next_ray, rays_iter, walls_in_sight);
                    }
                    while GraphMethods::less_than(next_ray.unwrap(), &current_ray) {
                        check_next_ray!(next_ray, rays_iter, walls_in_sight);
                    }

                    current_ray = ray_ret;
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::*;
    use crate::graph::MockGraphMethods;
    use crate::graph::{MockRays, MockRaysIterator, Wall};
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
            static ref ray_vec_iter: Vec<LinearGraph> = vec![
                LinearGraph::default(),
                LinearGraph::default(),
                LinearGraph::default(),
                LinearGraph::default(),
                LinearGraph::default(),
                LinearGraph::default(),
                LinearGraph::default(),
                LinearGraph::default(),
                LinearGraph::default(),
            ];
        }

        let less_than_context = MockGraphMethods::less_than_context();

        let (start_render_sender, start_render_receiver) = channel::<bool>();
        let (sender_walls, receiver_walls) = channel::<(Walls, usize)>();
        static thread_index: usize = 3;
        static threads_amount: usize = 4;

        static player_position: Coordinate = Coordinate { x: 10.0, y: 20.0 };
        static player_angle: Angle = Angle {
            start: Radians::PI,
            end: Radians::PI_2,
        };
        let walls_rays = vec![
            Some((
                Wall::new(Point { x: 2, y: 4 }, Point { x: 3, y: 4 }, Color::Red),
                LinearGraph::default(),
            )),
            Some((
                Wall::new(Point { x: 3, y: 4 }, Point { x: 4, y: 4 }, Color::Red),
                LinearGraph::default(),
            )),
            Some((
                Wall::new(Point { x: 4, y: 2 }, Point { x: 5, y: 2 }, Color::Red),
                LinearGraph::default(),
            )),
            Some((
                Wall::new(Point { x: 3, y: 4 }, Point { x: 4, y: 4 }, Color::Red),
                LinearGraph::default(),
            )),
            Some((
                Wall::new(Point { x: 4, y: 2 }, Point { x: 5, y: 2 }, Color::Red),
                LinearGraph::default(),
            )),
            Some((
                Wall::new(Point { x: 5, y: 2 }, Point { x: 5, y: 1 }, Color::Green),
                LinearGraph::default(),
            )),
            None,
            Some((
                Wall::new(Point { x: 5, y: 1 }, Point { x: 5, y: 0 }, Color::Green),
                LinearGraph::default(),
            )),
        ];

        let walls_in_sight = Walls(vec![
            Wall::new(Point { x: 2, y: 4 }, Point { x: 4, y: 4 }, Color::Red),
            Wall::new(Point { x: 4, y: 2 }, Point { x: 5, y: 2 }, Color::Red),
            Wall::new(Point { x: 5, y: 2 }, Point { x: 5, y: 0 }, Color::Green),
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
                for ray_ in ray_vec_iter.iter() {
                    rays_iterator
                        .expect_next()
                        .once()
                        .returning(move || Some(&ray_));
                }
                rays_iterator.expect_next().once().return_const(None);
                return rays_iterator;
            });

        for wall_ray in walls_rays.into_iter() {
            map.expect_cast_ray().once().return_const(wall_ray);
        }

        for _ in 0..4 {
            less_than_context.expect().once().return_const(true);
            less_than_context.expect().once().return_const(false);
        }
        less_than_context.expect().once().return_const(true);

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
