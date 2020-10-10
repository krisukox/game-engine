use crate::graph;
use crate::map::Map;
use crate::player_utils;
use piston_window::types::Vec2d;
use piston_window::Size;

pub struct ObjectGenerator {
    map: Map,
    rays: Vec<graph::LinearGraph>,
    resolution: Size, // Size currently contains u32 because of piston_window 0.83.0 instead of f64 in 0.113.0
    half_vertical_angle_value: player_utils::Radians,
    wall_height: f64,
}

impl ObjectGenerator {
    fn get_points_in_sight(
        &self,
        position: &graph::Coordinate,
        rays_indexes: std::ops::Range<usize>,
    ) -> Vec<graph::Coordinate> {
        let mut points_in_sight: Vec<graph::Coordinate> = Vec::new();
        for index in rays_indexes {
            let points = self.map.cast_ray(position, &self.rays[index]);
            for point in points {
                if !points_in_sight.contains(&point) {
                    points_in_sight.push(point);
                }
            }
        }
        return points_in_sight;
    }

    fn point_width_in_field_of_view(
        &self,
        angle: &player_utils::Angle,
        start_position: &graph::Coordinate,
        end_position: &graph::Coordinate,
    ) -> f64 {
        let point_radians = start_position.into_radians(end_position);
        return (point_radians - angle.start) / angle.value() * self.resolution.width as f64;
    }

    // returns 1/2 of point height
    fn point_height_in_field_of_view(
        &self,
        start_position: &graph::Coordinate,
        end_position: &graph::Coordinate,
    ) -> f64 {
        let point_radians = graph::ZERO_COORDINATE.into_radians(&graph::Coordinate {
            x: start_position.distance(&end_position),
            y: self.wall_height,
        });
        return point_radians / self.half_vertical_angle_value * self.resolution.height as f64
            / 2.0;
    }

    pub fn generate_polygons(
        &self,
        position: graph::Coordinate,
        rays_indexes: std::ops::Range<usize>,
        angle: player_utils::Angle,
    ) -> Vec<[Vec2d; 4]> {
        let mut last_point_width = 0.0;
        let mut polygons: Vec<[Vec2d; 4]> = Vec::new();
        let points_in_sight = self.get_points_in_sight(&position, rays_indexes);
        // let points_in_sight_iter = points_in_sight.iter();
        for end_position in points_in_sight {
            let point_width = self.point_width_in_field_of_view(&angle, &position, &end_position);
            let point_height = self.point_height_in_field_of_view(&position, &end_position);
            polygons.push([
                [last_point_width, point_height],
                [point_width, point_height],
                [point_width, -point_height],
                [last_point_width, -point_height],
            ]);
            last_point_width = point_width;
        }
        return polygons;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_points_in_sight() {
        let expected_points_in_sight = vec![
            graph::Coordinate { x: 34.0, y: 26.0 },
            graph::Coordinate { x: 34.0, y: 27.0 },
            graph::Coordinate { x: 34.0, y: 28.0 },
            graph::Coordinate { x: 33.0, y: 28.0 },
            graph::Coordinate { x: 32.0, y: 28.0 },
            graph::Coordinate { x: 32.0, y: 29.0 },
            graph::Coordinate { x: 32.0, y: 30.0 },
            graph::Coordinate { x: 31.0, y: 30.0 },
            graph::Coordinate { x: 30.0, y: 30.0 },
            graph::Coordinate { x: 30.0, y: 31.0 },
            graph::Coordinate { x: 30.0, y: 32.0 },
            graph::Coordinate { x: 29.0, y: 32.0 },
            graph::Coordinate { x: 28.0, y: 32.0 },
            graph::Coordinate { x: 27.0, y: 32.0 },
        ];

        let position = graph::Coordinate { x: 27.0, y: 26.0 };

        let mut rays: Vec<graph::LinearGraph> = Vec::new();
        let mut radians = 0.0;
        while radians < std::f64::consts::PI * 2.0 {
            rays.push(graph::LinearGraph::from_radians(radians));
            radians += 0.1;
        }
        let rays_indexes = 0..rays.len() / 4 + 1;
        if let Ok(map) = Map::new("test_resources/map.png") {
            let object_generator = ObjectGenerator {
                map,
                rays,
                resolution: Size {
                    width: 0,
                    height: 0,
                },
                half_vertical_angle_value: Default::default(),
                wall_height: Default::default(),
            };
            let points_in_sight = object_generator.get_points_in_sight(&position, rays_indexes);
            assert_eq!(expected_points_in_sight.len(), points_in_sight.len());
            for expected_point in expected_points_in_sight {
                assert!(points_in_sight.contains(&expected_point));
            }
        }
    }

    #[test] //angle.start < angle.end
    fn point_width_in_field_of_view_1() {
        let resolution_width = 800;

        let object_generator = ObjectGenerator {
            map: Map::dummy(),
            rays: Default::default(),
            resolution: Size {
                width: resolution_width,
                height: Default::default(),
            },
            half_vertical_angle_value: Default::default(),
            wall_height: Default::default(),
        };
        let angle = player_utils::Angle {
            start: player_utils::Radians(0.0),
            end: player_utils::Radians(std::f64::consts::PI),
        };
        let start_position = graph::Coordinate { x: 0.0, y: 0.0 };
        assert_eq!(
            object_generator.point_width_in_field_of_view(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: 5.0 },
            ),
            resolution_width as f64 / 4.0
        );
        assert_eq!(
            object_generator.point_width_in_field_of_view(
                &angle,
                &start_position,
                &graph::Coordinate { x: 0.0, y: 5.0 },
            ),
            resolution_width as f64 / 2.0
        );
        assert_eq!(
            object_generator.point_width_in_field_of_view(
                &angle,
                &start_position,
                &graph::Coordinate { x: -5.0, y: 5.0 },
            ),
            resolution_width as f64 * 3.0 / 4.0
        );
    }

