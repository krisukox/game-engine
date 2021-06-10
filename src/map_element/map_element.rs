use crate::graph::Coordinate;
use crate::map_element::{Color, Point};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait MapElement: Send + Sync {
    fn is_point_in_object(&self, point: &Point) -> bool;

    fn color(&self) -> Color;

    fn update(&mut self, _time_elapsed: f64) {}

    fn on_position_update(&mut self, _coordinate: &Coordinate) {}
}
