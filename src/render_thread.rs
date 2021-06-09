use crate::graph::{Coordinate, Walls};
use crate::map_element::MapElement;
use crate::map_element::Point;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::graph::MockLinearGraph as LinearGraph;
        use crate::player_utils::MockPlayer as Player;
        use crate::map::MockMap as Map;
    } else {
        use crate::graph::LinearGraph;
        use crate::player_utils::Player;
        use crate::map::Map;
    }
}

pub struct RenderThread {
    pub map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>>,
    pub player: Arc<RwLock<Player>>,
    pub map: Arc<Map>,
    pub rays: Arc<Vec<LinearGraph>>,
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
                player.get_rays_angle_range(self.thread_index, self.threads_amount),
                &map_elements,
            );
            self.sender_walls.send((walls, self.thread_index)).unwrap();
        }
    }

    fn get_walls_in_sight(
        &self,
        position: &Coordinate,
        rays_indexes_vec: std::vec::Vec<std::ops::Range<usize>>,
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Walls {
        let mut walls_in_sight = Walls(vec![]);
        let mut last_points: Vec<Point> = Vec::with_capacity(2);
        for rays_indexes in rays_indexes_vec {
            for index in rays_indexes {
                let mut points = self.map.cast_ray(position, &self.rays[index], map_elements);
                handle_points(&mut last_points, &mut walls_in_sight, &mut points);
            }
        }
        return walls_in_sight;
    }
}

fn handle_points(last_points: &mut Vec<Point>, walls: &mut Walls, new_points: &mut Vec<Point>) {
    if let Some(point) = walls.try_extend_last_wall(new_points) {
        handle_one_point(last_points, walls, point);
    }
}

fn handle_one_point(last_points: &mut Vec<Point>, walls: &mut Walls, new_point: Point) {
    if last_points.len() < 2 {
        last_points.push(new_point);
        if last_points.len() == 2 {
            if last_points[0].distance(&last_points[1]) == 1.0 {
                walls.try_extend_last_wall(last_points);
            }
            last_points.clear();
        }
    } else {
        last_points.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MockLinearGraph;
    use crate::graph::Wall;
    use crate::map::MockMap;
    use crate::player_utils::MockPlayer;
    use mockall::*;
    use std::sync::mpsc::channel;

    fn get_rays(amount: usize) -> Arc<Vec<MockLinearGraph>> {
        let mut rays = Vec::with_capacity(amount);
        for _ in 0..amount {
            rays.push(MockLinearGraph::new());
        }
        Arc::new(rays)
    }

    #[test]
    fn render_thread_start() {
        let mut seq = Sequence::new();

        let map_elements: Arc<RwLock<Vec<Box<dyn MapElement>>>> = Arc::new(RwLock::new(vec![]));

        let player = Arc::new(RwLock::new(MockPlayer::default()));
        let mut map = MockMap::default();
        let rays = get_rays(9);
        let (start_render_sender, start_render_receiver) = channel::<bool>();
        let (sender_walls, receiver_walls) = channel::<(Walls, usize)>();
        let thread_index = 3;
        let threads_amount = 4;

        let player_position = Coordinate { x: 10.0, y: 20.0 };
        let vec_points = vec![
            vec![Point { x: 1, y: 4 }, Point { x: 2, y: 4 }],
            vec![Point { x: 2, y: 4 }, Point { x: 2, y: 3 }],
            vec![Point { x: 2, y: 3 }, Point { x: 3, y: 3 }],
            vec![Point { x: 3, y: 4 }],
            vec![Point { x: 4, y: 4 }],
            vec![Point { x: 4, y: 5 }, Point { x: 5, y: 5 }],
            vec![Point { x: 5, y: 6 }, Point { x: 6, y: 6 }],
            vec![Point { x: 6, y: 6 }, Point { x: 7, y: 6 }],
        ];
        let walls_in_sight = Walls(vec![
            Wall {
                start_point: Point { x: 1, y: 4 },
                end_point: Point { x: 2, y: 4 },
            },
            Wall {
                start_point: Point { x: 2, y: 4 },
                end_point: Point { x: 2, y: 3 },
            },
            Wall {
                start_point: Point { x: 2, y: 3 },
                end_point: Point { x: 3, y: 3 },
            },
            Wall {
                start_point: Point { x: 3, y: 4 },
                end_point: Point { x: 4, y: 4 },
            },
            Wall {
                start_point: Point { x: 4, y: 5 },
                end_point: Point { x: 5, y: 5 },
            },
            Wall {
                start_point: Point { x: 5, y: 6 },
                end_point: Point { x: 7, y: 6 },
            },
        ]);

        {
            let mut player_write = player.write().unwrap();
            player_write
                .expect_position()
                .times(1)
                .return_const(player_position.clone())
                .in_sequence(&mut seq);
            let thread_index_clone = thread_index.clone();
            player_write
                .expect_get_rays_angle_range()
                .times(1)
                .withf(move |thread_index_, threads_amount_| {
                    *thread_index_ == thread_index_clone && *threads_amount_ == threads_amount
                })
                .return_const(vec![std::ops::Range { start: 1, end: 9 }])
                .in_sequence(&mut seq);
        }

        for points in vec_points {
            let player_position_clone = player_position.clone();
            map.expect_cast_ray()
                .times(1)
                .withf(move |position, _, _| *position == player_position_clone)
                .return_const(points)
                .in_sequence(&mut seq);
        }

        let render_thread = RenderThread {
            map_elements,
            player,
            map: Arc::new(map),
            rays,
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
