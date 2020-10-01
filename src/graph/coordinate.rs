#[derive(PartialEq, Debug)]
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
}
