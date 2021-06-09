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
    pub fn get_half_doors(door_area: &Rectangle, door_type: &DoorType) -> (HalfDoor, HalfDoor) {
        let start_point_1: Point;
        let end_point_1: Point;
        let start_point_2: Point;
        let end_point_2: Point;
        if *door_type == DoorType::Vertical {
            start_point_1 = Point {
                x: door_area.point_b.x,
                y: if (door_area.point_a.y - door_area.point_b.y) % 2 == 0 {
                    (door_area.point_a.y + door_area.point_b.y) / 2
                } else {
                    if door_area.point_a.y > door_area.point_b.y {
                        ((door_area.point_a.y + door_area.point_b.y) as f64 / 2.0).floor() as i64
                    } else {
                        ((door_area.point_a.y + door_area.point_b.y) as f64 / 2.0).ceil() as i64
                    }
                },
            };
            end_point_1 = door_area.point_a.clone();

            start_point_2 = Point {
                x: door_area.point_a.x,
                y: if (door_area.point_a.y - door_area.point_b.y) % 2 == 0 {
                    (door_area.point_a.y + door_area.point_b.y) / 2
                } else {
                    if door_area.point_a.y > door_area.point_b.y {
                        ((door_area.point_a.y + door_area.point_b.y) as f64 / 2.0).ceil() as i64
                    } else {
                        ((door_area.point_a.y + door_area.point_b.y) as f64 / 2.0).floor() as i64
                    }
                },
            };
            end_point_2 = door_area.point_b.clone();
        } else {
            start_point_1 = Point {
                x: if (door_area.point_a.x - door_area.point_b.x) % 2 == 0 {
                    (door_area.point_a.x + door_area.point_b.x) / 2
                } else {
                    if door_area.point_a.x > door_area.point_b.x {
                        ((door_area.point_a.x + door_area.point_b.x) as f64 / 2.0).floor() as i64
                    } else {
                        ((door_area.point_a.x + door_area.point_b.x) as f64 / 2.0).ceil() as i64
                    }
                },
                y: door_area.point_b.y,
            };
            end_point_1 = door_area.point_a.clone();

            start_point_2 = Point {
                x: if (door_area.point_a.x - door_area.point_b.x) % 2 == 0 {
                    (door_area.point_a.x + door_area.point_b.x) / 2
                } else {
                    if door_area.point_a.x > door_area.point_b.x {
                        ((door_area.point_a.x + door_area.point_b.x) as f64 / 2.0).ceil() as i64
                    } else {
                        ((door_area.point_a.x + door_area.point_b.x) as f64 / 2.0).floor() as i64
                    }
                },
                y: door_area.point_a.y,
            };
            end_point_2 = door_area.point_b.clone();
        }

        return (
            HalfDoor {
                start_point: start_point_1.clone(),
                end_point: end_point_1.clone(),
                rectangle: Rectangle {
                    point_a: end_point_1,
                    point_b: start_point_1,
                },
                door_type: door_type.clone(),
            },
            HalfDoor {
                start_point: start_point_2.clone(),
                end_point: end_point_2.clone(),
                rectangle: Rectangle {
                    point_a: end_point_2,
                    point_b: start_point_2,
                },
                door_type: door_type.clone(),
            },
        );
    }

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

    #[cfg(not(tarpaulin_include))]
    #[allow(dead_code)]
    pub fn is_point_in_object(&self, point: &Point) -> bool {
        self.rectangle.is_point_in_object(point)
    }
}

