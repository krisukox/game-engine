use crate::graph::LinearGraph;
use std::rc::Rc;

const PI_2: f64 = std::f64::consts::PI * 2.0;

// Angle start-end direction:
// 1.reverse clocwise
// 2.same as the coordinate system
//
// example below means that Angle::value returns ~1.04 radians(60 degree)
// instead of ~5.23 radians(300 degree)
// (same as in player_get_angle_value test case)
//
//  end     start
//   \   |y  /
//    \  |  /
//     \ | /
// x____\|/_____
//       |
//       |
//       |
//       |
#[derive(Debug)]
pub struct Angle {
    pub start: f64,             // radians
    pub end: f64,               // radians
    rays: Rc<Vec<LinearGraph>>, // all rays around the player
    radians_as_rays_index: f64, // number of rays devided by 2 PI
}

fn rotate(angle: &mut f64, value: f64) {
    if *angle + value < 0.0 {
        *angle = *angle + value + PI_2;
    } else if *angle + value > PI_2 {
        *angle = *angle + value - PI_2;
    } else {
        *angle += value;
    }
}

impl Angle {
    pub fn new(start: f64, end: f64, number_of_rays: usize) -> Angle {
        Angle {
            start: start,
            end: end,
            rays: Rc::new(LinearGraph::get_all_rays(number_of_rays)),
            radians_as_rays_index: number_of_rays as f64 / PI_2,
        }
    }

    pub fn value(&self) -> f64 {
        if self.start < self.end {
            return self.end - self.start;
        } else {
            return PI_2 - self.start + self.end;
        }
    }

    pub fn rotate(&mut self, angle_delta: f64) {
        rotate(&mut self.start, angle_delta);
        rotate(&mut self.end, angle_delta);
    }

    pub fn get_rays_angle_range(&self) -> std::vec::Vec<std::ops::Range<usize>> {
        if self.start > self.end {
            return vec![
                std::ops::Range {
                    start: self.start_into_rays_index(),
                    end: self.rays.len() - 1,
                },
                std::ops::Range {
                    start: 0,
                    end: self.end_into_rays_index(),
                },
            ];
        }
        vec![std::ops::Range {
            start: self.start_into_rays_index(),
            end: self.end_into_rays_index(),
        }]
    }

    pub fn get_rays(&self) -> Rc<Vec<LinearGraph>> {
        return Rc::clone(&self.rays);
    }

    fn start_into_rays_index(&self) -> usize {
        (self.radians_as_rays_index * self.start).floor() as usize
    }

    fn end_into_rays_index(&self) -> usize {
        (self.radians_as_rays_index * self.end).ceil() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn angle_value() {
        let start_angle = 0.1;
        let end_angle = 0.6;
        let angle_1 = Angle::new(start_angle, end_angle, 100);
        let angle_2 = Angle::new(end_angle, start_angle, 100);

        assert_eq!(angle_1.value(), end_angle - start_angle);
        assert_ne!(angle_1.value(), PI_2 - end_angle + start_angle);

        assert_eq!(angle_2.value(), PI_2 - end_angle + start_angle);
        assert_ne!(angle_2.value(), end_angle - start_angle);
    }

    #[test]
    fn angle_rotate() {
        let positive_delta = 0.6;
        let negative_delta = -0.4;

        let mut start_angle = 3.2;
        let mut end_angle = 3.9;
        let mut angle = Angle::new(start_angle, end_angle, 100);
        assert_eq!(angle.start, start_angle);
        assert_eq!(angle.end, end_angle);

        angle.rotate(positive_delta);
        assert_eq!(angle.start, start_angle + positive_delta);
        assert_eq!(angle.end, end_angle + positive_delta);

        start_angle = angle.start;
        end_angle = angle.end;

        angle.rotate(negative_delta);
        assert_eq!(angle.start, start_angle + negative_delta);
        assert_eq!(angle.end, end_angle + negative_delta);
    }

    #[test]
    fn angle_rotate_out_of_range() {
        let positive_delta = 0.8;
        let negative_delta = -0.6;

        let mut start_angle = 5.1;
        let mut end_angle = 5.5;
        let mut angle = Angle::new(start_angle, end_angle, 100);
        assert_eq!(angle.start, start_angle);
        assert_eq!(angle.end, end_angle);

        angle.rotate(positive_delta);
        assert_eq!(angle.start, start_angle + positive_delta);
        assert_eq!(angle.end, end_angle + positive_delta - PI_2);

        start_angle = angle.start;
        end_angle = angle.end;

        angle.rotate(negative_delta);
        assert_eq!(angle.start, start_angle + negative_delta);
        assert_eq!(angle.end, end_angle + negative_delta + PI_2);
    }

    #[test]
    fn get_rays_angle_1_range() {
        let start_angle = 5.1;
        let end_angle = 5.5;
        let number_of_rays = 100;
        let angle = Angle::new(start_angle, end_angle, number_of_rays);
        let ranges = angle.get_rays_angle_range();

        assert_eq!(ranges.len(), 1);
        assert_eq!(
            ranges[0].start,
            (start_angle * number_of_rays as f64 / PI_2).floor() as usize
        );
        assert_eq!(
            ranges[0].end,
            (end_angle * number_of_rays as f64 / PI_2).ceil() as usize
        );
    }

    #[test]
    fn get_rays_angle_2_ranges() {
        let start_angle = 5.1;
        let end_angle = 0.5;
        let number_of_rays = 100;
        let angle = Angle::new(start_angle, end_angle, number_of_rays);
        let ranges = angle.get_rays_angle_range();

        assert_eq!(ranges.len(), 2);
        assert_eq!(
            ranges[0].start,
            (start_angle * number_of_rays as f64 / PI_2).floor() as usize
        );
        assert_eq!(ranges[0].end, number_of_rays - 1);
        assert_eq!(ranges[1].start, 0);
        assert_eq!(
            ranges[1].end,
            (end_angle * number_of_rays as f64 / PI_2).ceil() as usize
        );
    }
}
