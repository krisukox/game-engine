use crate::graph;
use crate::map_element::MapElement;
use crate::map_element::Point;
use crate::player_utils;
use crate::polygon_generator;
use graphics::types::Vec2d;

#[cfg(test)]
use mockall::automock;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use self::polygon_generator::MockPolygonGenerator as PolygonGenerator;
        use crate::player_utils::MockPlayer as Player;
        use crate::map::MockMap as Map;
        use crate::graph::MockLinearGraph as LinearGraph;
    } else {
        use self::polygon_generator::PolygonGenerator;
        use crate::player_utils::Player;
        use crate::map::Map;
        use crate::graph::LinearGraph;
    }
}

pub struct ObjectGenerator {
    pub rays: Vec<LinearGraph>,
    pub polygon_generator: PolygonGenerator,
    pub map: Map,
}

fn handle_one_point(last_points: &mut Vec<Point>, walls: &mut graph::Walls, new_point: Point) {
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
    last_points: &mut Vec<Point>,
    walls: &mut graph::Walls,
    new_points: &mut Vec<Point>,
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
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> graph::Walls {
        let mut walls_in_sight = graph::Walls(vec![]);
        let mut last_points: Vec<Point> = Vec::with_capacity(2);
        for rays_indexes in rays_indexes_vec {
            for index in rays_indexes {
                let mut points = self.map.cast_ray(position, &self.rays[index], map_elements);
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

    pub fn generate_polygons(
        &self,
        player: &Player,
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Vec<[Vec2d; 4]> {
        return self.generate_polygons_(
            self.get_walls_in_sight(
                player.position(),
                player.get_rays_angle_range(),
                map_elements,
            ),
            player.position(),
            player.angle(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MockLinearGraph;
    use crate::map::MockMap;
    use crate::map_element::MockMapElement;
    use crate::player_utils::Angle;
    use crate::player_utils::MockPlayer;
    use crate::player_utils::Radians;
    use crate::wall_map::WallMap;
    use mockall::*;
    use polygon_generator::MockPolygonGenerator;

    // fn test_generate_polygons(
    //     walls_in_sight: graph::Walls,
    //     generate_polygons: Vec<[Vec2d; 4]>,
    //     walls_in_sight_indices: Vec<usize>,
    //     position: graph::Coordinate,
    // ) {
    //     let mut polygon_generator = MockPolygonGenerator::new();
    //     let mut map = MockMap::new();
    //     let mut player = MockPlayer::default();
    //     let mut seq = Sequence::new();

    //     let mut map_element = Box::new(MockMapElement::new());
    //     let mut map_elements = vec![map_element];

    //     let angle = player_utils::Angle {
    //         start: player_utils::Radians::new(0.0),
    //         end: player_utils::Radians::new(std::f64::consts::PI / 2.0),
    //     };

    //     for (index, polygon) in walls_in_sight_indices
    //         .iter()
    //         .zip(generate_polygons.iter().cloned())
    //     {
    //         let cloned_wall = walls_in_sight.0[*index].clone();
    //         let cloned_angle = angle.clone();
    //         let cloned_position = position.clone();
    //         polygon_generator
    //             .expect_generate_polygon()
    //             .times(1)
    //             .withf(
    //                 move |wall: &graph::Wall,
    //                       position: &graph::Coordinate,
    //                       angle: &player_utils::Angle| {
    //                     *wall == cloned_wall
    //                         && *position == cloned_position
    //                         && *angle == cloned_angle
    //                 },
    //             )
    //             .return_const(polygon)
    //             .in_sequence(&mut seq);
    //     }

    //     let object_generator = ObjectGenerator {
    //         map,
    //         rays: Default::default(),
    //         polygon_generator,
    //     };
    //     assert_eq!(
    //         object_generator.generate_polygons(&player, &map_elements),
    //         generate_polygons
    //     );
    // }

    #[test]
    fn generate_polygons() {
        let mut seq = Sequence::new();

        let mut map = MockMap::new();
        let rays = vec![
            MockLinearGraph::new(),
            MockLinearGraph::new(),
            MockLinearGraph::new(),
            MockLinearGraph::new(),
            MockLinearGraph::new(),
            MockLinearGraph::new(),
            MockLinearGraph::new(),
            MockLinearGraph::new(),
            MockLinearGraph::new(),
        ];
        let mut polygon_generator = MockPolygonGenerator::new();

        let mut player = MockPlayer::default();
        let map_elements: Vec<Box<dyn MapElement>> = vec![Box::new(MockMapElement::new())];

        let player_position = graph::Coordinate { x: 20.0, y: 10.0 };
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

        player
            .expect_position()
            .times(1)
            .return_const(player_position.clone())
            .in_sequence(&mut seq);
        player
            .expect_get_rays_angle_range()
            .times(1)
            .return_const(vec![std::ops::Range { start: 1, end: 9 }])
            .in_sequence(&mut seq);

        for points in vec_points {
            let player_position_clone = player_position.clone();
            map.expect_cast_ray()
                .times(1)
                .withf(move |position, _, _| *position == player_position_clone)
                .return_const(points)
                .in_sequence(&mut seq);
        }
        player
            .expect_position()
            .times(1)
            .return_const(graph::Coordinate { x: 2.0, y: 1.0 })
            .in_sequence(&mut seq);
        player
            .expect_angle()
            .times(1)
            .return_const(Angle {
                start: Radians::new(0.0),
                end: Radians::new(1.0),
            })
            .in_sequence(&mut seq);

        let walls_in_sight = graph::Walls(vec![
            graph::Wall {
                start_point: Point { x: 1, y: 4 },
                end_point: Point { x: 2, y: 4 },
            },
            graph::Wall {
                start_point: Point { x: 2, y: 4 },
                end_point: Point { x: 2, y: 3 },
            },
            graph::Wall {
                start_point: Point { x: 2, y: 3 },
                end_point: Point { x: 3, y: 3 },
            },
            graph::Wall {
                start_point: Point { x: 3, y: 4 },
                end_point: Point { x: 4, y: 4 },
            },
            graph::Wall {
                start_point: Point { x: 4, y: 5 },
                end_point: Point { x: 5, y: 5 },
            },
            graph::Wall {
                start_point: Point { x: 5, y: 6 },
                end_point: Point { x: 7, y: 6 },
            },
        ]);
        let expected_generate_polygons = vec![
            [[0.0, 0.1], [1.0, 0.1], [2.0, 0.1], [3.0, 0.1]],
            [[1.0, 0.3], [2.0, 0.3], [3.0, 0.3], [4.0, 0.3]],
            [[2.0, 0.5], [3.0, 0.5], [4.0, 0.5], [5.0, 0.5]],
            [[3.0, 0.6], [4.0, 0.6], [5.0, 0.6], [6.0, 0.6]],
            [[4.0, 0.4], [5.0, 0.4], [6.0, 0.4], [7.0, 0.4]],
            [[5.0, 0.2], [6.0, 0.2], [7.0, 0.2], [8.0, 0.2]],
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
            map,
            rays,
            polygon_generator,
        };
        assert_eq!(
            object_generator.generate_polygons(&player, &map_elements),
            expected_generate_polygons
        );
    }

    #[test]
    fn generate_polygons_no_walls_in_sight() {
        let mut seq = Sequence::new();

        let mut map = MockMap::new();
        let rays = vec![MockLinearGraph::new()];
        let polygon_generator = MockPolygonGenerator::new();

        let mut player = MockPlayer::default();
        let map_elements: Vec<Box<dyn MapElement>> = vec![Box::new(MockMapElement::new())];

        let player_position = graph::Coordinate { x: 20.0, y: 10.0 };

        player
            .expect_position()
            .times(1)
            .return_const(player_position.clone())
            .in_sequence(&mut seq);
        player
            .expect_get_rays_angle_range()
            .times(1)
            .return_const(vec![std::ops::Range { start: 0, end: 1 }])
            .in_sequence(&mut seq);

        let player_position_clone = player_position.clone();
        map.expect_cast_ray()
            .times(1)
            .withf(move |position, _, _| *position == player_position_clone)
            .return_const(Vec::<Point>::new())
            .in_sequence(&mut seq);

        player
            .expect_position()
            .times(1)
            .return_const(graph::Coordinate { x: 2.0, y: 1.0 })
            .in_sequence(&mut seq);
        player
            .expect_angle()
            .times(1)
            .return_const(Angle {
                start: Radians::new(0.0),
                end: Radians::new(1.0),
            })
            .in_sequence(&mut seq);

        let object_generator = ObjectGenerator {
            map,
            rays,
            polygon_generator,
        };
        assert_eq!(
            object_generator.generate_polygons(&player, &map_elements),
            Vec::<[Vec2d; 4]>::new()
        );
    }
}
