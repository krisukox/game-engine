use super::Point;
use crate::graph::Coordinate;
use crate::map_element::DoorType;

#[cfg(test)]
use mockall::automock;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Rectangle {
    pub point_a: Point,
    pub point_b: Point,
}
#[cfg_attr(test, automock)]
impl Rectangle {
    pub fn get_open_door_area(
        door_area: &Rectangle,
        open_door_area_opt: Option<Rectangle>,
        door_type: &DoorType,
    ) -> Rectangle {
        if let Some(open_door_area) = open_door_area_opt {
            return open_door_area;
        }
        let x1: i64;
        let x2: i64;
        let y1: i64;
        let y2: i64;
        if *door_type == DoorType::Vertical {
            if door_area.point_a.x < door_area.point_b.x {
                x1 = door_area.point_a.x - 10;
                x2 = door_area.point_b.x + 10;
            } else {
                x1 = door_area.point_a.x + 10;
                x2 = door_area.point_b.x - 10;
            }
            if door_area.point_a.y < door_area.point_b.y {
                y1 = door_area.point_a.y - 4;
                y2 = door_area.point_b.y + 4;
            } else {
                y1 = door_area.point_a.y + 4;
                y2 = door_area.point_b.y - 4;
            }
        } else {
            if door_area.point_a.x < door_area.point_b.x {
                x1 = door_area.point_a.x - 4;
                x2 = door_area.point_b.x + 4;
            } else {
                x1 = door_area.point_a.x + 4;
                x2 = door_area.point_b.x - 4;
            }
            if door_area.point_a.y < door_area.point_b.y {
                y1 = door_area.point_a.y - 10;
                y2 = door_area.point_b.y + 10;
            } else {
                y1 = door_area.point_a.y + 10;
                y2 = door_area.point_b.y - 10;
            }
        }
        return Rectangle {
            point_a: Point::new_i64(x1, y1),
            point_b: Point::new_i64(x2, y2),
        };
    }

    pub(crate) fn is_point_in_object(&self, point: &Point) -> bool {
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

    pub(crate) fn is_coordinate_in_object(&self, point: &Coordinate) -> bool {
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
    fn get_open_door_area_default() {
        let door_area = Rectangle {
            point_a: Point { x: 2, y: 1 },
            point_b: Point { x: 3, y: 6 },
        };
        let expected_open_door_area = Rectangle {
            point_a: Point::new_i64(door_area.point_a.x - 10, door_area.point_a.y - 4),
            point_b: Point::new_i64(door_area.point_b.x + 10, door_area.point_b.y + 4),
        };
        assert_eq!(
            Rectangle::get_open_door_area(&door_area, None, &DoorType::Vertical),
            expected_open_door_area
        );
        let door_area = Rectangle {
            point_a: Point { x: 3, y: 6 },
            point_b: Point { x: 2, y: 1 },
        };
        let expected_open_door_area = Rectangle {
            point_a: Point::new_i64(door_area.point_a.x + 10, door_area.point_a.y + 4),
            point_b: Point::new_i64(door_area.point_b.x - 10, door_area.point_b.y - 4),
        };
        assert_eq!(
            Rectangle::get_open_door_area(&door_area, None, &DoorType::Vertical),
            expected_open_door_area
        );
        let door_area = Rectangle {
            point_a: Point { x: 4, y: 1 },
            point_b: Point { x: 9, y: 2 },
        };
        let expected_open_door_area = Rectangle {
            point_a: Point::new_i64(door_area.point_a.x - 4, door_area.point_a.y - 10),
            point_b: Point::new_i64(door_area.point_b.x + 4, door_area.point_b.y + 10),
        };
        assert_eq!(
            Rectangle::get_open_door_area(&door_area, None, &DoorType::Horizontal),
            expected_open_door_area
        );
        let door_area = Rectangle {
            point_a: Point { x: 9, y: 2 },
            point_b: Point { x: 4, y: 1 },
        };
        let expected_open_door_area = Rectangle {
            point_a: Point::new_i64(door_area.point_a.x + 4, door_area.point_a.y + 10),
            point_b: Point::new_i64(door_area.point_b.x - 4, door_area.point_b.y - 10),
        };
        assert_eq!(
            Rectangle::get_open_door_area(&door_area, None, &DoorType::Horizontal),
            expected_open_door_area
        );
    }

    #[test]
    fn get_open_door_area_custom() {
        let open_door_area = Rectangle {
            point_a: Point { x: 9, y: 2 },
            point_b: Point { x: 4, y: 1 },
        };
        assert_eq!(
            Rectangle::get_open_door_area(
                &Default::default(),
                Some(open_door_area.clone()),
                &DoorType::Horizontal
            ),
            open_door_area
        );
    }

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
