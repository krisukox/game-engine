use super::{DoorType, Point, Rectangle};

#[cfg(test)]
use mockall::automock;

pub struct HalfDoor {
    pub start_point: Point,
    pub end_point: Point,
    pub rectangle: Rectangle,
    pub door_type: DoorType,
}

macro_rules! open_door_step {
    ($id:ident,$exp:tt) => {
        if $id.start_point.$exp < $id.end_point.$exp {
            if $id.end_point.$exp == $id.rectangle.point_a.$exp {
                $id.rectangle.point_b.$exp += 1;
            } else {
                $id.rectangle.point_a.$exp += 1;
            }
        } else {
            if $id.end_point.$exp == $id.rectangle.point_a.$exp {
                $id.rectangle.point_b.$exp -= 1;
            } else {
                $id.rectangle.point_a.$exp -= 1;
            }
        }
    };
}

macro_rules! close_door_step {
    ($id:ident,$exp:tt) => {
        if $id.start_point.$exp < $id.end_point.$exp {
            if $id.end_point.$exp == $id.rectangle.point_a.$exp {
                $id.rectangle.point_b.$exp -= 1;
            } else {
                $id.rectangle.point_a.$exp -= 1;
            }
        } else {
            if $id.end_point.$exp == $id.rectangle.point_a.$exp {
                $id.rectangle.point_b.$exp += 1;
            } else {
                $id.rectangle.point_a.$exp += 1;
            }
        }
    };
}

#[cfg_attr(test, automock)]
impl HalfDoor {
    fn is_opened(&self) -> bool {
        if self.door_type == DoorType::Vertical {
            if self.rectangle.point_a.y == self.rectangle.point_b.y {
                return true;
            }
        } else {
            if self.rectangle.point_a.x == self.rectangle.point_b.x {
                return true;
            }
        }
        return false;
    }

    pub fn open_door_step(&mut self) -> bool {
        if self.is_opened() {
            return true;
        }
        if self.door_type == DoorType::Vertical {
            open_door_step!(self, y);
        } else {
            open_door_step!(self, x);
        }
        return false;
    }

    fn is_closed(&self) -> bool {
        if self.door_type == DoorType::Vertical {
            if self.rectangle.point_a.y == self.start_point.y
                || self.rectangle.point_b.y == self.start_point.y
            {
                return true;
            }
        } else {
            if self.rectangle.point_a.x == self.start_point.x
                || self.rectangle.point_b.x == self.start_point.x
            {
                return true;
            }
        }
        return false;
    }

    pub fn close_door_step(&mut self) -> bool {
        if self.is_closed() {
            return true;
        }
        if self.door_type == DoorType::Vertical {
            close_door_step!(self, y);
        } else {
            close_door_step!(self, x);
        }
        return false;
    }

    pub fn is_point_in_object(&self, point: &Point) -> bool {
        self.rectangle.is_point_in_object(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_door_step_already_opened() {
        let mut half_door_vertical = HalfDoor {
            start_point: Default::default(),
            end_point: Default::default(),
            rectangle: Rectangle {
                point_a: Point { x: 8, y: 5 },
                point_b: Point { x: 10, y: 5 },
            },
            door_type: DoorType::Vertical,
        };
        let mut half_door_horizontal = HalfDoor {
            start_point: Default::default(),
            end_point: Default::default(),
            rectangle: Rectangle {
                point_a: Point { x: 8, y: 5 },
                point_b: Point { x: 8, y: 7 },
            },
            door_type: DoorType::Horizontal,
        };

        assert!(half_door_vertical.open_door_step());
        assert!(half_door_horizontal.open_door_step());
    }

    #[test]
    fn close_door_step_already_closed() {
        let mut half_door_vertical = HalfDoor {
            start_point: Point { x: 10, y: 5 },
            end_point: Default::default(),
            rectangle: Rectangle {
                point_a: Point { x: 8, y: 5 },
                point_b: Point { x: 10, y: 5 },
            },
            door_type: DoorType::Vertical,
        };
        let mut half_door_horizontal = HalfDoor {
            start_point: Point { x: 8, y: 5 },
            end_point: Default::default(),
            rectangle: Rectangle {
                point_a: Point { x: 8, y: 5 },
                point_b: Point { x: 8, y: 7 },
            },
            door_type: DoorType::Horizontal,
        };

        assert!(half_door_vertical.close_door_step());
        assert!(half_door_horizontal.close_door_step());
    }

    #[test]
    fn open_door_step() {
        let vertical_point_a_1 = Point { x: 4, y: 6 };
        let vertical_point_a_2 = Point {
            x: vertical_point_a_1.x,
            y: vertical_point_a_1.y - 1,
        };
        let horizontal_point_a_1 = Point { x: 7, y: 1 };
        let horizontal_point_a_2 = Point {
            x: horizontal_point_a_1.x - 1,
            y: horizontal_point_a_1.y,
        };

        let point_b = Point { x: 3, y: 2 };

        let mut half_door_vertical = HalfDoor {
            start_point: Point { x: 4, y: 8 },
            end_point: Point { x: 3, y: 2 },
            rectangle: Rectangle {
                point_a: vertical_point_a_1,
                point_b: point_b.clone(),
            },
            door_type: DoorType::Vertical,
        };
        let mut half_door_horizontal = HalfDoor {
            start_point: Point { x: 9, y: 1 },
            end_point: Point { x: 3, y: 2 },
            rectangle: Rectangle {
                point_a: Point { x: 7, y: 1 },
                point_b: point_b.clone(),
            },
            door_type: DoorType::Horizontal,
        };

        assert!(!half_door_vertical.open_door_step());
        assert!(!half_door_horizontal.open_door_step());

        assert_eq!(vertical_point_a_2, half_door_vertical.rectangle.point_a);
        assert_eq!(point_b, half_door_vertical.rectangle.point_b);

        assert_eq!(horizontal_point_a_2, half_door_horizontal.rectangle.point_a);
        assert_eq!(point_b, half_door_horizontal.rectangle.point_b);
    }

    #[test]
    fn close_door_step() {
        let vertical_point_a_1 = Point { x: 4, y: 6 };
        let vertical_point_a_2 = Point {
            x: vertical_point_a_1.x,
            y: vertical_point_a_1.y + 1,
        };
        let horizontal_point_a_1 = Point { x: 7, y: 1 };
        let horizontal_point_a_2 = Point {
            x: horizontal_point_a_1.x + 1,
            y: horizontal_point_a_1.y,
        };

        let point_b = Point { x: 3, y: 2 };

        let mut half_door_vertical = HalfDoor {
            start_point: Point { x: 4, y: 8 },
            end_point: Point { x: 3, y: 2 },
            rectangle: Rectangle {
                point_a: vertical_point_a_1,
                point_b: point_b.clone(),
            },
            door_type: DoorType::Vertical,
        };
        let mut half_door_horizontal = HalfDoor {
            start_point: Point { x: 9, y: 1 },
            end_point: Point { x: 3, y: 2 },
            rectangle: Rectangle {
                point_a: Point { x: 7, y: 1 },
                point_b: point_b.clone(),
            },
            door_type: DoorType::Horizontal,
        };

        assert!(!half_door_vertical.close_door_step());
        assert!(!half_door_horizontal.close_door_step());

        assert_eq!(vertical_point_a_2, half_door_vertical.rectangle.point_a);
        assert_eq!(point_b, half_door_vertical.rectangle.point_b);

        assert_eq!(horizontal_point_a_2, half_door_horizontal.rectangle.point_a);
        assert_eq!(point_b, half_door_horizontal.rectangle.point_b);
    }
}
