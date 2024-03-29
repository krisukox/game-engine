mod color;
mod door;
mod half_door;
mod map_element;
mod point;
mod rectangle;
mod wall_map;

pub use color::Color;
pub use door::{Door, DoorType, DoorVelocity};
pub use map_element::MapElement;
pub use point::Point;
pub use rectangle::Rectangle;
pub use wall_map::WallMap;

cfg_if::cfg_if! {
    if #[cfg(test)]{
        pub use map_element::MockMapElement;
        pub use rectangle::MockRectangle;
    }
}
