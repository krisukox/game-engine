use crate::graph::{Coordinate, LinearGraph, Wall};
use crate::map_element::{Color, Point};
use mockall_double::double;

#[double]
use crate::graph::GraphMethods;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait MapElement: Send + Sync {
    fn is_point_in_object(&self, point: &Point) -> bool;

    fn color(&self) -> Color;

    fn update(&mut self, _time_elapsed: f64) {}

    fn on_position_update(&mut self, _coordinate: &Coordinate) {}

    fn is_coordinate_in_object(
        &self,
        coordinate: &Coordinate,
        start_position: &Coordinate,
    ) -> Option<(Wall, LinearGraph)> {
        if coordinate.x.fract() == 0.0 {
            if self.is_point_in_object(&Point {
                x: coordinate.x as i64,
                y: coordinate.y.floor() as i64,
            }) {
                return Some((
                    Wall {
                        start_point: Point {
                            x: coordinate.x as i64,
                            y: coordinate.y.floor() as i64,
                        },
                        end_point: Point {
                            x: coordinate.x as i64,
                            y: coordinate.y.ceil() as i64,
                        },
                        primary_object_color: self.color(),
                    },
                    GraphMethods::from_two_coordinates(
                        start_position,
                        Coordinate {
                            x: coordinate.x,
                            y: coordinate.y.ceil() + 0.0001,
                        },
                    ),
                ));
            }
            if coordinate.x >= 1.0
                && self.is_point_in_object(&Point {
                    x: coordinate.x as i64 - 1,
                    y: coordinate.y.floor() as i64,
                })
            {
                return Some((
                    Wall {
                        start_point: Point {
                            x: coordinate.x as i64,
                            y: coordinate.y.ceil() as i64,
                        },
                        end_point: Point {
                            x: coordinate.x as i64,
                            y: coordinate.y.floor() as i64,
                        },

                        primary_object_color: self.color(),
                    },
                    GraphMethods::from_two_coordinates(
                        start_position,
                        Coordinate {
                            x: coordinate.x,
                            y: coordinate.y.floor() - 0.0001,
                        },
                    ),
                ));
            }
        } else if coordinate.y.fract() == 0.0 {
            if self.is_point_in_object(&Point {
                x: coordinate.x.floor() as i64,
                y: coordinate.y as i64,
            }) {
                return Some((
                    Wall {
                        start_point: Point {
                            x: coordinate.x.ceil() as i64,
                            y: coordinate.y as i64,
                        },
                        end_point: Point {
                            x: coordinate.x.floor() as i64,
                            y: coordinate.y as i64,
                        },
                        primary_object_color: self.color(),
                    },
                    GraphMethods::from_two_coordinates(
                        start_position,
                        Coordinate {
                            x: coordinate.x.floor() - 0.0001,
                            y: coordinate.y,
                        },
                    ),
                ));
            }
            if coordinate.y >= 1.0
                && self.is_point_in_object(&Point {
                    x: coordinate.x.floor() as i64,
                    y: coordinate.y as i64 - 1,
                })
            {
                return Some((
                    Wall {
                        start_point: Point {
                            x: coordinate.x.floor() as i64,
                            y: coordinate.y as i64,
                        },
                        end_point: Point {
                            x: coordinate.x.ceil() as i64,
                            y: coordinate.y as i64,
                        },

                        primary_object_color: self.color(),
                    },
                    GraphMethods::from_two_coordinates(
                        start_position,
                        Coordinate {
                            x: coordinate.x.ceil() + 0.0001,
                            y: coordinate.y,
                        },
                    ),
                ));
            }
        }
        return None;
    }
}
