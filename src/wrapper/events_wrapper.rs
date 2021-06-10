use piston::{event_loop, Event};

cfg_if::cfg_if! {
    if #[cfg(test)]{
        use super::test_utils::Window;
    } else {
        use glutin_window::GlutinWindow as Window;
    }
}

pub struct Events(event_loop::Events);

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(not(tarpaulin_include))]
#[allow(dead_code)]
#[allow(unused_variables)]
#[cfg_attr(test, automock)]
impl Events {
    pub fn new() -> Self {
        Self(event_loop::Events::new(event_loop::EventSettings::new()))
    }

    pub fn next_event(&mut self, window: &mut Window) -> Option<Event> {
        #[cfg(not(test))]
        return self.0.next(window);
        #[cfg(test)]
        return None;
    }
}
