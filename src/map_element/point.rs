use crate::graph::Coordinate;

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: if x < 0.0 { 0 } else { x as i64 },
            y: if y < 0.0 { 0 } else { y as i64 },
        }
    }

    pub fn new_i64(x: i64, y: i64) -> Self {
        Self {
            x: if x < 0 { 0 } else { x },
            y: if y < 0 { 0 } else { y },
        }
    }

    pub(crate) fn distance(&self, point: &Self) -> f64 {
        return (((self.x - point.x).pow(2) + (self.y - point.y).pow(2)) as f64).sqrt();
    }

    pub(crate) fn distance_coor(&self, coordinate: &Coordinate) -> f64 {
        return ((self.x as f64 - coordinate.x).powf(2.0)
            + (self.y as f64 - coordinate.y).powf(2.0))
        .sqrt();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let point = Point::new(-1.5, 6.6);
        assert_eq!(Point { x: 0, y: 6 }, point);
    }

    #[test]
    fn new_i64() {
        let point = Point::new_i64(-2, 6);
        assert_eq!(Point { x: 0, y: 6 }, point);
    }

    #[test]
    fn distance() {
        let point_1 = Point::new_i64(3, 6);
        let point_2 = Point::new_i64(7, 10);
        let distance = 32_f64.sqrt();
        assert_eq!(point_1.distance(&point_2), distance);
    }

    #[test]
    fn distance_coor() {
        let point = Point::new_i64(3, 6);
        let coordinate = Coordinate { x: 5.5, y: 8.5 };
        let distance = 12.5_f64.sqrt();
        assert_eq!(point.distance_coor(&coordinate), distance);
    }
}
