use piston;
use piston::event_loop;
use piston::Window;

pub struct Events(event_loop::Events);

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
impl Events {
    #[cfg(not(test))]
    pub fn next<W>(&mut self, window: &mut W) -> Option<piston::Event>
    where
        W: Window,
    {
        self.0.next(window)
    }
    #[cfg(test)]
    pub fn next<W: 'static>(&mut self, window: &mut W) -> Option<piston::Event>
    where
        W: Window,
    {
        self.0.next(window)
    }
}
