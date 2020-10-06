#[derive(PartialEq, Debug, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub fn distance(&self, coordinate: &Coordinate) -> f64 {
        return (((self.x - coordinate.x).powf(2_f64) + (self.y - coordinate.y).powf(2_f64))
            as f64)
            .sqrt();
    }

    pub fn get_nearest_coordinates(&self) -> Vec<Coordinate> {
        let x_floor = self.x.floor();
        let y_floor = self.y.floor();
        if x_floor == self.x && y_floor == self.y {
            return vec![self.clone()];
        } else if x_floor == self.x {
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
        } else if y_floor == self.y {
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
}

impl std::ops::AddAssign<&Coordinate> for Coordinate {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

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
    fn get_near_coordinates() {
        let coordinate_1 = Coordinate { x: 1.0, y: 2.5 };
        let coordinate_2 = Coordinate { x: 3.5, y: 4.0 };
        let coordinate_3 = Coordinate { x: 5.0, y: 6.0 };
        let coordinate_4 = Coordinate { x: 5.5, y: 4.5 };
        let points_1 = coordinate_1.get_nearest_coordinates();
        let points_2 = coordinate_2.get_nearest_coordinates();
        let points_3 = coordinate_3.get_nearest_coordinates();
        let points_4 = coordinate_4.get_nearest_coordinates();
        assert_eq!(points_1.len(), 2);
        assert_eq!(points_2.len(), 2);
        assert_eq!(points_3.len(), 1);
        assert_eq!(points_4.len(), 1);

        assert!(points_1.contains(&Coordinate {
            x: coordinate_1.x,
            y: coordinate_1.y.ceil()
        }));
        assert!(points_1.contains(&Coordinate {
            x: coordinate_1.x,
            y: coordinate_1.y.floor()
        }));

        assert!(points_2.contains(&Coordinate {
            x: coordinate_2.x.ceil(),
            y: coordinate_2.y
        }));
        assert!(points_2.contains(&Coordinate {
            x: coordinate_2.x.floor(),
            y: coordinate_2.y
        }));

        assert_eq!(
            points_3[0],
            Coordinate {
                x: coordinate_3.x,
                y: coordinate_3.y
            }
        );

        assert_eq!(
            points_4[0],
            Coordinate {
                x: coordinate_4.x.round(),
                y: coordinate_4.y.round()
            }
        );
    }
}
