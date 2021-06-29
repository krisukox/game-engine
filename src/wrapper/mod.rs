mod events_wrapper;
mod graphics_wrapper;
#[cfg(test)]
pub mod test_utils;

pub use events_wrapper::Events;
pub use graphics_wrapper::Graphics;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use graphics_wrapper::MockGraphics;
        pub use events_wrapper::MockEvents;
    }
}
