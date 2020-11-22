use crate::graph;
use crate::player_utils;
use piston::window::Size;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub struct PointGenerator {
    pub resolution: Size,
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
        let graph = graph::LinearGraph::from_radians((point_radians - angle.start).0);
        let cross_point = graph::Coordinate {
            x: 1.0 / (graph.tangens + 1.0),
            y: (1.0 / (graph.tangens + 1.0)) * graph.tangens,
        };
        let distance_between = cross_point.distance(&graph::Coordinate { x: 1.0, y: 0.0 });
        let whole_line = 2.0_f64.sqrt();

        return distance_between / whole_line * self.resolution.width as f64;
    }

    fn point_width_outside_field_of_view(
        &self,
        angle: &player_utils::Angle,
        point_radians: &player_utils::Radians,
    ) -> f64 {
        let graph = graph::LinearGraph::from_radians((point_radians - angle.start).0);
        let cross_point = graph::Coordinate {
            x: 1.0 / (graph.tangens + 1.0),
            y: (1.0 / (graph.tangens + 1.0)) * graph.tangens,
        };
        let distance_between = cross_point.distance(&graph::Coordinate { x: 1.0, y: 0.0 });
        let whole_line = 2.0_f64.sqrt();
        if distance_between < whole_line {
            return -distance_between / whole_line * self.resolution.width as f64;
        }
        return distance_between / whole_line * self.resolution.width as f64;
    }

    // returns 1/2 of point height
    pub fn point_height(
        &self,
        angle: &player_utils::Angle,
        start_position: &graph::Coordinate,
        end_position: &graph::Coordinate,
    ) -> f64 {
        let point_radians = start_position.into_radians(end_position);
        let graph = graph::LinearGraph::from_radians((point_radians - angle.start).0);
        let cross_point = graph::Coordinate {
            x: 1.0 / (graph.tangens + 1.0),
            y: (1.0 / (graph.tangens + 1.0)) * graph.tangens,
        };
        let small_distance = graph::ZERO_COORDINATE.distance(&cross_point);
        let whole_distance = start_position.distance(&end_position);
        return (small_distance / whole_distance * self.wall_height / 2.0)
            / graph::LinearGraph::from_radians(self.half_vertical_angle_value.0).tangens
            * self.resolution.height;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] //angle.start < angle.end
    fn point_width_1() {
        let resolution_width = 800.0;
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

    // #[test] //angle.start > angle.end
    // fn point_width_2() {
    //     let resolution_width = 800.0;

    //     let polygon_generator = PointGenerator {
    //         resolution: Size {
    //             width: resolution_width,
    //             height: 0.0,
    //         },
    //         half_vertical_angle_value: Default::default(),
    //         wall_height: Default::default(),
    //     };

    //     let angle = player_utils::Angle {
    //         start: player_utils::Radians(std::f64::consts::PI * 3.0 / 2.0),
    //         end: player_utils::Radians(std::f64::consts::PI / 2.0),
    //     };
    //     let start_position = graph::Coordinate { x: 0.0, y: 0.0 };
    //     assert_eq!(
    //         polygon_generator.point_width(
    //             &angle,
    //             &start_position,
    //             &graph::Coordinate { x: -5.0, y: -5.0 },
    //         ),
    //         -(resolution_width as f64 / 4.0)
    //     );
    //     assert_eq!(
    //         polygon_generator.point_width(
    //             &angle,
    //             &start_position,
    //             &graph::Coordinate { x: 0.0, y: -5.0 },
    //         ),
    //         0.0
    //     );
    //     assert_eq!(
    //         polygon_generator.point_width(
    //             &angle,
    //             &start_position,
    //             &graph::Coordinate { x: 5.0, y: -5.0 },
    //         ),
    //         resolution_width as f64 / 4.0
    //     );
    //     assert_eq!(
    //         polygon_generator.point_width(
    //             &angle,
    //             &start_position,
    //             &graph::Coordinate { x: 5.0, y: 0.0 },
    //         ),
    //         resolution_width as f64 / 2.0
    //     );
    //     assert_eq!(
    //         polygon_generator.point_width(
    //             &angle,
    //             &start_position,
    //             &graph::Coordinate { x: 5.0, y: 5.0 },
    //         ),
    //         resolution_width as f64 * 3.0 / 4.0
    //     );
    //     assert_eq!(
    //         polygon_generator.point_width(
    //             &angle,
    //             &start_position,
    //             &graph::Coordinate { x: 0.0, y: 5.0 },
    //         ),
    //         resolution_width as f64
    //     );
    //     assert_eq!(
    //         polygon_generator.point_width(
    //             &angle,
    //             &start_position,
    //             &graph::Coordinate { x: -5.0, y: 5.0 },
    //         ),
    //         resolution_width as f64 + resolution_width as f64 / 4.0
    //     );
    // }

    // #[test]
    // fn point_height() {
    //     let resolution_height = 600.0;
    //     let resolution_width = 800.0;

    //     let polygon_generator = PointGenerator {
    //         resolution: Size {
    //             width: 0.0,
    //             height: resolution_height,
    //         },
    //         half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 2.0),
    //         wall_height: 5.0,
    //     };

    //     assert_eq!(
    //         polygon_generator.point_height(
    //             &graph::Coordinate { x: 0.0, y: 0.0 },
    //             &graph::Coordinate { x: 5.0, y: 0.0 },
    //         ),
    //         resolution_height as f64 / 4.0
    //     );

    //     let polygon_generator = PointGenerator {
    //         resolution: Size {
    //             width: 0.0,
    //             height: resolution_height,
    //         },
    //         half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 3.0),
    //         wall_height: 5.0,
    //     };

    //     assert_eq!(
    //         polygon_generator.point_height(
    //             &graph::Coordinate { x: 0.0, y: 0.0 },
    //             &graph::Coordinate { x: 5.0, y: 0.0 },
    //         ),
    //         resolution_height as f64 * 3.0 / 8.0
    //     );
    // }

    // #[test]
    // fn new_test() {
    //     let resolution_width = 800.0;
    //     let resolution_height = 600.0;

    //     let polygon_generator = PointGenerator {
    //         resolution: Size {
    //             width: resolution_width,
    //             height: resolution_height,
    //         },
    //         half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 2.0),
    //         wall_height: 5.0,
    //     };
    //     let angle = player_utils::Angle {
    //         start: player_utils::Radians(std::f64::consts::PI / 4.0),
    //         end: player_utils::Radians(3.0 * std::f64::consts::PI / 4.0),
    //     };
    //     let start_point = graph::Coordinate { x: 5.0, y: 0.0 };

    //     let end_point_1 = graph::Coordinate { x: 6.0, y: 3.0 };
    //     let end_point_2 = graph::Coordinate { x: 6.0, y: 4.0 };
    //     let end_point_3 = graph::Coordinate { x: 6.0, y: 5.0 };

    //     let coordinate_1 = graph::Coordinate {
    //         x: polygon_generator.point_width(&angle, &start_point, &end_point_1),
    //         y: polygon_generator.point_height(&start_point, &end_point_1),
    //     };
    //     let coordinate_2 = graph::Coordinate {
    //         x: polygon_generator.point_width(&angle, &start_point, &end_point_2),
    //         y: polygon_generator.point_height(&start_point, &end_point_2),
    //     };
    //     let coordinate_3 = graph::Coordinate {
    //         x: polygon_generator.point_width(&angle, &start_point, &end_point_3),
    //         y: polygon_generator.point_height(&start_point, &end_point_3),
    //     };
    //     println!("{:?}\n{:?}\n{:?}", coordinate_1, coordinate_2, coordinate_3);

    //     let a_graph = (coordinate_1.y - coordinate_2.y) / (coordinate_1.x - coordinate_2.x);
    //     let b_graph = coordinate_1.y - (a_graph * coordinate_1.x);
    //     println!("a_graph {:?}\nb_graph {:?}", a_graph, b_graph);

    //     let delta_x_2_3 = coordinate_3.x - coordinate_2.x;
    //     let delta_y_2_3 = coordinate_3.y - coordinate_2.y;
    //     let calculated_delta_y = delta_x_2_3 * a_graph;

    //     println!(
    //         "delta_x_2_3 {:?} delta_y_2_3 {:?} calculated_delta_y {:?}",
    //         delta_x_2_3, delta_y_2_3, calculated_delta_y
    //     );
    // }
}
