use crate::graph::Coordinate;
use crate::graph::Wall;
use crate::map_element::{Color, Point};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait MapElement: Send + Sync {
    fn is_point_in_object(&self, point: &Point) -> bool;

    fn color(&self) -> Color;

    fn update(&mut self, _time_elapsed: f64) {}

    fn on_position_update(&mut self, _coordinate: &Coordinate) {}

    fn is_coordinate_in_object(&self, coordinate: &Coordinate) -> Option<Wall> {
        if coordinate.x.fract() == 0.0 {
            if self.is_point_in_object(&Point {
                x: coordinate.x as i64,
                y: coordinate.y.floor() as i64,
            }) {
                return Some(Wall {
                    start_point: Point {
                        x: coordinate.x as i64,
                        y: coordinate.y.ceil() as i64,
                    },
                    end_point: Point {
                        x: coordinate.x as i64,
                        y: coordinate.y.floor() as i64,
                    },
                    primary_object_color: self.color(),
                });
            }
            if self.is_point_in_object(&Point {
                x: coordinate.x as i64 - 1,
                y: coordinate.y.floor() as i64,
            }) {
                return Some(Wall {
                    start_point: Point {
                        x: coordinate.x as i64,
                        y: coordinate.y.floor() as i64,
                    },
                    end_point: Point {
                        x: coordinate.x as i64,
                        y: coordinate.y.ceil() as i64,
                    },
                    primary_object_color: self.color(),
                });
            }
        } else if coordinate.y.fract() == 0.0 {
            if self.is_point_in_object(&Point {
                x: coordinate.x.floor() as i64,
                y: coordinate.y as i64,
            }) {
                return Some(Wall {
                    start_point: Point {
                        x: coordinate.x.floor() as i64,
                        y: coordinate.y as i64,
                    },
                    end_point: Point {
                        x: coordinate.x.ceil() as i64,
                        y: coordinate.y as i64,
                    },
                    primary_object_color: self.color(),
                });
            }
            if self.is_point_in_object(&Point {
                x: coordinate.x.floor() as i64,
                y: coordinate.y as i64 - 1,
            }) {
                return Some(Wall {
                    start_point: Point {
                        x: coordinate.x.ceil() as i64,
                        y: coordinate.y as i64,
                    },
                    end_point: Point {
                        x: coordinate.x.floor() as i64,
                        y: coordinate.y as i64,
                    },
                    primary_object_color: self.color(),
                });
            }
        }
        return None;
    }
}
