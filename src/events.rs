use piston;
use piston::event_loop;
use piston::event_loop::EventSettings;
use piston::Window;

pub struct Events(event_loop::Events);

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
impl Events {
    pub fn new() -> Self {
        Self(event_loop::Events::new(EventSettings::new()))
    }

    // TODO consider to add two W types instead of two implementation of draw function
    #[cfg(not(test))]
    pub fn next_event<W>(&mut self, window: &mut W) -> Option<piston::Event>
    where
        W: Window,
    {
        self.0.next(window)
    }
    #[cfg(test)]
    pub fn next_event<W: 'static>(&mut self, window: &mut W) -> Option<piston::Event>
    where
        W: Window,
    {
        self.0.next(window)
    }
}