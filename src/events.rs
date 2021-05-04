use piston::{event_loop, Event, Window};

pub struct Events(event_loop::Events);

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
impl Events {
    #[cfg(not(test))]
    pub fn new() -> Self {
        Self(event_loop::Events::new(event_loop::EventSettings::new()))
    }

    // TODO consider to add two W types instead of two implementation of draw function
    #[cfg(not(test))]
    pub fn next_event<W>(&mut self, window: &mut W) -> Option<Event>
    where
        W: Window,
    {
        self.0.next(window)
    }
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn next_event<W: 'static>(&mut self, _window: &mut W) -> Option<Event> {
        None
    }
}
