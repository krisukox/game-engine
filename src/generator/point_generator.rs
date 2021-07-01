use crate::graph::{Coordinate, LinearGraph};
use crate::map_element::Point;
use crate::player_utils;
use piston::window::Size;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub struct PointGenerator {
    resolution: Size,
    vertical_tangens: f64,
    half_wall_height: f64,
}

#[cfg_attr(test, automock)]
impl PointGenerator {
    pub fn new(
        resolution: Size,
        vertical_angle_value: player_utils::Radians,
        wall_height: f64,
    ) -> PointGenerator {
        PointGenerator {
            resolution,
            vertical_tangens: LinearGraph::from_radians(vertical_angle_value / 2.0).tangens,
            half_wall_height: wall_height / 2.0,
        }
    }

    pub fn point_width(
        &self,
        angle: &player_utils::Angle,
        start_position: &Coordinate,
        end_position: &Point,
    ) -> f64 {
        let point_radians = start_position.into_radians(end_position);
        if angle.is_inside(point_radians) {
            return self.point_width_inside_field_of_view(angle, &point_radians);
        }
        return self.point_width_outside_field_of_view(angle, &point_radians);
    }

    fn compute_graphs(
        angle: &player_utils::Angle,
        point_radians: &player_utils::Radians,
    ) -> (LinearGraph, LinearGraph, f64, Coordinate) {
        let direction = LinearGraph::from_radians(angle.get_direction() - angle.start);

        let perpendicular_direction = LinearGraph::from_radians(
            angle.get_direction() + player_utils::Radians::new(std::f64::consts::PI / 2.0)
                - angle.start,
        );

        let cross_point_middle_x = (1.0 / (direction.tangens.powi(2) + 1.0)).sqrt();
        let cross_point_middle_y = direction.tangens * cross_point_middle_x;

        let perpendicular_direction_b =
            cross_point_middle_y - perpendicular_direction.tangens * cross_point_middle_x;

        let graph_point_radians = LinearGraph::from_radians(point_radians - angle.start);
        return (
            graph_point_radians,
            perpendicular_direction,
            perpendicular_direction_b,
            Coordinate {
                x: cross_point_middle_x,
                y: cross_point_middle_y,
            },
        );
    }

    fn compute_distances_for_width(
        angle: &player_utils::Angle,
        point_radians: &player_utils::Radians,
    ) -> (f64, f64) {
        let (
            graph_point_radians,
            perpendicular_direction,
            perpendicular_direction_b,
            cross_point_middle,
        ) = Self::compute_graphs(angle, point_radians);

        let cross_point_y_0 = Coordinate {
            x: -perpendicular_direction_b / perpendicular_direction.tangens,
            y: 0.0,
        };

        let whole_distance = cross_point_y_0.distance(&cross_point_middle) * 2.0;

        let cross_point_x = -perpendicular_direction_b
            / (perpendicular_direction.tangens - graph_point_radians.tangens);
        let cross_point_y = cross_point_x * graph_point_radians.tangens;

        let short_distance = cross_point_y_0.distance(&Coordinate {
            x: cross_point_x,
            y: cross_point_y,
        });
        return (short_distance, whole_distance);
    }

    fn point_width_inside_field_of_view(
        &self,
        angle: &player_utils::Angle,
        point_radians: &player_utils::Radians,
    ) -> f64 {
        let (short_distance, whole_distance) =
            Self::compute_distances_for_width(angle, point_radians);

        return short_distance / whole_distance * self.resolution.width as f64;
    }

    fn point_width_outside_field_of_view(
        &self,
        angle: &player_utils::Angle,
        point_radians: &player_utils::Radians,
    ) -> f64 {
        let (short_distance, whole_distance) =
            Self::compute_distances_for_width(angle, point_radians);

        if short_distance < whole_distance {
            return -short_distance / whole_distance * self.resolution.width as f64;
        }
        return short_distance / whole_distance * self.resolution.width as f64;
    }

