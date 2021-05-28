use crate::player_utils;
// use float_cmp::approx_eq;

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

pub const ZERO_COORDINATE: Coordinate = Coordinate { x: 0.0, y: 0.0 };

impl Coordinate {
    pub fn distance(&self, coordinate: &Coordinate) -> f64 {
        return (((self.x - coordinate.x).powf(2_f64) + (self.y - coordinate.y).powf(2_f64))
            as f64)
            .sqrt();
    }

    pub fn get_nearest_coordinates(&self, last_coordinate: &Coordinate) -> Vec<Coordinate> {
        let x_floor = self.x.floor();
        let y_floor = self.y.floor();
        if x_floor == self.x && y_floor == self.y {
            return vec![self.clone()];
        } else if x_floor == self.x {
            if last_coordinate.x < x_floor {
                return vec![
                    Coordinate {
                        x: self.x,
                        y: y_floor,
                    },
                    Coordinate {
                        x: self.x,
                        y: self.y.ceil(),
                    },
                ];
            }
            return vec![
                Coordinate {
                    x: self.x,
                    y: self.y.ceil(),
                },
                Coordinate {
                    x: self.x,
                    y: y_floor,
                },
            ];
        } else if y_floor == self.y {
            if last_coordinate.y < y_floor {
                return vec![
                    Coordinate {
                        x: self.x.ceil(),
                        y: self.y,
                    },
                    Coordinate {
                        x: x_floor,
                        y: self.y,
                    },
                ];
            }
            return vec![
                Coordinate {
                    x: x_floor,
                    y: self.y,
                },
                Coordinate {
                    x: self.x.ceil(),
                    y: self.y,
                },
            ];
        }
        println!("It shouldn't heppend but treat it normally");
        return vec![Coordinate {
            x: self.x.round(),
            y: self.y.round(),
        }];
    }

    pub fn into_radians(&self, end_coordinate: &Coordinate) -> player_utils::Radians {
        let delta_x = self.x - end_coordinate.x;
        let delta_y = self.y - end_coordinate.y;
        if delta_x == 0.0 {
            if self.y < end_coordinate.y {
                return player_utils::Radians::new(std::f64::consts::PI / 2.0);
            }
            return player_utils::Radians::new(std::f64::consts::PI * 3.0 / 2.0);
        }
        if self.x < end_coordinate.x {
            if self.y > end_coordinate.y {
                return player_utils::Radians::new((delta_y / delta_x).atan() + player_utils::PI_2);
            }
            return player_utils::Radians::new((delta_y / delta_x).atan());
        }
        return player_utils::Radians::new((delta_y / delta_x).atan() + std::f64::consts::PI);
    }
}

impl std::ops::AddAssign<&Coordinate> for Coordinate {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// impl PartialEq for Coordinate {
//     fn eq(&self, other: &Self) -> bool {
//         self.isbn == other.isbn
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let coordinate_1 = Coordinate { x: 1.0, y: 2.0 };
        let coordinate_2 = Coordinate { x: 2.0, y: 3.0 };
        let coordinate_3 = Coordinate { x: 4.0, y: 6.0 };
        assert_eq!(coordinate_1.distance(&coordinate_2), (1_f64 + 1_f64).sqrt());
        assert_eq!(
            coordinate_3.distance(&coordinate_1),
            (9_f64 + 16_f64).sqrt()
        );
    }

    #[test]
    fn get_nearest_coordinates() {
        let coordinate_1 = Coordinate { x: 5.0, y: 6.0 };
        let coordinate_2 = Coordinate { x: 5.5, y: 4.5 };
        let coordinate_3 = Coordinate { x: 1.0, y: 2.5 };
        let coordinate_3_last_1 = Coordinate { x: 0.5, y: 1.0 };
        let coordinate_3_last_2 = Coordinate { x: 1.5, y: 2.0 };
        let coordinate_4 = Coordinate { x: 3.5, y: 4.0 };
        let coordinate_4_last_1 = Coordinate { x: 3.0, y: 3.5 };
        let coordinate_4_last_2 = Coordinate { x: 4.0, y: 4.5 };

        assert_eq!(
            coordinate_1.get_nearest_coordinates(&Default::default()),
            vec![Coordinate {
                x: coordinate_1.x,
                y: coordinate_1.y
            }]
        );

        assert_eq!(
            coordinate_2.get_nearest_coordinates(&Default::default()),
            vec![Coordinate {
                x: coordinate_2.x.round(),
                y: coordinate_2.y.round()
            }]
        );

        assert_eq!(
            coordinate_3.get_nearest_coordinates(&coordinate_3_last_1),
            vec![
                Coordinate {
                    x: coordinate_3.x,
                    y: coordinate_3.y.floor()
                },
                Coordinate {
                    x: coordinate_3.x,
                    y: coordinate_3.y.ceil()
                }
            ]
        );
        assert_eq!(
            coordinate_3.get_nearest_coordinates(&coordinate_3_last_2),
            vec![
                Coordinate {
                    x: coordinate_3.x,
                    y: coordinate_3.y.ceil()
                },
                Coordinate {
                    x: coordinate_3.x,
                    y: coordinate_3.y.floor()
                }
            ]
        );

        assert_eq!(
            coordinate_4.get_nearest_coordinates(&coordinate_4_last_1),
            vec![
                Coordinate {
                    x: coordinate_4.x.ceil(),
                    y: coordinate_4.y
                },
                Coordinate {
                    x: coordinate_4.x.floor(),
                    y: coordinate_4.y
                }
            ]
        );
        assert_eq!(
            coordinate_4.get_nearest_coordinates(&coordinate_4_last_2),
            vec![
                Coordinate {
                    x: coordinate_4.x.floor(),
                    y: coordinate_4.y
                },
                Coordinate {
                    x: coordinate_4.x.ceil(),
                    y: coordinate_4.y
                }
            ]
        );
    }

    #[test]
    fn into_radians() {
        let start_coordinate = Coordinate { x: 0.0, y: 0.0 };
        let end_coordinates = vec![
            Coordinate { x: 1.0, y: 0.0 },
            Coordinate { x: 1.0, y: 1.0 },
            Coordinate { x: 0.0, y: 1.0 },
            Coordinate { x: -1.0, y: 1.0 },
            Coordinate { x: -1.0, y: 0.0 },
            Coordinate { x: -1.0, y: -1.0 },
            Coordinate { x: 0.0, y: -1.0 },
            Coordinate { x: 1.0, y: -1.0 },
        ];
        let mut radian = player_utils::Radians::new(0.0);

        for end_coordinate in end_coordinates {
            assert_eq!(start_coordinate.into_radians(&end_coordinate), radian);
            radian += player_utils::Radians::new(std::f64::consts::PI / 4.0);
        }
    }
}
