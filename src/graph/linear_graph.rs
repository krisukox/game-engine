use crate::graph::coordinate::Coordinate;

#[derive(Debug, Clone)]
pub enum Slope {
    Vertical,
    Value(f64),
}

pub enum Direction {
    Increasing,
    Decreasing,
}

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
}