impl PartialEq for HalfDoor {
    fn eq(&self, other: &Self) -> bool {
        if (self.rectangle.point_a == other.rectangle.point_a
            || self.rectangle.point_a == other.rectangle.point_b)
            && (self.rectangle.point_b == other.rectangle.point_a
                || self.rectangle.point_b == other.rectangle.point_b)
        {
            return self.start_point == other.start_point
                && self.end_point == other.end_point
                && self.door_type == other.door_type;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_half_doors_odd() {
        let expected_half_door_1 = HalfDoor {
            start_point: Point { x: 3, y: 4 },
            end_point: Point { x: 2, y: 1 },
            rectangle: Rectangle {
                point_a: Point { x: 3, y: 4 },
                point_b: Point { x: 2, y: 1 },
            },
            door_type: DoorType::Vertical,
        };
        let expected_half_door_2 = HalfDoor {
            start_point: Point { x: 2, y: 3 },
            end_point: Point { x: 3, y: 6 },
            rectangle: Rectangle {
                point_a: Point { x: 2, y: 3 },
                point_b: Point { x: 3, y: 6 },
            },
            door_type: DoorType::Vertical,
        };
        let (half_door_1, half_door_2) = HalfDoor::get_half_doors(
            &Rectangle {
                point_a: Point { x: 2, y: 1 },
                point_b: Point { x: 3, y: 6 },
            },
            &DoorType::Vertical,
        );
        assert!(half_door_1 == expected_half_door_1 || half_door_1 == expected_half_door_2);
        assert!(half_door_2 == expected_half_door_1 || half_door_2 == expected_half_door_2);
        let (half_door_1, half_door_2) = HalfDoor::get_half_doors(
            &Rectangle {
                point_a: Point { x: 3, y: 6 },
                point_b: Point { x: 2, y: 1 },
            },
            &DoorType::Vertical,
        );
        assert!(half_door_1 == expected_half_door_1 || half_door_1 == expected_half_door_2);
        assert!(half_door_2 == expected_half_door_1 || half_door_2 == expected_half_door_2);
        let expected_half_door_1 = HalfDoor {
            start_point: Point { x: 7, y: 2 },
            end_point: Point { x: 4, y: 1 },
            rectangle: Rectangle {
                point_a: Point { x: 7, y: 2 },
                point_b: Point { x: 4, y: 1 },
            },
            door_type: DoorType::Horizontal,
        };
        let expected_half_door_2 = HalfDoor {
            start_point: Point { x: 6, y: 1 },
            end_point: Point { x: 9, y: 2 },
            rectangle: Rectangle {
                point_a: Point { x: 6, y: 1 },
                point_b: Point { x: 9, y: 2 },
            },
            door_type: DoorType::Horizontal,
        };
        let (half_door_1, half_door_2) = HalfDoor::get_half_doors(
            &Rectangle {
                point_a: Point { x: 4, y: 1 },
                point_b: Point { x: 9, y: 2 },
            },
            &DoorType::Horizontal,
        );
        assert!(half_door_1 == expected_half_door_1 || half_door_1 == expected_half_door_2);
        assert!(half_door_2 == expected_half_door_1 || half_door_2 == expected_half_door_2);
        let (half_door_1, half_door_2) = HalfDoor::get_half_doors(
            &Rectangle {
                point_a: Point { x: 9, y: 2 },
                point_b: Point { x: 4, y: 1 },
            },
            &DoorType::Horizontal,
        );
        assert!(half_door_1 == expected_half_door_1 || half_door_1 == expected_half_door_2);
        assert!(half_door_2 == expected_half_door_1 || half_door_2 == expected_half_door_2);
    }

    #[test]
    fn get_half_doors_even() {
        let (half_door_1, half_door_2) = HalfDoor::get_half_doors(
            &Rectangle {
                point_a: Point { x: 2, y: 1 },
                point_b: Point { x: 3, y: 7 },
            },
            &DoorType::Vertical,
        );
        let expected_half_door_1 = HalfDoor {
            start_point: Point { x: 3, y: 4 },
            end_point: Point { x: 2, y: 1 },
            rectangle: Rectangle {
                point_a: Point { x: 3, y: 4 },
                point_b: Point { x: 2, y: 1 },
            },
            door_type: DoorType::Vertical,
        };
        let expected_half_door_2 = HalfDoor {
            start_point: Point { x: 2, y: 4 },
            end_point: Point { x: 3, y: 7 },
            rectangle: Rectangle {
                point_a: Point { x: 2, y: 4 },
                point_b: Point { x: 3, y: 7 },
            },
            door_type: DoorType::Vertical,
        };
        assert!(half_door_1 == expected_half_door_1 || half_door_1 == expected_half_door_2);
        assert!(half_door_2 == expected_half_door_1 || half_door_2 == expected_half_door_2);
        let (half_door_1, half_door_2) = HalfDoor::get_half_doors(
            &Rectangle {
                point_a: Point { x: 4, y: 1 },
                point_b: Point { x: 10, y: 2 },
            },
            &DoorType::Horizontal,
        );
        let expected_half_door_1 = HalfDoor {
            start_point: Point { x: 7, y: 2 },
            end_point: Point { x: 4, y: 1 },
            rectangle: Rectangle {
                point_a: Point { x: 7, y: 2 },
                point_b: Point { x: 4, y: 1 },
            },
            door_type: DoorType::Horizontal,
        };
        let expected_half_door_2 = HalfDoor {
            start_point: Point { x: 7, y: 1 },
            end_point: Point { x: 10, y: 2 },
            rectangle: Rectangle {
                point_a: Point { x: 7, y: 1 },
                point_b: Point { x: 10, y: 2 },
            },
            door_type: DoorType::Horizontal,
        };
        assert!(half_door_1 == expected_half_door_1 || half_door_1 == expected_half_door_2);
        assert!(half_door_2 == expected_half_door_1 || half_door_2 == expected_half_door_2);
    }

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
