use std::ops::{Add, AddAssign, Div, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Radians(pub f64); // Radians range [0, pi*2)
                             // TODO consider to add a constructor that will be checking if the value is in the range

pub const PI_2: f64 = std::f64::consts::PI * 2.0;

fn fix_radians(radians: f64) -> f64 {
    if radians < 0.0 {
        return radians + PI_2;
    } else if radians > PI_2 {
        return radians - PI_2;
    } else {
        return radians;
    }
}

impl AddAssign for Radians {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = fix_radians(self.0 + rhs.0);
    }
}

impl Add for Radians {
    type Output = Self;
    fn add(self, rhs: Radians) -> Self {
        Radians(fix_radians(self.0 + rhs.0))
    }
}

impl Sub for Radians {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Radians(fix_radians(self.0 - rhs.0))
    }
}

impl Div for Radians {
    type Output = f64;
    fn div(self, rhs: Self) -> f64 {
        self.0 / rhs.0
    }
}

impl Radians {
    pub const PI: Radians = Radians(std::f64::consts::PI);
    pub const PI_2: Radians = Radians(std::f64::consts::PI * 2.0);

    pub fn into_rays_index(&self, number_of_rays: usize) -> f64 {
        number_of_rays as f64 / PI_2 * self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_assign() {
        let radians_value = 2.5;
        let radians_delta = 3.0;
        let mut radians = Radians(radians_value);
        radians += Radians(radians_delta);
        assert_eq!(radians, Radians(radians_value + radians_delta));
        radians += Radians(radians_delta);
        assert_eq!(
            radians,
            Radians(radians_value + radians_delta + radians_delta - PI_2)
        );
        radians += Radians(-radians_delta);
        assert_eq!(radians, Radians(radians_value + radians_delta));
        radians += Radians(-radians_delta);
        assert_eq!(radians, Radians(radians_value));
    }

    #[test]
    fn add_sub() {
        let radians_value = 2.5;
        assert_eq!(
            Radians(radians_value) + Radians(radians_value),
            Radians(radians_value + radians_value)
        );
        assert_eq!(
            Radians(radians_value) + Radians(radians_value) + Radians(radians_value),
            Radians(radians_value + radians_value + radians_value - PI_2)
        );
        assert_eq!(
            Radians(radians_value) + Radians(radians_value) + Radians(radians_value)
                - Radians(radians_value),
            Radians(radians_value + radians_value)
        );
        assert_eq!(
            Radians(radians_value) + Radians(radians_value) + Radians(radians_value)
                - Radians(radians_value)
                - Radians(radians_value),
            Radians(radians_value)
        );
    }

    #[test]
    fn div() {
        let radians_1 = 2.5;
        let radians_2 = 0.5;
        assert_eq!(
            Radians(radians_1) / Radians(radians_2),
            radians_1 / radians_2
        );
    }

    #[test]
    fn into_rays_index() {
        let radians_value_1 = std::f64::consts::PI;
        let radians_value_2 = std::f64::consts::PI * 3.0 / 2.0;
        let number_of_rays = 100;

        assert_eq!(
            Radians(radians_value_1).into_rays_index(number_of_rays),
            number_of_rays as f64 / 2.0
        );
        assert_eq!(
            Radians(radians_value_2).into_rays_index(number_of_rays),
            number_of_rays as f64 * 3.0 / 4.0
        );
    }
}
