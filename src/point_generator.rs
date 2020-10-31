use crate::graph;
use crate::player_utils;
use piston_window::Size;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub struct PointGenerator {
    pub resolution: Size, // Size currently contains u32 because of piston_window 0.83.0 instead of f64 in 0.113.0
    pub half_vertical_angle_value: player_utils::Radians,
    pub wall_height: f64,
}

#[cfg_attr(test, automock)]
impl PointGenerator {
    pub fn point_width(
        &self,
        angle: &player_utils::Angle,
        start_position: &graph::Coordinate,
        end_position: &graph::Coordinate,
    ) -> f64 {
        let point_radians = start_position.into_radians(end_position);
        if angle.is_inside(point_radians) {
            return self.point_width_inside_field_of_view(angle, &point_radians);
        }
        return self.point_width_outside_field_of_view(angle, &point_radians);
    }

    fn point_width_inside_field_of_view(
        &self,
        angle: &player_utils::Angle,
        point_radians: &player_utils::Radians,
    ) -> f64 {
        return (point_radians - angle.start) / angle.value() * self.resolution.width as f64;
    }

    fn point_width_outside_field_of_view(
        &self,
        angle: &player_utils::Angle,
        point_radians: &player_utils::Radians,
    ) -> f64 {
        let radians_to_angle_start = (angle.start - point_radians).min(point_radians - angle.start);
        let radians_to_angle_end = (angle.end - point_radians).min(point_radians - angle.end);
        if radians_to_angle_start < radians_to_angle_end {
            return -(radians_to_angle_start / angle.value() * self.resolution.width as f64);
        }
        return (radians_to_angle_end + angle.value()) / angle.value()
            * self.resolution.width as f64;
    }

    // returns 1/2 of point height
    pub fn point_height(
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] //angle.start < angle.end
    fn point_width_1() {
        let resolution_width = 800;
        let polygon_generator = PointGenerator {
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
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: -5.0 },
            ),
            -(resolution_width as f64 / 4.0)
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: 0.0 },
            ),
            0.0
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 0.0, y: 5.0 },
            ),
            resolution_width as f64 / 2.0
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: -5.0, y: 0.0 },
            ),
            resolution_width as f64
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: -5.0, y: -5.0 },
            ),
            resolution_width as f64 + resolution_width as f64 / 4.0
        );
    }

    #[test] //angle.start > angle.end
    fn point_width_2() {
        let resolution_width = 800;

        let polygon_generator = PointGenerator {
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
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: -5.0, y: -5.0 },
            ),
            -(resolution_width as f64 / 4.0)
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 0.0, y: -5.0 },
            ),
            0.0
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: -5.0 },
            ),
            resolution_width as f64 / 4.0
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: 0.0 },
            ),
            resolution_width as f64 / 2.0
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 5.0, y: 5.0 },
            ),
            resolution_width as f64 * 3.0 / 4.0
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: 0.0, y: 5.0 },
            ),
            resolution_width as f64
        );
        assert_eq!(
            polygon_generator.point_width(
                &angle,
                &start_position,
                &graph::Coordinate { x: -5.0, y: 5.0 },
            ),
            resolution_width as f64 + resolution_width as f64 / 4.0
        );
    }

    #[test]
    fn point_height() {
        let resolution_height = 600;

        let polygon_generator = PointGenerator {
            resolution: Size {
                width: 0,
                height: resolution_height,
            },
            half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 2.0),
            wall_height: 5.0,
        };

        assert_eq!(
            polygon_generator.point_height(
                &graph::Coordinate { x: 0.0, y: 0.0 },
                &graph::Coordinate { x: 5.0, y: 0.0 },
            ),
            resolution_height as f64 / 4.0
        );

        let polygon_generator = PointGenerator {
            resolution: Size {
                width: 0,
                height: resolution_height,
            },
            half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 3.0),
            wall_height: 5.0,
        };

        assert_eq!(
            polygon_generator.point_height(
                &graph::Coordinate { x: 0.0, y: 0.0 },
                &graph::Coordinate { x: 5.0, y: 0.0 },
            ),
            resolution_height as f64 * 3.0 / 8.0
        );
    }
}
