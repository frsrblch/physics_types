use super::{Duration, Length};
use crate::constants::G;
use crate::time::Frequency;
use crate::Mass;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::f64::consts::{PI, TAU};

pub const DEG: Angle = Angle::in_deg(1.0);
pub const RAD: Angle = Angle::in_rad(1.0);

scalar! {
    struct Angle(f64) {
        fn in_rad(radians) -> Self;
    }
}

impl Angle {
    #[inline]
    pub const fn in_deg(degrees: f64) -> Self {
        Self::new(degrees * Self::RAD_PER_DEG)
    }

    #[inline]
    pub fn sin(self) -> f64 {
        self.value.sin()
    }

    #[inline]
    pub fn cos(self) -> f64 {
        self.value.cos()
    }

    #[inline]
    pub fn tan(self) -> f64 {
        self.value.tan()
    }

    #[inline]
    pub fn sin_cos(self) -> (f64, f64) {
        self.value.sin_cos()
    }

    const RAD_PER_DEG: f64 = PI / 180.0;

    pub const TAU: Self = Angle::in_rad(TAU);

    pub const PI: Self = Angle::in_rad(PI);

    #[inline]
    pub fn asin(value: f64) -> Angle {
        Angle::in_rad(value.asin())
    }

    #[inline]
    pub fn acos(value: f64) -> Angle {
        Angle::in_rad(value.acos())
    }

    #[inline]
    pub fn atan(value: f64) -> Angle {
        Angle::in_rad(value.atan())
    }
}

impl Distribution<Angle> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Angle {
        Angle::in_rad(rng.gen_range((-PI)..PI))
    }
}

scalar! {
    struct AngularSpeed(f64) {
        fn in_rad_per_s(rad_per_s) -> Self;
    }
}

impl AngularSpeed {
    #[inline]
    pub fn of_orbit(mass: Mass, radius: Length) -> Self {
        let r_cubed = radius.value * radius.value * radius.value;
        Self::new((G * mass.value / r_cubed).sqrt())
    }
}

scalar_div!(Angle | Duration = AngularSpeed);
scalar_div!(AngularSpeed | Frequency = Angle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_gen() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let angle: Angle = rng.gen();

            assert!(angle >= Angle::in_rad(-PI));
            assert!(angle < Angle::in_rad(PI))
        }
    }
}
