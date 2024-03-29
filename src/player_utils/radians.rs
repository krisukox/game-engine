use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Radians(f64); // Radians range [0, pi*2)

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

impl Radians {
    pub fn new(radians: f64) -> Radians {
        Radians(fix_radians(radians))
    }

    pub(crate) fn into_rays_index(&self, number_of_rays: usize) -> f64 {
        number_of_rays as f64 / PI_2 * self.0
    }

    pub(crate) fn tan(&self) -> f64 {
        self.0.tan()
    }

    pub fn to_f64(&self) -> f64 {
        self.0
    }

    pub const ZERO: Radians = Radians(0.0);
    pub const PI: Radians = Radians(std::f64::consts::PI);
    pub const PI_2: Radians = Radians(std::f64::consts::PI * 2.0);
    #[cfg(test)]
    pub const OUT_OF_RANGE: Radians = Radians(std::f64::consts::PI * 3.0);
}

impl AddAssign for Radians {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = fix_radians(self.0 + rhs.0);
    }
}

impl SubAssign for Radians {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = fix_radians(self.0 - rhs.0);
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

impl Sub<Radians> for &Radians {
    type Output = Radians;
    fn sub(self, rhs: Radians) -> Self::Output {
        Radians(fix_radians(self.0 - rhs.0))
    }
}

impl Div for Radians {
    type Output = Radians;
    fn div(self, rhs: Self) -> Self::Output {
        Radians(self.0 / rhs.0)
    }
}

impl Div<f64> for Radians {
    type Output = Radians;
    fn div(self, rhs: f64) -> Self::Output {
        Radians(self.0 / rhs)
    }
}

impl PartialOrd for Radians {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.to_f64() == other.to_f64() {
            return Some(std::cmp::Ordering::Equal);
        }
        if (self.to_f64() - other.to_f64()).abs() > std::f64::consts::PI {
            if self.to_f64() > other.to_f64() {
                return Some(std::cmp::Ordering::Less);
            }
            return Some(std::cmp::Ordering::Greater);
        }
        if self.to_f64() < other.to_f64() {
            return Some(std::cmp::Ordering::Less);
        }
        return Some(std::cmp::Ordering::Greater);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub_assign() {
        let radians_value = 2.5;
        let radians_delta = 3.0;
        let mut radians = Radians::new(radians_value);
        radians += Radians::new(radians_delta);
        assert_eq!(radians, Radians(radians_value + radians_delta));
        radians += Radians::new(radians_delta);
        assert_eq!(
            radians,
            Radians(radians_value + radians_delta + radians_delta - PI_2)
        );
        radians -= Radians::new(radians_delta);
        assert_eq!(radians, Radians(radians_value + radians_delta));
        radians -= Radians::new(radians_delta);
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
            Radians::new(radians_value) + Radians::new(radians_value) + Radians::new(radians_value)
                - Radians::new(radians_value)
                - Radians::new(radians_value),
            Radians(radians_value)
        );
    }

    #[test]
    fn div() {
        let radians_1 = 2.5;
        let radians_2 = 0.5;
        assert_eq!(
            Radians::new(radians_1) / Radians::new(radians_2),
            Radians(radians_1 / radians_2)
        );

        assert_eq!(
            Radians::new(radians_1) / radians_2,
            Radians(radians_1 / radians_2)
        );
    }

    #[test]
    fn into_rays_index() {
        let radians_value_1 = std::f64::consts::PI;
        let radians_value_2 = std::f64::consts::PI * 3.0 / 2.0;
        let number_of_rays = 100;

        assert_eq!(
            Radians::new(radians_value_1).into_rays_index(number_of_rays),
            number_of_rays as f64 / 2.0
        );
        assert_eq!(
            Radians::new(radians_value_2).into_rays_index(number_of_rays),
            number_of_rays as f64 * 3.0 / 4.0
        );
    }

    #[test]
    fn partial_cmp() {
        let radians_1 = Radians(std::f64::consts::PI * 7.0 / 4.0);
        let radians_2 = Radians(std::f64::consts::PI / 4.0);
        let radians_3 = Radians(std::f64::consts::PI * 2.0 / 4.0);

        assert!(radians_1 < radians_2);
        assert!(radians_2 > radians_1);

        assert!(radians_2 < radians_3);
        assert!(radians_3 > radians_2);

        assert!(radians_1 == radians_1);
    }
}
