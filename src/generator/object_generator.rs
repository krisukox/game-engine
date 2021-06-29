use crate::generator::Polygon;
use crate::graph;
use crate::graph::Walls;
use crate::player_utils;
use mockall_double::double;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};

#[cfg(test)]
use mockall::automock;

#[double]
use super::polygon_generator::PolygonGenerator;
#[double]
use crate::player_utils::Player;

pub struct ObjectGenerator {
    pub polygon_generator: PolygonGenerator,
    pub receiver_walls: Receiver<(Walls, usize)>,
    pub render_threads_amount: usize,
}

#[cfg_attr(test, automock)]
impl ObjectGenerator {
    fn receive_and_merge_walls(&self) -> Option<graph::Walls> {
        let mut walls_map: HashMap<usize, Walls> =
            HashMap::with_capacity(self.render_threads_amount);
        for _ in 0..self.render_threads_amount {
            let (walls, index) = self.receiver_walls.recv().unwrap();
            walls_map.insert(index, walls);
        }
        if let Some(mut walls) = walls_map.remove(&0) {
            for index in 1..self.render_threads_amount {
                if let Some(walls_to_merge) = walls_map.remove(&index) {
                    walls.merge(walls_to_merge);
                }
            }
            return Some(walls);
        }
        return None;
    }

    fn generate_farther_polygons(
        &self,
        index: usize,
        walls_in_sight: &Vec<graph::Wall>,
        position: &graph::Coordinate,
        angle: &player_utils::Angle,
        polygons: &mut Vec<Polygon>,
    ) -> usize {
        if index < walls_in_sight.len() - 1
            && walls_in_sight[index].point_distance_end(position)
                < walls_in_sight[index + 1].point_distance_start(position)
        {
            let ret_index = self.generate_farther_polygons(
                index + 1,
                walls_in_sight,
                position,
                angle,
                polygons,
            );
            polygons.push(self.polygon_generator.generate_polygon(
                &walls_in_sight[index],
                position,
                angle,
            ));
            return ret_index;
        }
        polygons.push(self.polygon_generator.generate_polygon(
            &walls_in_sight[index],
            position,
            angle,
        ));
        return index;
    }

    pub fn generate_polygons_(
        &self,
        walls_in_sight: graph::Walls,
        position: &graph::Coordinate,
        angle: &player_utils::Angle,
    ) -> Vec<Polygon> {
        let mut polygons: Vec<Polygon> = Vec::new();
        let mut index = 0;
        if walls_in_sight.0.len() == 0 {
            return vec![];
        }
        while index < walls_in_sight.0.len() - 1 {
            if walls_in_sight.0[index].point_distance_end(&position)
                >= walls_in_sight.0[index + 1].point_distance_start(&position)
            {
                polygons.push(self.polygon_generator.generate_polygon(
                    &walls_in_sight.0[index],
                    &position,
                    &angle,
                ));
            } else {
                index = self.generate_farther_polygons(
                    index,
                    &walls_in_sight.0,
                    &position,
                    &angle,
                    &mut polygons,
                );
            }
            index += 1;
        }
        if index == walls_in_sight.0.len() - 1 {
            polygons.push(self.polygon_generator.generate_polygon(
                &walls_in_sight.0[index],
                &position,
                &angle,
            ));
        }
        return polygons;
    }

