use super::radians::Radians;

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
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Angle {
    pub start: Radians,
    pub end: Radians,
}

impl Angle {
    pub(crate) fn value(&self) -> Radians {
        self.end - self.start
    }

    pub(crate) fn rotate_left(&mut self, angle_delta: Radians) {
        self.start += angle_delta;
        self.end += angle_delta;
    }

    pub(crate) fn rotate_right(&mut self, angle_delta: Radians) {
        self.start -= angle_delta;
        self.end -= angle_delta;
    }

    pub(crate) fn get_rays_angle_range(
        &self,
        number_of_rays: usize,
        index: usize,     // 0, 1, 2...
        all_index: usize, // 1, 2, 3...
    ) -> std::vec::Vec<std::ops::Range<usize>> {
        let start = Radians::new(
            self.start.to_f64() + (self.value().to_f64() / all_index as f64) * index as f64,
        ) - Radians::new(0.02);
        let end = Radians::new(
            self.start.to_f64() + (self.value().to_f64() / all_index as f64) * (index + 1) as f64,
        ) + Radians::new(0.02);

        // let start = self.start - Radians::new(0.02);
        // let end = self.end + Radians::new(0.02);
        if start > end {
            return vec![
                std::ops::Range {
                    start: start.into_rays_index(number_of_rays).floor() as usize,
                    end: number_of_rays - 1,
                },
                std::ops::Range {
                    start: 0,
                    end: end.into_rays_index(number_of_rays).ceil() as usize,
                },
            ];
        }
        vec![std::ops::Range {
            start: start.into_rays_index(number_of_rays).floor() as usize,
            end: end.into_rays_index(number_of_rays).ceil() as usize,
        }]
    }

    pub(crate) fn is_inside(&self, radians: Radians) -> bool {
        if self.start > self.end {
            if radians >= self.start || radians <= self.end {
                return true;
            }
            return false;
        }
        if radians >= self.start && radians <= self.end {
            return true;
        }
        return false;
    }

    pub(crate) fn get_direction(&self) -> Radians {
        self.start + self.value() / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player_utils::radians::PI_2;

    #[test]
    fn angle_value() {
        let start_angle = 5.5;
        let end_angle = 0.4;
        let angle_1 = Angle {
            start: Radians::new(start_angle),
            end: Radians::new(end_angle),
        };

        assert_eq!(
            angle_1.value(),
            Radians::new(end_angle - start_angle + PI_2)
        );
    }

    #[test]
    fn angle_rotate() {
        let delta = 2.0;

        let start_angle = 4.0;
        let end_angle = 5.5;
        let mut angle = Angle {
            start: Radians::new(start_angle),
            end: Radians::new(end_angle),
        };
        assert_eq!(angle.start, Radians::new(start_angle));
        assert_eq!(angle.end, Radians::new(end_angle));

        angle.rotate_left(Radians::new(delta));
        assert_eq!(angle.start, Radians::new(start_angle + delta));
        assert_eq!(angle.end, Radians::new(end_angle + delta - PI_2));

        angle.rotate_left(Radians::new(delta));
        assert_eq!(
            angle.start,
            Radians::new(start_angle + delta + delta - PI_2)
        );
        assert_eq!(angle.end, Radians::new(end_angle + delta + delta - PI_2));

        angle.rotate_right(Radians::new(delta));
        assert_eq!(angle.start, Radians::new(start_angle + delta));
        assert_eq!(angle.end, Radians::new(end_angle + delta - PI_2));
    }

    #[test]
    fn get_rays_angle_1_range() {
        let start_angle = 5.1;
        let end_angle = 5.5;
        let number_of_rays = 100;
        let angle = Angle {
            start: Radians::new(start_angle),
            end: Radians::new(end_angle),
        };
        let ranges = angle.get_rays_angle_range(number_of_rays, 0, 1);

        assert_eq!(ranges.len(), 1);
        assert_eq!(
            ranges[0],
            ((start_angle - 0.02) * number_of_rays as f64 / PI_2).floor() as usize
                ..((end_angle + 0.02) * number_of_rays as f64 / PI_2).ceil() as usize
        );
    }

    #[test]
    fn get_rays_angle_2_ranges() {
        let start_angle = 5.1;
        let end_angle = 0.5;
        let number_of_rays = 100;
        let angle = Angle {
            start: Radians::new(start_angle),
            end: Radians::new(end_angle),
        };
        let ranges = angle.get_rays_angle_range(number_of_rays, 0, 1);

        assert_eq!(ranges.len(), 2);
        assert_eq!(
            ranges[0],
            ((start_angle - 0.02) * number_of_rays as f64 / PI_2).floor() as usize
                ..number_of_rays - 1
        );
        assert_eq!(
            ranges[1],
            0..((end_angle + 0.02) * number_of_rays as f64 / PI_2).ceil() as usize
        );
    }

    #[test]
    fn is_inside() {
        let start_angle = 5.2;
        let end_angle = 2.3;
        let angle = Angle {
            start: Radians::new(start_angle),
            end: Radians::new(end_angle),
        };

        assert!(!angle.is_inside(Radians::new(4.5)));
        assert!(angle.is_inside(Radians::new(5.2)));
        assert!(angle.is_inside(Radians::new(5.8)));
        assert!(angle.is_inside(Radians::new(0.4)));
        assert!(angle.is_inside(Radians::new(2.3)));
        assert!(!angle.is_inside(Radians::new(2.5)));
    }
}
