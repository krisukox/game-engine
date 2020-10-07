use super::coordinate::Coordinate;

#[derive(Clone)]
pub struct Tangens(pub f64);
pub struct Radians(pub f64);

#[derive(Debug, PartialEq)]
pub struct LinearGraph {
    radians: f64,
    tangens: f64,
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

fn generate_one_graph(amount: usize, index: usize) -> LinearGraph {
    if index == 0 {
        return LinearGraph::from_radians(0.0);
    } else if index == amount / 4 {
        return LinearGraph::from_radians(std::f64::consts::PI / 2.0);
    } else if index == amount / 2 {
        return LinearGraph::from_radians(std::f64::consts::PI);
    } else if index == amount * 3 / 4 {
        return LinearGraph::from_radians(std::f64::consts::PI * 3.0 / 2.0);
    } else {
        return LinearGraph::from_radians(
            std::f64::consts::PI * 2.0 * index as f64 / amount as f64,
        );
    }
}

impl LinearGraph {
    pub fn new(tangens: Tangens, radians: Radians) -> LinearGraph {
        LinearGraph {
            tangens: tangens.0,
            radians: radians.0,
        }
    }

    pub fn from_radians(radians: f64) -> LinearGraph {
        if radians == 0.0 || radians == std::f64::consts::PI {
            return LinearGraph {
                radians: radians,
                tangens: 0.0,
            };
        }
        LinearGraph {
            radians: radians,
            tangens: radians.tan(),
        }
    }

    pub fn get_next(&self, current_coordinate: &Coordinate) -> Coordinate {
        if self.radians >= 0.0 && self.radians < std::f64::consts::PI / 2.0 {
            let next_x = next_integer(current_coordinate.x);
            let next_y = next_integer(current_coordinate.y);
            let delta_x = next_x - current_coordinate.x;
            let delta_y = next_y - current_coordinate.y;
            if delta_x * self.tangens < delta_y {
                return Coordinate {
                    x: next_x,
                    y: current_coordinate.y + delta_x * self.tangens,
                };
            } else {
                return Coordinate {
                    x: current_coordinate.x + delta_y / self.tangens,
                    y: next_y,
                };
            }
        } else if self.radians == std::f64::consts::PI / 2.0 {
            return Coordinate {
                x: current_coordinate.x,
                y: next_integer(current_coordinate.y),
            };
        } else if self.radians > std::f64::consts::PI / 2.0 && self.radians < std::f64::consts::PI {
            let prev_x = prev_integer(current_coordinate.x);
            let next_y = next_integer(current_coordinate.y);
            let delta_x = prev_x - current_coordinate.x; // negative
            let delta_y = next_y - current_coordinate.y;
            if -delta_x * -self.tangens < delta_y {
                return Coordinate {
                    x: prev_x,
                    y: current_coordinate.y + delta_x * self.tangens,
                };
            } else {
                return Coordinate {
                    x: current_coordinate.x + delta_y / self.tangens,
                    y: next_y,
                };
            }
        } else if self.radians >= std::f64::consts::PI
            && self.radians < std::f64::consts::PI * 3.0 / 2.0
        {
            let prev_x = prev_integer(current_coordinate.x);
            let prev_y = prev_integer(current_coordinate.y);
            let delta_x = prev_x - current_coordinate.x; // negative
            let delta_y = prev_y - current_coordinate.y; // negative
            if -delta_x * self.tangens < -delta_y {
                return Coordinate {
                    x: prev_x,
                    y: current_coordinate.y + delta_x * self.tangens,
                };
            } else {
                return Coordinate {
                    x: current_coordinate.x + delta_y / self.tangens,
                    y: prev_y,
                };
            }
        } else if self.radians == std::f64::consts::PI * 3.0 / 2.0 {
            return Coordinate {
                x: current_coordinate.x,
                y: prev_integer(current_coordinate.y),
            };
        } else if self.radians > std::f64::consts::PI * 3.0 / 2.0
            && self.radians < std::f64::consts::PI * 2.0
        {
            let next_x = next_integer(current_coordinate.x);
            let prev_y = prev_integer(current_coordinate.y);
            let delta_x = next_x - current_coordinate.x;
            let delta_y = prev_y - current_coordinate.y; // negative
            if delta_x * -self.tangens < -delta_y {
                return Coordinate {
                    x: next_x,
                    y: current_coordinate.y + delta_x * self.tangens,
                };
            } else {
                return Coordinate {
                    x: current_coordinate.x + delta_y / self.tangens,
                    y: prev_y,
                };
            }
        }
        panic!("radians value out of scope");
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

    #[test]
    fn get_all_8_rays() {
        const NUMBER_OF_RAYS: usize = 8_usize;

        let all_rays = LinearGraph::get_all_rays(NUMBER_OF_RAYS);
        let radians: [f64; NUMBER_OF_RAYS] = [
            0.0,
            std::f64::consts::PI / 4.0,
            std::f64::consts::PI / 2.0,
            std::f64::consts::PI * 3.0 / 4.0,
            std::f64::consts::PI,
            std::f64::consts::PI * 5.0 / 4.0,
            std::f64::consts::PI * 6.0 / 4.0,
            std::f64::consts::PI * 7.0 / 4.0,
        ];
        for index in 0..NUMBER_OF_RAYS {
            println!("get_all_rays_8 {:?}", all_rays[index]);
            assert_eq!(all_rays[index], LinearGraph::from_radians(radians[index]));
        }
    }

    #[test]
    fn from_radians() {
        let radians_1 = 0.0_f64;
        let radians_2 = std::f64::consts::PI;
        let radians_3 = 3.0;
        assert_eq!(
            LinearGraph::from_radians(radians_1),
            LinearGraph {
                radians: radians_1,
                tangens: 0.0
            }
        );
        assert_eq!(
            LinearGraph::from_radians(radians_2),
            LinearGraph {
                radians: radians_2,
                tangens: 0.0
            }
        );
        assert_eq!(
            LinearGraph::from_radians(radians_3),
            LinearGraph {
                radians: radians_3,
                tangens: radians_3.tan()
            }
        );
    }
}