    // returns 1/2 of point height
    pub fn point_height(
        &self,
        angle: &player_utils::Angle,
        start_position: &Coordinate,
        end_position: &Point,
    ) -> f64 {
        let point_radians = start_position.into_radians(end_position);
        let (graph_point_radians, perpendicular_direction, perpendicular_direction_b, _) =
            Self::compute_graphs(angle, &point_radians);

        let cross_point_x = -perpendicular_direction_b
            / (perpendicular_direction.tangens - graph_point_radians.tangens);
        let cross_point_y = cross_point_x * graph_point_radians.tangens;

        let short_distance = Coordinate::ZERO.distance(&Coordinate {
            x: cross_point_x,
            y: cross_point_y,
        });
        let whole_distance = end_position.distance_coor(&start_position);
        return (short_distance / whole_distance * self.half_wall_height) / self.vertical_tangens
            * self.resolution.height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn point_width_inside_field_of_view() {
        let resolution_width = 800.0;
        let point_generator = PointGenerator::new(
            Size {
                width: resolution_width,
                height: Default::default(),
            },
            Default::default(),
            Default::default(),
        );

        let angle = player_utils::Angle {
            start: player_utils::Radians::new(0.0),
            end: player_utils::Radians::new(std::f64::consts::PI * 2.0 / 3.0),
        };
        let start_position = Coordinate { x: 0.0, y: 0.0 };
        let end_position = Point { x: 0, y: 10 };

        let short_distance = 3.0_f64.sqrt() * 4.0 / 3.0;
        let whole_distance = 3.0_f64.sqrt() * 2.0;

        assert!(approx_eq!(
            f64,
            point_generator.point_width(&angle, &start_position, &end_position),
            short_distance / whole_distance * resolution_width,
            ulps = 3
        ));
    }

    #[test]
    fn point_width_outside_field_of_view() {
        let resolution_width = 800.0;
        let point_generator = PointGenerator::new(
            Size {
                width: resolution_width,
                height: Default::default(),
            },
            Default::default(),
            Default::default(),
        );

        let angle = player_utils::Angle {
            start: player_utils::Radians::new(std::f64::consts::PI / 4.0),
            end: player_utils::Radians::new(std::f64::consts::PI * 3.0 / 4.0),
        };
        let start_position = Coordinate { x: 8.0, y: 1.0 };
        let end_position_1 = Point { x: 14, y: 4 };
        let end_position_2 = Point { x: 2, y: 4 };

        let short_distance_1 = -1.0;
        let short_distance_2 = 3.0;
        let whole_distance = 2.0;

        assert!(approx_eq!(
            f64,
            point_generator.point_width(&angle, &start_position, &end_position_1),
            short_distance_1 / whole_distance * resolution_width,
            ulps = 3
        ));

        assert!(approx_eq!(
            f64,
            point_generator.point_width(&angle, &start_position, &end_position_2,),
            short_distance_2 / whole_distance * resolution_width,
            ulps = 3
        ));
    }

    #[test]
    fn point_height() {
        let resolution_height = 600.0;
        let wall_height = 4.0;
        let point_generator = PointGenerator::new(
            Size {
                width: Default::default(),
                height: resolution_height,
            },
            player_utils::Radians::new(std::f64::consts::PI / 2.0),
            wall_height,
        );
        let half_wall_height = wall_height / 2.0;

        let angle = player_utils::Angle {
            start: player_utils::Radians::new(std::f64::consts::PI / 4.0),
            end: player_utils::Radians::new(std::f64::consts::PI * 3.0 / 4.0),
        };

        let start_position = Coordinate { x: 8.0, y: 1.0 };
        let end_position_1 = Point { x: 8, y: 4 };
        let end_position_2 = Point { x: 4, y: 5 };

        assert!(approx_eq!(
            f64,
            point_generator.point_height(&angle, &start_position, &end_position_1),
            half_wall_height / 3.0 * resolution_height,
            ulps = 3
        ));

        assert!(approx_eq!(
            f64,
            point_generator.point_height(&angle, &start_position, &end_position_2),
            half_wall_height / 4.0 * resolution_height,
            ulps = 3
        ));
    }
}
