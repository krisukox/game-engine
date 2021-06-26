use super::LinearGraph;
use crate::player_utils::Angle;

#[cfg(test)]
use mockall::automock;

pub struct Rays(pub Vec<LinearGraph>);

#[cfg_attr(test, automock)]
impl Rays {
    #[cfg(not(test))]
    pub fn iter<'a>(
        &'a self,
        angle: &Angle,
        index: usize,     // 0, 1, 2...
        all_index: usize, // 1, 2, 3...
    ) -> impl Iterator<Item = &'a super::LinearGraph> {
        let (start, end) = angle.get_rays_angle_range(self.0.len(), index, all_index);
        return super::RaysIterator::new(&self.0, start, end);
    }
    #[cfg(test)]
    pub fn iter(
        &self,
        _angle: &Angle,
        _index: usize,
        _all_index: usize,
    ) -> impl Iterator<Item = &'static crate::graph::MockLinearGraph> {
        return super::MockRaysIterator::default();
    }
}
