mod door;
mod half_door;
mod map_element;
mod point;
mod rectangle;

use half_door::HalfDoor;
// use rectangle::Area;

pub use door::{Door, DoorType, DoorVelocity};
pub use map_element::MapElement;
pub use point::Point;
pub use rectangle::Rectangle;

cfg_if::cfg_if! {
    if #[cfg(test)]{
    pub use half_door::MockHalfDoor;
    pub use map_element::MockMapElement;
    pub use rectangle::MockRectangle;
    }
}