    #[test] //angle.start > angle.end
    fn point_width_in_field_of_view_2() {
        let resolution_width = 800;

        let object_generator = ObjectGenerator {
            map: Map::dummy(),
            rays: Default::default(),
            resolution: Size {
                width: resolution_width,
                height: 0,
            },
            half_vertical_angle_value: Default::default(),
            wall_height: Default::default(),
        };
        let angle = player_utils::Angle {
            start: player_utils::Radians(std::f64::consts::PI * 3.0 / 2.0),
            end: player_utils::Radians(std::f64::consts::PI / 2.0),
        };
        let start_position = graph::Coordinate { x: 0.0, y: 0.0 };
        assert_eq!(
            object_generator.point_width_in_field_of_view(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: -5.0 },
            ),
            resolution_width as f64 / 4.0
        );
        assert_eq!(
            object_generator.point_width_in_field_of_view(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: 0.0 },
            ),
            resolution_width as f64 / 2.0
        );
        assert_eq!(
            object_generator.point_width_in_field_of_view(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: 5.0 },
            ),
            resolution_width as f64 * 3.0 / 4.0
        );
    }

    #[test]
    fn point_height_in_field_of_view() {
        let resolution_height = 600;

        let object_generator = ObjectGenerator {
            map: Map::dummy(),
            rays: Default::default(),
            resolution: Size {
                width: 0,
                height: resolution_height,
            },
            half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 2.0),
            wall_height: 5.0,
        };
        assert_eq!(
            object_generator.point_height_in_field_of_view(
                &graph::Coordinate { x: 0.0, y: 0.0 },
                &graph::Coordinate { x: 5.0, y: 0.0 },
            ),
            resolution_height as f64 / 4.0
        );

        let object_generator = ObjectGenerator {
            map: Map::dummy(),
            rays: Default::default(),
            resolution: Size {
                width: 0,
                height: resolution_height,
            },
            half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 3.0),
            wall_height: 5.0,
        };
        assert_eq!(
            object_generator.point_height_in_field_of_view(
                &graph::Coordinate { x: 0.0, y: 0.0 },
                &graph::Coordinate { x: 5.0, y: 0.0 },
            ),
            resolution_height as f64 * 3.0 / 8.0
        );
    }
}
