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
    pub start: f64, // radians
    pub end: f64,   // radians
}

const PI_2: f64 = std::f64::consts::PI * 2.0;

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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn angle_value() {
        let start_angle = 0.1;
        let end_angle = 0.6;
        let angle_1 = Angle {
            start: start_angle,
            end: end_angle,
        };
        let angle_2 = Angle {
            start: end_angle,
            end: start_angle,
        };

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
        let mut angle = Angle {
            start: start_angle,
            end: end_angle,
        };
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
        let mut angle = Angle {
            start: start_angle,
            end: end_angle,
        };
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
}
