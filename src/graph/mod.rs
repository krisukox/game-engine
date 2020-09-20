mod coordinate;
mod linear_graph;

pub use self::coordinate::Coordinate;
pub use self::linear_graph::{Direction, LinearGraph, Slope};

#[cfg(test)]
mod tests {
    use super::*;

    fn test_coordinates(
        slope: &Slope,
        first_coordinate: &Coordinate,
        second_coordinate: &Coordinate,
    ) {
        let graph_increasing: LinearGraph = LinearGraph::new(slope.clone(), Direction::Increasing);
        let graph_decreasing: LinearGraph = LinearGraph::new(slope.clone(), Direction::Decreasing);
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
        let positive_slope = 1.5;
        let first_coordinate = Coordinate { x: 1.0, y: 1.5 };

        let coordinate_next_y = 2.0;
        let second_coordinate: Coordinate = Coordinate {
            x: first_coordinate.x + (coordinate_next_y - first_coordinate.y) / positive_slope,
            y: coordinate_next_y,
        };

        test_coordinates(
            &Slope::Value(positive_slope),
            &first_coordinate,
            &second_coordinate,
        );
    }

    #[test]
    fn next_coordinate_negative_slope_y_closer() {
        let negative_slope = -1.5;
        let first_coordinate = Coordinate { x: 1.0, y: 1.5 };

        let coordinate_prev_y = 1.0;
        let second_coordinate: Coordinate = Coordinate {
            x: first_coordinate.x + (coordinate_prev_y - first_coordinate.y) / negative_slope,
            y: coordinate_prev_y,
        };

        test_coordinates(
            &Slope::Value(negative_slope),
            &first_coordinate,
            &second_coordinate,
        );
    }

    #[test]
    fn next_coordinate_positive_slope_x_closer() {
        let positive_slope = 0.25;
        let first_coordinate = Coordinate { x: 1.5, y: 1.0 };

        let coordinate_next_x = 2.0;
        let second_coordinate = Coordinate {
            x: coordinate_next_x,
            y: first_coordinate.y + (coordinate_next_x - first_coordinate.x) * positive_slope,
        };

        test_coordinates(
            &Slope::Value(positive_slope),
            &first_coordinate,
            &second_coordinate,
        );
    }

    #[test]
    fn next_coordinate_negative_slope_x_closer() {
        let negative_slope = -0.25;
        let first_coordinate = Coordinate { x: 1.5, y: 1.0 };

        let coordinate_next_x = 2.0;
        let second_coordinate = Coordinate {
            x: coordinate_next_x,
            y: first_coordinate.y + (coordinate_next_x - first_coordinate.x) * negative_slope,
        };

        test_coordinates(
            &Slope::Value(negative_slope),
            &first_coordinate,
            &second_coordinate,
        );
    }

    #[test]
    fn next_coordinate_vertical() {
        let first_coordinate_1 = Coordinate { x: 1.5, y: 1.0 };
        let second_coordinate_1 = Coordinate { x: 1.5, y: 2.0 };

        let first_coordinate_2 = Coordinate { x: 2.0, y: 1.0 };
        let second_coordinate_2 = Coordinate { x: 2.0, y: 2.0 };

        test_coordinates(&Slope::Vertical, &first_coordinate_1, &second_coordinate_1);
        test_coordinates(&Slope::Vertical, &first_coordinate_2, &second_coordinate_2);
    }

    #[test]
    fn next_coordinate_horizontal() {
        let first_coordinate_1 = Coordinate { x: 2.0, y: 1.5 };
        let second_coordinate_1 = Coordinate { x: 3.0, y: 1.5 };

        let first_coordinate_2 = Coordinate { x: 2.0, y: 2.0 };
        let second_coordinate_2 = Coordinate { x: 3.0, y: 2.0 };

        test_coordinates(
            &Slope::Value(0.0),
            &first_coordinate_1,
            &second_coordinate_1,
        );
        test_coordinates(
            &Slope::Value(0.0),
            &first_coordinate_2,
            &second_coordinate_2,
        );
    }
}
