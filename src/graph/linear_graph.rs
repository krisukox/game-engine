use super::coordinate::Coordinate;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Slope {
    Vertical,
    Value(f64),
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

// Increasing and Decreasing refers to x value if Slope is Value
// Increasing and Decreasing refers to y value if Slope is Vertical
//
// Decr      Incr
//   _       _
//  |\   |y  /|
//    \  |  /
//     \ | /
// x____\|/_____
//      /|\
//     / | \
//    /  |  \
//  |/_  |  _\|
// Decr     Incr

#[derive(Debug, PartialEq)]
pub struct LinearGraph {
    slope: Slope,
    direction: Direction,
}

fn next_integer(value: f64) -> f64 {
    let next_value = value.ceil();
    if next_value == value {
        return next_value + 1_f64;
    }
    return next_value;
}

fn prev_integer(value: f64) -> f64 {
    let prev_value = value.floor();
    if prev_value == value {
        return prev_value - 1_f64;
    }
    return prev_value;
}

fn calculate_slope(amount: usize, index: usize) -> Slope {
    let radians = std::f64::consts::PI * index as f64 / (amount / 2) as f64;
    return Slope::Value(radians.tan());
}

fn calculate_graph(amount: usize, index: usize) -> LinearGraph {
    if index > amount / 2 {
        return LinearGraph {
            slope: calculate_slope(amount, index - amount / 2),
            direction: Direction::Decreasing,
        };
    } else {
        return LinearGraph {
            slope: calculate_slope(amount, index),
            direction: Direction::Increasing,
        };
    }
}

fn generate_one_graph(amount: usize, index: usize) -> LinearGraph {
    if index == 0 {
        return LinearGraph {
            slope: Slope::Value(0.0),
            direction: Direction::Increasing,
        };
    } else if index == amount / 4 {
        return LinearGraph {
            slope: Slope::Vertical,
            direction: Direction::Increasing,
        };
    } else if index == amount / 2 {
        return LinearGraph {
            slope: Slope::Value(0.0),
            direction: Direction::Decreasing,
        };
    } else if index == amount * 3 / 4 {
        return LinearGraph {
            slope: Slope::Vertical,
            direction: Direction::Decreasing,
        };
    } else {
        return calculate_graph(amount, index);
    }
}

impl LinearGraph {
    pub fn new(slope: Slope, direction: Direction) -> LinearGraph {
        LinearGraph { slope, direction }
    }

    pub fn get_next(&self, current_coordinate: &Coordinate) -> Coordinate {
        match self.direction {
            Direction::Increasing => {
                if let Slope::Value(slope) = self.slope {
                    if slope > 0_f64 {
                        let next_x = next_integer(current_coordinate.x);
                        let next_y = next_integer(current_coordinate.y);
                        let delta_x = next_x - current_coordinate.x;
                        let delta_y = next_y - current_coordinate.y;
                        if delta_x * slope < delta_y {
                            return Coordinate {
                                x: next_x,
                                y: current_coordinate.y + delta_x * slope,
                            };
                        } else {
                            return Coordinate {
                                x: current_coordinate.x + delta_y / slope,
                                y: next_y,
                            };
                        }
                    } else {
                        let next_x = next_integer(current_coordinate.x);
                        let prev_y = prev_integer(current_coordinate.y);
                        let delta_x = next_x - current_coordinate.x;
                        let delta_y = prev_y - current_coordinate.y; // negative
                        if delta_x * -slope < -delta_y {
                            return Coordinate {
                                x: next_x,
                                y: current_coordinate.y + delta_x * slope,
                            };
                        } else {
                            return Coordinate {
                                x: current_coordinate.x + delta_y / slope,
                                y: prev_y,
                            };
                        }
                    }
                } else {
                    return Coordinate {
                        x: current_coordinate.x,
                        y: next_integer(current_coordinate.y),
                    };
                }
            }
            Direction::Decreasing => {
                if let Slope::Value(slope) = self.slope {
                    if slope > 0_f64 {
                        let prev_x = prev_integer(current_coordinate.x);
                        let prev_y = prev_integer(current_coordinate.y);
                        let delta_x = prev_x - current_coordinate.x; // negative
                        let delta_y = prev_y - current_coordinate.y; // negative
                        if -delta_x * slope < -delta_y {
                            return Coordinate {
                                x: prev_x,
                                y: current_coordinate.y + delta_x * slope,
                            };
                        } else {
                            return Coordinate {
                                x: current_coordinate.x + delta_y / slope,
                                y: prev_y,
                            };
                        }
                    } else {
                        let prev_x = prev_integer(current_coordinate.x);
                        let next_y = next_integer(current_coordinate.y);
                        let delta_x = prev_x - current_coordinate.x; // negative
                        let delta_y = next_y - current_coordinate.y;
                        if -delta_x * -slope < delta_y {
                            return Coordinate {
                                x: prev_x,
                                y: current_coordinate.y + delta_x * slope,
                            };
                        } else {
                            return Coordinate {
                                x: current_coordinate.x + delta_y / slope,
                                y: next_y,
                            };
                        }
                    }
                } else {
                    return Coordinate {
                        x: current_coordinate.x,
                        y: prev_integer(current_coordinate.y),
                    };
                }
            }
        }
    }

    pub fn get_all_rays(number_of_rays: usize) -> Vec<LinearGraph> {
        let mut all_rays = Vec::with_capacity(number_of_rays);
        for index in 0..number_of_rays {
            all_rays.push(generate_one_graph(number_of_rays, index));
        }
        return all_rays;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOAT_1: f64 = 1.6;
    const FLOAT_2: f64 = 2.0;
    const FLOAT_3: f64 = 2.4;

    #[test]
    fn test_next_integer() {
        const NEXT_INTEGER_1: f64 = 2.0;
        const NEXT_INTEGER_2: f64 = 3.0;
        const NEXT_INTEGER_3: f64 = 3.0;

        assert_eq!(next_integer(FLOAT_1), NEXT_INTEGER_1);
        assert_eq!(next_integer(FLOAT_2), NEXT_INTEGER_2);
        assert_eq!(next_integer(FLOAT_3), NEXT_INTEGER_3);

        assert_ne!(next_integer(FLOAT_1), NEXT_INTEGER_3);
        assert_ne!(next_integer(FLOAT_2), NEXT_INTEGER_1);
        assert_ne!(next_integer(FLOAT_3), NEXT_INTEGER_1);
    }

    #[test]
    fn test_prev_integer() {
        const PREV_INTEGER_1: f64 = 1.0;
        const PREV_INTEGER_2: f64 = 1.0;
        const PREV_INTEGER_3: f64 = 2.0;

        assert_eq!(prev_integer(FLOAT_1), PREV_INTEGER_1);
        assert_eq!(prev_integer(FLOAT_2), PREV_INTEGER_2);
        assert_eq!(prev_integer(FLOAT_3), PREV_INTEGER_3);

        assert_ne!(prev_integer(FLOAT_1), PREV_INTEGER_3);
        assert_ne!(prev_integer(FLOAT_2), PREV_INTEGER_3);
        assert_ne!(prev_integer(FLOAT_3), PREV_INTEGER_1);
    }

    #[test]
    fn get_all_rays_check_size() {
        let size = 123456_usize;
        assert_eq!(LinearGraph::get_all_rays(size).len(), size);
    }

    // #[test]
    // fn get_all_rays_4() {
    //     let all_rays = LinearGraph::get_all_rays(8);
    //     assert_eq!(
    //         all_rays[0],
    //         LinearGraph {
    //             slope: Slope::Value(0.0),
    //             direction: Direction::Increasing,
    //         }
    //     );
    //     assert_eq!(
    //         all_rays[1],
    //         LinearGraph {
    //             slope: Slope::Value(std::f64::consts::PI / 4.0),
    //             direction: Direction::Increasing,
    //         }
    //     );
    //     assert_eq!(
    //         all_rays[2],
    //         LinearGraph {
    //             slope: Slope::Vertical,
    //             direction: Direction::Decreasing,
    //         }
    //     );
    //     assert_eq!(
    //         all_rays[3],
    //         LinearGraph {
    //             slope: Slope::Value(std::f64::consts::PI * 3.0 / 4.0),
    //             direction: Direction::Decreasing,
    //         }
    //     );
    //     assert_eq!(
    //         all_rays[4],
    //         LinearGraph {
    //             slope: Slope::Value(0.0),
    //             direction: Direction::Decreasing,
    //         }
    //     );
    //     assert_eq!(
    //         all_rays[5],
    //         LinearGraph {
    //             slope: Slope::Value(0.0),
    //             direction: Direction::Decreasing,
    //         }
    //     );
    //     assert_eq!(
    //         all_rays[6],
    //         LinearGraph {
    //             slope: Slope::Vertical,
    //             direction: Direction::Increasing,
    //         }
    //     );
    //     assert_eq!(
    //         all_rays[7],
    //         LinearGraph {
    //             slope: Slope::Value(0.0),
    //             direction: Direction::Increasing,
    //         }
    //     );
    // }
}
