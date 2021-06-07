pub mod engine;
pub mod graph;
pub mod map_element;
pub mod player_utils;

pub use map::Map;
// pub use map_element::{Door, DoorType, MapElement, Point, Rectangle, WallMap};
pub use piston::window::Size;

mod events_wrapper;
mod graphics_wrapper;
mod map;
mod object_generator;
mod point_generator;
mod polygon_generator;

#[cfg(test)]
mod test_utils;
