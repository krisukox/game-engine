use super::Point;
use crate::graph::Coordinate;
use crate::map_element::MapElement;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use super::half_door::MockHalfDoor as HalfDoor;
        use super::MockRectangle as Rectangle;
    } else {
        use super::HalfDoor;
        use super::Rectangle;
    }
}

pub struct Door {
    half_doors: (HalfDoor, HalfDoor),
    open_door_area: Rectangle,
    door_state: DoorState,
    door_velocity: f64,
    time_elapsed_ms: f64,
}

impl Door {
    #[cfg(not(tarpaulin_include))]
    #[cfg(not(test))]
    pub fn new(
        door_area: Rectangle,
        door_velocity: DoorVelocity,
        door_type: DoorType,
        open_door_area_opt: Option<Rectangle>,
    ) -> Self {
        Self {
            half_doors: Self::get_half_doors(&door_area, &door_type),
            open_door_area: Self::get_open_door_area(&door_area, open_door_area_opt, &door_type),
            door_state: DoorState::Closed,
            door_velocity: door_velocity.into(),
            time_elapsed_ms: 0.0,
        }
    }

    #[cfg(not(tarpaulin_include))]
    #[cfg(not(test))]
    fn get_half_doors(door_area: &Rectangle, door_type: &DoorType) -> (HalfDoor, HalfDoor) {
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

    #[cfg(not(tarpaulin_include))]
    #[cfg(not(test))]
    fn get_open_door_area(
        door_area: &Rectangle,
        open_door_area_opt: Option<Rectangle>,
        door_type: &DoorType,
    ) -> Rectangle {
        if let Some(open_door_area) = open_door_area_opt {
            return open_door_area;
        } else {
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
    }

    fn open_door_step(&mut self) -> bool {
        let ret1 = self.half_doors.0.open_door_step();
        let ret2 = self.half_doors.1.open_door_step();

        ret1 && ret2
    }

    fn close_door_step(&mut self) -> bool {
        let ret1 = self.half_doors.0.close_door_step();
        let ret2 = self.half_doors.1.close_door_step();

        ret1 && ret2
    }
}

impl MapElement for Door {
    fn is_point_in_object(&self, point: &Point) -> bool {
        self.half_doors.0.is_point_in_object(point) || self.half_doors.1.is_point_in_object(point)
    }

    fn update(&mut self, time_elapsed: f64) {
        if self.door_state == DoorState::Opening {
            self.time_elapsed_ms += time_elapsed * 1000.0;

            while self.time_elapsed_ms >= self.door_velocity {
                self.time_elapsed_ms -= self.door_velocity;
                if self.open_door_step() {
                    self.door_state = DoorState::Opened;
                    break;
                }
            }
        } else if self.door_state == DoorState::Closing {
            self.time_elapsed_ms += time_elapsed * 1000.0;

            while self.time_elapsed_ms >= self.door_velocity {
                self.time_elapsed_ms -= self.door_velocity;
                if self.close_door_step() {
                    self.door_state = DoorState::Closed;
                    break;
                }
            }
        }
    }

    fn on_position_update(&mut self, coordinate: &Coordinate) {
        if self.open_door_area.is_coordinate_in_object(&coordinate) {
            if self.door_state == DoorState::Closed || self.door_state == DoorState::Closing {
                self.door_state = DoorState::Opening;
            }
        } else {
            if self.door_state == DoorState::Opened || self.door_state == DoorState::Opening {
                self.door_state = DoorState::Closing;
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum DoorState {
    Closed,
    Opening,
    Opened,
    Closing,
}
pub enum DoorVelocity {
    VerySlow,
    Slow,
    Fast,
    VeryFast,
}

impl Into<f64> for DoorVelocity {
    fn into(self) -> f64 {
        match self {
            Self::VerySlow => 200.0,
            Self::Slow => 150.0,
            Self::Fast => 100.0,
            Self::VeryFast => 50.0,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum DoorType {
    Vertical,
    Horizontal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_element::MockHalfDoor;
    use crate::map_element::MockRectangle;
    use mockall::*;

    #[test]
    fn is_point_in_object() {
        let mut seq = Sequence::new();

        let mut half_door_1 = MockHalfDoor::new();
        let mut half_door_2 = MockHalfDoor::new();

        let point = Point { x: 10, y: 20 };

        let clone_point = point.clone();
        half_door_1
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point_| *point_ == clone_point)
            .return_const(false)
            .in_sequence(&mut seq);
        let clone_point = point.clone();
        half_door_2
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point_| *point_ == clone_point)
            .return_const(true)
            .in_sequence(&mut seq);
        let clone_point = point.clone();
        half_door_1
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point_| *point_ == clone_point)
            .return_const(false)
            .in_sequence(&mut seq);
        let clone_point = point.clone();
        half_door_2
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point_| *point_ == clone_point)
            .return_const(false)
            .in_sequence(&mut seq);

        let door = Door {
            half_doors: (half_door_1, half_door_2),
            open_door_area: Default::default(),
            door_state: DoorState::Closed,
            door_velocity: Default::default(),
            time_elapsed_ms: Default::default(),
        };
        assert!(door.is_point_in_object(&point));
        assert!(!door.is_point_in_object(&point));
    }

    #[test]
    fn update_opening() {
        let mut seq = Sequence::new();

        let mut half_door_1 = MockHalfDoor::new();
        let mut half_door_2 = MockHalfDoor::new();

        half_door_1
            .expect_open_door_step()
            .times(1)
            .return_const(false)
            .in_sequence(&mut seq);
        half_door_2
            .expect_open_door_step()
            .times(1)
            .return_const(false)
            .in_sequence(&mut seq);
        half_door_1
            .expect_open_door_step()
            .times(1)
            .return_const(true)
            .in_sequence(&mut seq);
        half_door_2
            .expect_open_door_step()
            .times(1)
            .return_const(true)
            .in_sequence(&mut seq);

        let mut door = Door {
            half_doors: (half_door_1, half_door_2),
            open_door_area: Default::default(),
            door_state: DoorState::Opening,
            door_velocity: 150.0,
            time_elapsed_ms: 0.0,
        };
        door.update(0.5);
        assert_eq!(door.door_state, DoorState::Opened)
    }

    #[test]
    fn update_closing() {
        let mut seq = Sequence::new();

        let mut half_door_1 = MockHalfDoor::new();
        let mut half_door_2 = MockHalfDoor::new();

        half_door_1
            .expect_close_door_step()
            .times(1)
            .return_const(false)
            .in_sequence(&mut seq);
        half_door_2
            .expect_close_door_step()
            .times(1)
            .return_const(false)
            .in_sequence(&mut seq);
        half_door_1
            .expect_close_door_step()
            .times(1)
            .return_const(true)
            .in_sequence(&mut seq);
        half_door_2
            .expect_close_door_step()
            .times(1)
            .return_const(true)
            .in_sequence(&mut seq);

        let mut door = Door {
            half_doors: (half_door_1, half_door_2),
            open_door_area: Default::default(),
            door_state: DoorState::Closing,
            door_velocity: 150.0,
            time_elapsed_ms: 0.0,
        };
        door.update(0.5);
        assert_eq!(door.door_state, DoorState::Closed)
    }

    fn check_on_position_update(
        door_state_start: DoorState,
        door_state_end: DoorState,
        is_coordinate_in_object: bool,
    ) {
        let mut seq = Sequence::new();

        let mut half_door_1 = MockHalfDoor::new();
        let mut half_door_2 = MockHalfDoor::new();
        let mut area = MockRectangle::new();

        let coordinate = Coordinate { x: 10.0, y: 20.0 };

        let coordinate_clone = coordinate.clone();
        area.expect_is_coordinate_in_object()
            .times(1)
            .withf(move |coordinate| *coordinate == coordinate_clone)
            .return_const(is_coordinate_in_object)
            .in_sequence(&mut seq);

        let mut door = Door {
            half_doors: (half_door_1, half_door_2),
            open_door_area: area,
            door_state: door_state_start,
            door_velocity: Default::default(),
            time_elapsed_ms: Default::default(),
        };

        door.on_position_update(&coordinate);
        assert_eq!(door.door_state, door_state_end);
    }

    #[test]
    fn on_position_update() {
        check_on_position_update(DoorState::Closed, DoorState::Opening, true);
        check_on_position_update(DoorState::Closing, DoorState::Opening, true);
        check_on_position_update(DoorState::Opened, DoorState::Closing, false);
        check_on_position_update(DoorState::Opening, DoorState::Closing, false);
    }
}
