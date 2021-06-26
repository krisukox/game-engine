use super::LinearGraph;
use std::iter::Iterator;

#[cfg(test)]
use mockall::mock;

pub struct RaysIterator<'a> {
    rays: &'a Vec<LinearGraph>,
    current: usize,
    end: usize,
}

impl RaysIterator<'_> {
    pub fn new(rays: &Vec<LinearGraph>, start: usize, end: usize) -> RaysIterator<'_> {
        RaysIterator {
            rays,
            current: if start < rays.len() { start } else { 0 },
            end: if end < rays.len() {
                end
            } else {
                rays.len() - 1
            },
        }
    }
}

impl<'a> Iterator for RaysIterator<'a> {
    type Item = &'a LinearGraph;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else if self.current >= self.rays.len() {
            self.current = 0;
        }
        let ray = &self.rays[self.current];
        self.current += 1;
        return Some(ray);
    }
}

#[cfg(test)]
mock! {
    pub RaysIterator {}
    impl Iterator for RaysIterator {
        type Item = &'static crate::graph::MockLinearGraph;

        fn next(&mut self) -> Option<&'static crate::graph::MockLinearGraph>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::LinearGraph;
    use crate::player_utils::Radians;

    fn get_rays(amount: usize) -> Vec<LinearGraph> {
        let mut rays = Vec::with_capacity(amount);
        for _ in 0..amount {
            rays.push(LinearGraph::from_radians(Radians::PI));
        }
        rays
    }

    #[test]
    fn next() {
        let rays = get_rays(10);

        let rays_iterator = RaysIterator::new(&rays, 2, 7);
        for (ray_expected, ray_ret) in rays_iterator.zip(rays[2..7].iter()) {
            assert!(std::ptr::eq(ray_expected, ray_ret));
        }

        let rays_iterator = RaysIterator::new(&rays, 7, 2);
        for (ray_expected, ray_ret) in rays_iterator.zip(rays[7..].iter().chain(rays[..2].iter())) {
            assert!(std::ptr::eq(ray_expected, ray_ret));
        }
    }

    #[test]
    fn out_of_range_next() {
        let rays = get_rays(10);

        let rays_iterator = RaysIterator::new(&rays, 10, 12);
        for (ray_expected, ray_ret) in rays_iterator.zip(rays.iter()) {
            assert!(std::ptr::eq(ray_expected, ray_ret));
        }
    }
}
