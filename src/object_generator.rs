use crate::graph;
use crate::map::Map;
use crate::player_utils;
use piston_window::Size;

pub struct ObjectGenerator {
    map: Map,
    rays: Vec<graph::LinearGraph>,
    resolution: Size, // Size currently contains u32 because of piston_window 0.83.0 instead of 0.113.0
}

impl ObjectGenerator {
    pub fn get_points_in_sight(
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
        if point_radians < angle.start {
            return (point_radians + std::f64::consts::PI * 2.0 - angle.start) / angle.value()
                * self.resolution.width as f64;
        }
        return (point_radians - angle.start) / angle.value() * self.resolution.width as f64;
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
        let dummy_map = Map::dummy();
        let dummy_rays: Vec<graph::LinearGraph> = Vec::new();
        let resolution_width = 800;

        let object_generator = ObjectGenerator {
            map: dummy_map,
            rays: dummy_rays,
            resolution: Size {
                width: resolution_width,
                height: 0,
            },
        };
        let angle = player_utils::Angle::new(0.0, std::f64::consts::PI, 4);
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
        let dummy_map = Map::dummy();
        let dummy_rays: Vec<graph::LinearGraph> = Vec::new();
        let resolution_width = 800;

        let object_generator = ObjectGenerator {
            map: dummy_map,
            rays: dummy_rays,
            resolution: Size {
                width: resolution_width,
                height: 0,
            },
        };
        let angle = player_utils::Angle::new(
            std::f64::consts::PI * 3.0 / 2.0,
            std::f64::consts::PI / 2.0,
            4,
        );
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
}