    pub fn generate_polygons(&self, player: &Arc<RwLock<Player>>) -> Vec<Polygon> {
        if let Some(merged_walls) = self.receive_and_merge_walls() {
            let player_read = player.read().unwrap();
            return self.generate_polygons_(
                merged_walls,
                player_read.position(),
                player_read.angle(),
            );
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::*;
    use crate::generator::MockPolygonGenerator;
    use crate::generator::Polygon;
    use crate::map_element::{Color, Point};
    use crate::player_utils::Angle;
    use crate::player_utils::MockPlayer;
    use crate::player_utils::Radians;
    use lazy_static::*;
    use mockall::*;
    use std::sync::mpsc;

    #[test]
    fn generate_polygons_farther() {
        let mut seq = Sequence::new();

        let render_threads_amount = 2;

        let mut polygon_generator = MockPolygonGenerator::new();
        let player = Arc::new(RwLock::new(MockPlayer::default()));
        let (sender_walls, receiver_walls) = mpsc::channel::<(Walls, usize)>();

        {
            let mut player_write = player.write().unwrap();
            player_write
                .expect_position()
                .times(1)
                .return_const(graph::Coordinate { x: 2.0, y: 1.0 })
                .in_sequence(&mut seq);
            player_write
                .expect_angle()
                .times(1)
                .return_const(Angle {
                    start: Radians::new(0.0),
                    end: Radians::new(1.0),
                })
                .in_sequence(&mut seq);
        }

        let walls_in_sight = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 1, y: 4 },
                end_point: Point { x: 2, y: 4 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 2, y: 4 },
                end_point: Point { x: 2, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 2, y: 3 },
                end_point: Point { x: 3, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 3, y: 4 },
                end_point: Point { x: 4, y: 4 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 4, y: 5 },
                end_point: Point { x: 5, y: 5 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 6 },
                end_point: Point { x: 7, y: 6 },
                primary_object_color: Color::Red,
            },
        ]);

        sender_walls
            .send((Walls(walls_in_sight.0[0..3].to_vec()), 0))
            .unwrap();
        sender_walls
            .send((Walls(walls_in_sight.0[3..6].to_vec()), 1))
            .unwrap();

        let expected_generate_polygons = vec![
            Polygon {
                area: [[0.0, 0.1], [1.0, 0.1], [2.0, 0.1], [3.0, 0.1]],
                color: Color::Red,
            },
            Polygon {
                area: [[1.0, 0.3], [2.0, 0.3], [3.0, 0.3], [4.0, 0.3]],
                color: Color::Red,
            },
            Polygon {
                area: [[2.0, 0.5], [3.0, 0.5], [4.0, 0.5], [5.0, 0.5]],
                color: Color::Red,
            },
            Polygon {
                area: [[3.0, 0.6], [4.0, 0.6], [5.0, 0.6], [6.0, 0.6]],
                color: Color::Red,
            },
            Polygon {
                area: [[4.0, 0.4], [5.0, 0.4], [6.0, 0.4], [7.0, 0.4]],
                color: Color::Red,
            },
            Polygon {
                area: [[5.0, 0.2], [6.0, 0.2], [7.0, 0.2], [8.0, 0.2]],
                color: Color::Red,
            },
        ];
        let walls_in_sight_indices: Vec<usize> = vec![0, 1, 5, 4, 3, 2];
        for (index, polygon) in walls_in_sight_indices
            .iter()
            .zip(expected_generate_polygons.iter().cloned())
        {
            let cloned_wall = walls_in_sight.0[*index].clone();
            let cloned_angle = Angle {
                start: Radians::new(0.0),
                end: Radians::new(1.0),
            };
            let cloned_position = graph::Coordinate { x: 2.0, y: 1.0 };
            polygon_generator
                .expect_generate_polygon()
                .times(1)
                .withf(
                    move |wall: &graph::Wall,
                          position: &graph::Coordinate,
                          angle: &player_utils::Angle| {
                        *wall == cloned_wall
                            && *position == cloned_position
                            && *angle == cloned_angle
                    },
                )
                .return_const(polygon)
                .in_sequence(&mut seq);
        }

        let object_generator = ObjectGenerator {
            polygon_generator,
            receiver_walls,
            render_threads_amount,
        };
        assert_eq!(
            object_generator.generate_polygons(&player),
            expected_generate_polygons
        );
    }

    fn generate_polygons(size: usize) -> Vec<Polygon> {
        let mut polygons = vec![];
        let mut temp_val = 0.3;
        for _ in 0..size {
            polygons.push(Polygon {
                area: [
                    [temp_val, temp_val + 0.3],
                    [temp_val + 0.6, temp_val + 0.9],
                    [temp_val + 1.2, temp_val + 1.5],
                    [temp_val + 1.8, temp_val + 2.1],
                ],
                color: Color::Red,
            });
            temp_val += 2.4;
        }
        return polygons;
    }

    fn check_generate_polygons_merge_walls(
        walls_in_sight: graph::Walls,
        merged_walls: graph::Walls,
    ) {
        let mut seq = Sequence::new();

        let render_threads_amount = 3;

        let mut polygon_generator = MockPolygonGenerator::new();
        let player = Arc::new(RwLock::new(MockPlayer::default()));
        let (sender_walls, receiver_walls) = mpsc::channel::<(Walls, usize)>();
        lazy_static! {
            static ref angle: Angle = Angle {
                start: Radians::new(0.0),
                end: Radians::new(1.0),
            };
            static ref position: graph::Coordinate = graph::Coordinate { x: 2.0, y: 1.0 };
        }

        {
            let mut player_write = player.write().unwrap();
            player_write
                .expect_position()
                .times(1)
                .return_const(position.clone())
                .in_sequence(&mut seq);
            player_write
                .expect_angle()
                .times(1)
                .return_const(angle.clone())
                .in_sequence(&mut seq);
        }

        sender_walls
            .send((Walls(vec![walls_in_sight.0[0].clone()]), 0))
            .unwrap();
        sender_walls
            .send((
                Walls(walls_in_sight.0[1..walls_in_sight.0.len() - 1].to_vec()),
                1,
            ))
            .unwrap();
        sender_walls
            .send((
                Walls(vec![walls_in_sight.0[walls_in_sight.0.len() - 1].clone()]),
                2,
            ))
            .unwrap();

        let expected_generate_polygons = generate_polygons(merged_walls.0.len());

        for (wall, polygon) in merged_walls
            .0
            .into_iter()
            .zip(expected_generate_polygons.iter().cloned())
        {
            polygon_generator
                .expect_generate_polygon()
                .times(1)
                .withf(
                    move |wall_: &graph::Wall,
                          position_: &graph::Coordinate,
                          angle_: &player_utils::Angle| {
                        *wall_ == wall && *position_ == *position && *angle_ == *angle
                    },
                )
                .return_const(polygon)
                .in_sequence(&mut seq);
        }

        let object_generator = ObjectGenerator {
            polygon_generator,
            receiver_walls,
            render_threads_amount,
        };
        assert_eq!(
            object_generator.generate_polygons(&player),
            expected_generate_polygons
        );
    }

    #[test]
    fn generate_polygons_merge_walls() {
        let walls_in_sight_1 = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 2, y: 3 },
                end_point: Point { x: 4, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 3, y: 3 },
                end_point: Point { x: 5, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 3 },
                end_point: Point { x: 5, y: 5 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 4 },
                end_point: Point { x: 5, y: 6 },
                primary_object_color: Color::Red,
            },
        ]);
        let merged_walls_1 = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 2, y: 3 },
                end_point: Point { x: 5, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 3 },
                end_point: Point { x: 5, y: 6 },
                primary_object_color: Color::Red,
            },
        ]);

        let walls_in_sight_2 = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 5, y: 6 },
                end_point: Point { x: 5, y: 4 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 5 },
                end_point: Point { x: 5, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 3 },
                end_point: Point { x: 3, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 3, y: 3 },
                end_point: Point { x: 2, y: 3 },
                primary_object_color: Color::Red,
            },
        ]);
        let merged_walls_2 = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 5, y: 6 },
                end_point: Point { x: 5, y: 3 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 3 },
                end_point: Point { x: 2, y: 3 },
                primary_object_color: Color::Red,
            },
        ]);
        let walls_in_sight_3 = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 5, y: 6 },
                end_point: Point { x: 5, y: 5 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 5 },
                end_point: Point { x: 5, y: 4 },
                primary_object_color: Color::Green,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 4 },
                end_point: Point { x: 5, y: 3 },
                primary_object_color: Color::Red,
            },
        ]);
        let merged_walls_3 = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 5, y: 6 },
                end_point: Point { x: 5, y: 5 },
                primary_object_color: Color::Red,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 5 },
                end_point: Point { x: 5, y: 4 },
                primary_object_color: Color::Green,
            },
            graph::Wall {
                start_point: Point { x: 5, y: 4 },
                end_point: Point { x: 5, y: 3 },
                primary_object_color: Color::Red,
            },
        ]);

        check_generate_polygons_merge_walls(walls_in_sight_1, merged_walls_1);
        check_generate_polygons_merge_walls(walls_in_sight_2, merged_walls_2);
        check_generate_polygons_merge_walls(walls_in_sight_3, merged_walls_3);
    }

    #[test]
    fn generate_polygons_no_walls_in_sight() {
        let mut seq = Sequence::new();

        let render_threads_amount = 2;

        let polygon_generator = MockPolygonGenerator::new();
        let player = Arc::new(RwLock::new(MockPlayer::default()));

        {
            let mut player_write = player.write().unwrap();

            player_write
                .expect_position()
                .times(1)
                .return_const(Default::default())
                .in_sequence(&mut seq);
            player_write
                .expect_angle()
                .times(1)
                .return_const(Default::default())
                .in_sequence(&mut seq);
        }

        let (sender_walls, receiver_walls) = mpsc::channel::<(Walls, usize)>();
        sender_walls.send((Walls(vec![]), 0)).unwrap();
        sender_walls.send((Walls(vec![]), 1)).unwrap();

        let object_generator = ObjectGenerator {
            polygon_generator,
            receiver_walls,
            render_threads_amount,
        };
        assert_eq!(
            object_generator.generate_polygons(&player),
            Vec::<Polygon>::new()
        );
    }

    #[test]
    fn generate_polygons_no_render_threads() {
        let render_threads_amount = 0;

        let polygon_generator = MockPolygonGenerator::new();
        let player = Arc::new(RwLock::new(MockPlayer::default()));

        let (_, receiver_walls) = mpsc::channel::<(Walls, usize)>();

        let object_generator = ObjectGenerator {
            polygon_generator,
            receiver_walls,
            render_threads_amount,
        };
        assert_eq!(
            object_generator.generate_polygons(&player),
            Vec::<Polygon>::new()
        );
    }
}
