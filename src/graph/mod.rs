mod coordinate;
mod linear_graph;
mod wall;

pub use self::coordinate::{Coordinate, ZERO_COORDINATE};
pub use self::linear_graph::LinearGraph;
pub use self::wall::{Wall, Walls};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player_utils::Radians;

    fn test_coordinates(
        tangens: f64,
        radians: f64,
        first_coordinate: &Coordinate,
        second_coordinate: &Coordinate,
    ) {
        let radians_1 = if radians < 0.0 {
            radians + std::f64::consts::PI * 2.0
        } else {
            radians
        };
        let radians_2 = if radians_1 + std::f64::consts::PI < std::f64::consts::PI * 2.0 {
            radians_1 + std::f64::consts::PI
        } else {
            radians_1 - std::f64::consts::PI
        };
        let graph_increasing: LinearGraph = LinearGraph {
            tangens,
            radians: Radians::new(radians_1),
        };
        let graph_decreasing: LinearGraph = LinearGraph {
            tangens,
            radians: Radians::new(radians_2),
        };
        let result_second_coordinate = graph_increasing.get_next(&first_coordinate);
        let result_first_coordinate = graph_decreasing.get_next(&second_coordinate);

        assert_eq!(*first_coordinate, result_first_coordinate);
        assert_eq!(*second_coordinate, result_second_coordinate);

        assert_eq!(
            *first_coordinate,
            graph_decreasing.get_next(&result_second_coordinate)
        );
        assert_eq!(
            *second_coordinate,
            graph_increasing.get_next(&result_first_coordinate)
        );
    }

    #[test]
    fn next_coordinate_positive_slope_y_closer() {
        let tangens = 1.5_f64;
        let radians = tangens.atan();
        let first_coordinate = Coordinate { x: 1.0, y: 1.5 };

        let coordinate_next_y = 2.0;
        let second_coordinate: Coordinate = Coordinate {
            x: first_coordinate.x + (coordinate_next_y - first_coordinate.y) / tangens,
            y: coordinate_next_y,
        };

        test_coordinates(tangens, radians, &first_coordinate, &second_coordinate);
    }

    #[test]
    fn next_coordinate_negative_slope_y_closer() {
        let tangens = -1.5_f64;
        let radians = tangens.atan();
        let first_coordinate = Coordinate { x: 1.0, y: 1.5 };

        let coordinate_prev_y = 1.0;
        let second_coordinate: Coordinate = Coordinate {
            x: first_coordinate.x + (coordinate_prev_y - first_coordinate.y) / tangens,
            y: coordinate_prev_y,
        };

        test_coordinates(tangens, radians, &first_coordinate, &second_coordinate);
    }

    #[test]
    fn next_coordinate_positive_slope_x_closer() {
        let tangens = 0.25_f64;
        let radians = tangens.atan();
        let first_coordinate = Coordinate { x: 1.5, y: 1.0 };

        let coordinate_next_x = 2.0;
        let second_coordinate = Coordinate {
            x: coordinate_next_x,
            y: first_coordinate.y + (coordinate_next_x - first_coordinate.x) * tangens,
        };

        test_coordinates(tangens, radians, &first_coordinate, &second_coordinate);
    }

    #[test]
    fn next_coordinate_negative_slope_x_closer() {
        let tangens = -0.25_f64;
        let radians = tangens.atan();
        let first_coordinate = Coordinate { x: 1.5, y: 1.0 };

        let coordinate_next_x = 2.0;
        let second_coordinate = Coordinate {
            x: coordinate_next_x,
            y: first_coordinate.y + (coordinate_next_x - first_coordinate.x) * tangens,
        };

        test_coordinates(tangens, radians, &first_coordinate, &second_coordinate);
    }

    #[test]
    fn next_coordinate_vertical() {
        let first_coordinate_1 = Coordinate { x: 1.5, y: 1.0 };
        let second_coordinate_1 = Coordinate { x: 1.5, y: 2.0 };

        let first_coordinate_2 = Coordinate { x: 2.0, y: 2.0 };
        let second_coordinate_2 = Coordinate { x: 2.0, y: 1.0 };

        test_coordinates(
            std::f64::INFINITY,
            std::f64::consts::PI / 2.0,
            &first_coordinate_1,
            &second_coordinate_1,
        );
        test_coordinates(
            std::f64::NEG_INFINITY,
            std::f64::consts::PI * 3.0 / 2.0,
            &first_coordinate_2,
            &second_coordinate_2,
        );
    }

    #[test]
    fn next_coordinate_horizontal() {
        let first_coordinate_1 = Coordinate { x: 2.0, y: 1.5 };
        let second_coordinate_1 = Coordinate { x: 3.0, y: 1.5 };

        let first_coordinate_2 = Coordinate { x: 3.0, y: 2.0 };
        let second_coordinate_2 = Coordinate { x: 2.0, y: 2.0 };

        test_coordinates(0.0, 0.0, &first_coordinate_1, &second_coordinate_1);
        test_coordinates(
            0.0,
            std::f64::consts::PI,
            &first_coordinate_2,
            &second_coordinate_2,
        );
    }
}
