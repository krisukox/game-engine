use super::Point;
use crate::graph::Coordinate;

#[cfg(test)]
use mockall::automock;

#[derive(Clone, Default)]
pub struct Rectangle {
    pub point_a: Point,
    pub point_b: Point,
}
#[cfg_attr(test, automock)]
impl Rectangle {
    pub fn is_point_in_object(&self, point: &Point) -> bool {
        if self.point_a.x < self.point_b.x {
            if point.x < self.point_a.x || point.x > self.point_b.x {
                return false;
            }
        } else {
            if point.x > self.point_a.x || point.x < self.point_b.x {
                return false;
            }
        }

        if self.point_a.y < self.point_b.y {
            if point.y < self.point_a.y || point.y > self.point_b.y {
                return false;
            }
        } else {
            if point.y > self.point_a.y || point.y < self.point_b.y {
                return false;
            }
        }
        return true;
    }

    pub fn is_coordinate_in_object(&self, point: &Coordinate) -> bool {
        if self.point_a.x < self.point_b.x {
            if point.x < self.point_a.x as f64 || point.x > self.point_b.x as f64 {
                return false;
            }
        } else {
            if point.x > self.point_a.x as f64 || point.x < self.point_b.x as f64 {
                return false;
            }
        }

        if self.point_a.y < self.point_b.y {
            if point.y < self.point_a.y as f64 || point.y > self.point_b.y as f64 {
                return false;
            }
        } else {
            if point.y > self.point_a.y as f64 || point.y < self.point_b.y as f64 {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point_in_object_1() {
        let rectangle = Rectangle {
            point_a: Point { x: 3, y: 4 },
            point_b: Point { x: 6, y: 1 },
        };
        let point_in_object_1 = Point { x: 5, y: 3 };
        let point_in_object_2 = Point { x: 6, y: 4 };
        let point_out_object_1 = Point { x: 1, y: 2 };
        let point_out_object_2 = Point { x: 5, y: 5 };
        assert!(rectangle.is_point_in_object(&point_in_object_1));
        assert!(rectangle.is_point_in_object(&point_in_object_2));
        assert!(!rectangle.is_point_in_object(&point_out_object_1));
        assert!(!rectangle.is_point_in_object(&point_out_object_2));
    }

    #[test]
    fn is_point_in_object_2() {
        let rectangle = Rectangle {
            point_a: Point { x: 6, y: 1 },
            point_b: Point { x: 3, y: 4 },
        };
        let point_in_object_1 = Point { x: 5, y: 3 };
        let point_in_object_2 = Point { x: 6, y: 4 };
        let point_out_object_1 = Point { x: 1, y: 2 };
        let point_out_object_2 = Point { x: 5, y: 5 };
        assert!(rectangle.is_point_in_object(&point_in_object_1));
        assert!(rectangle.is_point_in_object(&point_in_object_2));
        assert!(!rectangle.is_point_in_object(&point_out_object_1));
        assert!(!rectangle.is_point_in_object(&point_out_object_2));
    }

    #[test]
    fn is_coordinate_in_object_1() {
        let rectangle = Rectangle {
            point_a: Point { x: 3, y: 4 },
            point_b: Point { x: 6, y: 1 },
        };
        let coordinate_in_object_1 = Coordinate { x: 5.5, y: 3.5 };
        let coordinate_in_object_2 = Coordinate { x: 6.0, y: 4.0 };
        let coordinate_out_object_1 = Coordinate { x: 1.5, y: 2.5 };
        let coordinate_out_object_2 = Coordinate { x: 5.5, y: 4.5 };
        assert!(rectangle.is_coordinate_in_object(&coordinate_in_object_1));
        assert!(rectangle.is_coordinate_in_object(&coordinate_in_object_2));
        assert!(!rectangle.is_coordinate_in_object(&coordinate_out_object_1));
        assert!(!rectangle.is_coordinate_in_object(&coordinate_out_object_2));
    }

    #[test]
    fn is_coordinate_in_object_2() {
        let rectangle = Rectangle {
            point_a: Point { x: 6, y: 1 },
            point_b: Point { x: 3, y: 4 },
        };
        let coordinate_in_object_1 = Coordinate { x: 5.5, y: 3.5 };
        let coordinate_in_object_2 = Coordinate { x: 6.0, y: 4.0 };
        let coordinate_out_object_1 = Coordinate { x: 1.5, y: 2.5 };
        let coordinate_out_object_2 = Coordinate { x: 5.5, y: 4.5 };
        assert!(rectangle.is_coordinate_in_object(&coordinate_in_object_1));
        assert!(rectangle.is_coordinate_in_object(&coordinate_in_object_2));
        assert!(!rectangle.is_coordinate_in_object(&coordinate_out_object_1));
        assert!(!rectangle.is_coordinate_in_object(&coordinate_out_object_2));
    }
}
