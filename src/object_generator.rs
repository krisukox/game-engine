use crate::graph;
use crate::map::Map;
use crate::player_utils;
use crate::polygon_generator;
use graphics::types::Vec2d;
#[cfg(test)]
use mockall::automock;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use self::polygon_generator::MockPolygonGenerator as PolygonGenerator;
        use crate::player_utils::MockPlayer as Player;
    } else {
        use self::polygon_generator::PolygonGenerator;
        use crate::player_utils::Player as Player;
    }
}

pub struct ObjectGenerator {
    pub map: Map,
    pub rays: Vec<graph::LinearGraph>,
    pub polygon_generator: PolygonGenerator,
}

fn handle_one_point(
    last_points: &mut Vec<graph::Coordinate>,
    walls: &mut graph::Walls,
    new_point: graph::Coordinate,
) {
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

fn handle_points(
    last_points: &mut Vec<graph::Coordinate>,
    walls: &mut graph::Walls,
    new_points: &mut Vec<graph::Coordinate>,
) {
    if let Some(point) = walls.try_extend_last_wall(new_points) {
        handle_one_point(last_points, walls, point);
    }
}

#[cfg_attr(test, automock)]
impl ObjectGenerator {
    fn get_walls_in_sight(
        &self,
        position: &graph::Coordinate,
        rays_indexes_vec: std::vec::Vec<std::ops::Range<usize>>,
    ) -> graph::Walls {
        let mut walls_in_sight = graph::Walls(vec![]);
        let mut last_points: Vec<graph::Coordinate> = Vec::with_capacity(2);
        for rays_indexes in rays_indexes_vec {
            for index in rays_indexes {
                let mut points = self.map.cast_ray(position, &self.rays[index]);
                handle_points(&mut last_points, &mut walls_in_sight, &mut points);
            }
        }
        return walls_in_sight;
    }

    fn generate_farther_polygons(
        &self,
        index: usize,
        walls_in_sight: &Vec<graph::Wall>,
        position: &graph::Coordinate,
        angle: &player_utils::Angle,
        polygons: &mut Vec<[Vec2d; 4]>,
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
    ) -> Vec<[Vec2d; 4]> {
        let mut polygons: Vec<[Vec2d; 4]> = Vec::new();
        let mut index = 0;
        while index < walls_in_sight.0.len() - 1 {
            if walls_in_sight.0[index].point_distance_end(&position)
                > walls_in_sight.0[index + 1].point_distance_start(&position)
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

    pub fn generate_polygons(&self, player: &Player) -> Vec<[Vec2d; 4]> {
        #[cfg(not(test))]
        return self.generate_polygons_(
            self.get_walls_in_sight(&player.position, player.get_rays_angle_range()),
            &player.position,
            &player.angle,
        );
        #[cfg(test)]
        return vec![];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::player_utils::Radians;
    use mockall::*;

    fn test_generate_polygons(
        walls_in_sight: graph::Walls,
        generate_polygons: Vec<[Vec2d; 4]>,
        walls_in_sight_indices: Vec<usize>,
        position: graph::Coordinate,
    ) {
        let mut object_generator = polygon_generator::MockPolygonGenerator::new();
        let mut seq = Sequence::new();

        let angle = player_utils::Angle {
            start: player_utils::Radians::new(0.0),
            end: player_utils::Radians::new(std::f64::consts::PI / 2.0),
        };

        for (index, polygon) in walls_in_sight_indices
            .iter()
            .zip(generate_polygons.iter().cloned())
        {
            let cloned_wall = walls_in_sight.0[*index].clone();
            let cloned_angle = angle.clone();
            let cloned_position = position.clone();
            object_generator
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
            map: Default::default(),
            rays: Default::default(),
            polygon_generator: object_generator,
        };
        assert_eq!(
            object_generator.generate_polygons_(walls_in_sight, &position, &angle),
            generate_polygons
        );
    }

    #[test]
    fn generate_polygons_1() {
        let walls_in_sight = graph::Walls(vec![
            graph::Wall {
                start_point: graph::Coordinate { x: 1.0, y: 4.0 },
                end_point: graph::Coordinate { x: 2.0, y: 4.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 2.0, y: 4.0 },
                end_point: graph::Coordinate { x: 2.0, y: 3.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 2.0, y: 3.0 },
                end_point: graph::Coordinate { x: 3.0, y: 3.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 3.0, y: 4.0 },
                end_point: graph::Coordinate { x: 4.0, y: 4.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 4.0, y: 5.0 },
                end_point: graph::Coordinate { x: 5.0, y: 5.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 5.0, y: 6.0 },
                end_point: graph::Coordinate { x: 6.0, y: 6.0 },
            },
        ]);
        let generate_polygons = vec![
            [[0.0, 0.1], [1.0, 0.1], [2.0, 0.1], [3.0, 0.1]],
            [[1.0, 0.3], [2.0, 0.3], [3.0, 0.3], [4.0, 0.3]],
            [[2.0, 0.5], [3.0, 0.5], [4.0, 0.5], [5.0, 0.5]],
            [[3.0, 0.6], [4.0, 0.6], [5.0, 0.6], [6.0, 0.6]],
            [[4.0, 0.4], [5.0, 0.4], [6.0, 0.4], [7.0, 0.4]],
            [[5.0, 0.2], [6.0, 0.2], [7.0, 0.2], [8.0, 0.2]],
        ];
        let walls_in_sight_indices: Vec<usize> = vec![0, 1, 5, 4, 3, 2];
        let position = graph::Coordinate { x: 2.0, y: 1.0 };

        test_generate_polygons(
            walls_in_sight,
            generate_polygons,
            walls_in_sight_indices,
            position,
        );
    }

    #[test]
    fn generate_polygons_2() {
        let walls_in_sight = graph::Walls(vec![
            graph::Wall {
                start_point: graph::Coordinate { x: 1.0, y: 7.0 },
                end_point: graph::Coordinate { x: 2.0, y: 7.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 2.0, y: 6.0 },
                end_point: graph::Coordinate { x: 3.0, y: 6.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 3.0, y: 5.0 },
                end_point: graph::Coordinate { x: 4.0, y: 5.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 4.0, y: 4.0 },
                end_point: graph::Coordinate { x: 5.0, y: 4.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 5.0, y: 4.0 },
                end_point: graph::Coordinate { x: 5.0, y: 5.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 5.0, y: 5.0 },
                end_point: graph::Coordinate { x: 6.0, y: 5.0 },
            },
        ]);
        let generate_polygons = vec![
            [[0.0, 0.1], [1.0, 0.1], [2.0, 0.1], [3.0, 0.1]],
            [[1.0, 0.3], [2.0, 0.3], [3.0, 0.3], [4.0, 0.3]],
            [[2.0, 0.5], [3.0, 0.5], [4.0, 0.5], [5.0, 0.5]],
            [[3.0, 0.6], [4.0, 0.6], [5.0, 0.6], [6.0, 0.6]],
            [[4.0, 0.4], [5.0, 0.4], [6.0, 0.4], [7.0, 0.4]],
            [[5.0, 0.2], [6.0, 0.2], [7.0, 0.2], [8.0, 0.2]],
        ];
        let walls_in_sight_indices: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let position = graph::Coordinate { x: 5.0, y: 2.0 };

        test_generate_polygons(
            walls_in_sight,
            generate_polygons,
            walls_in_sight_indices,
            position,
        );
    }

    #[test]
    fn get_walls_in_sight() {
        let expected_walls_in_sight = graph::Walls(vec![
            graph::Wall {
                start_point: graph::Coordinate { x: 34.0, y: 26.0 },
                end_point: graph::Coordinate { x: 34.0, y: 28.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 34.0, y: 28.0 },
                end_point: graph::Coordinate { x: 32.0, y: 28.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 32.0, y: 28.0 },
                end_point: graph::Coordinate { x: 32.0, y: 30.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 32.0, y: 30.0 },
                end_point: graph::Coordinate { x: 30.0, y: 30.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 30.0, y: 30.0 },
                end_point: graph::Coordinate { x: 30.0, y: 32.0 },
            },
            graph::Wall {
                start_point: graph::Coordinate { x: 30.0, y: 32.0 },
                end_point: graph::Coordinate { x: 27.0, y: 32.0 },
            },
        ]);

        let position = graph::Coordinate { x: 27.0, y: 26.0 };

        let mut rays: Vec<graph::LinearGraph> = Vec::new();
        let mut radians = 0.0;
        while radians < std::f64::consts::PI * 2.0 {
            rays.push(graph::LinearGraph::from_radians(Radians::new(radians)));
            radians += 0.05;
        }
        let rays_indexes = 0..rays.len() / 4 + 1;
        if let Ok(map) = Map::new("test_resources/map.png") {
            let object_generator = ObjectGenerator {
                map,
                rays,
                polygon_generator: polygon_generator::MockPolygonGenerator::new(),
            };
            assert_eq!(
                object_generator.get_walls_in_sight(&position, vec![rays_indexes]),
                expected_walls_in_sight
            );
        }
    }

    #[test]
    fn handle_one_point_test() {
        let mut last_points = Vec::new();
        let mut walls = Default::default();
        let last_point_1 = graph::Coordinate { x: 1.0, y: 3.0 };
        let last_point_2 = graph::Coordinate { x: 2.0, y: 3.0 };

        handle_one_point(&mut last_points, &mut walls, last_point_1.clone());
        assert_eq!(last_points, vec![last_point_1]);

        handle_one_point(&mut last_points, &mut walls, last_point_2);
        assert!(last_points.is_empty());

        handle_one_point(&mut last_points, &mut walls, Default::default());
        last_points = vec![
            graph::Coordinate { x: 5.0, y: 7.0 },
            graph::Coordinate { x: 9.0, y: 11.0 },
        ];

        handle_one_point(&mut last_points, &mut walls, Default::default());
        assert!(last_points.is_empty());
    }
}
