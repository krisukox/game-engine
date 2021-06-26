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
    ) -> (usize, usize) {
        let start = Radians::new(
            self.start.to_f64() + (self.value().to_f64() / all_index as f64) * index as f64,
        ) - Radians::new(0.02);
        let end = Radians::new(
            self.start.to_f64() + (self.value().to_f64() / all_index as f64) * (index + 1) as f64,
        ) + Radians::new(0.02);

        (
            start.into_rays_index(number_of_rays).floor() as usize,
            end.into_rays_index(number_of_rays).ceil() as usize,
        )
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
        let number_of_rays = 1000;
        let angle = Angle {
            start: Radians::new(start_angle),
            end: Radians::new(end_angle),
        };
        let (ret_start, ret_end) = angle.get_rays_angle_range(number_of_rays, 0, 3);
        assert_eq!(
            ret_start,
            ((start_angle - 0.02) * number_of_rays as f64 / PI_2).floor() as usize
        );
        assert_eq!(
            ret_end,
            ((start_angle + angle.value().to_f64() * 1.0 / 3.0 + 0.02) * number_of_rays as f64
                / PI_2)
                .ceil() as usize
        );

        let (ret_start, ret_end) = angle.get_rays_angle_range(number_of_rays, 1, 3);
        assert_eq!(
            ret_start,
            ((start_angle + angle.value().to_f64() * 1.0 / 3.0 - 0.02) * number_of_rays as f64
                / PI_2)
                .floor() as usize
        );
        assert_eq!(
            ret_end,
            ((start_angle + angle.value().to_f64() * 2.0 / 3.0 + 0.02) * number_of_rays as f64
                / PI_2)
                .ceil() as usize
        );

        let (ret_start, ret_end) = angle.get_rays_angle_range(number_of_rays, 2, 3);
        assert_eq!(
            ret_start,
            ((start_angle + angle.value().to_f64() * 2.0 / 3.0 - 0.02) * number_of_rays as f64
                / PI_2)
                .floor() as usize
        );
        assert_eq!(
            ret_end,
            ((start_angle + angle.value().to_f64() * 3.0 / 3.0 + 0.02) * number_of_rays as f64
                / PI_2)
                .ceil() as usize
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
