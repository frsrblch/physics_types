use crate::{Angle, Vector3};
use std::ops::Mul;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Spherical<T> {
    pub magnitude: T,
    pub phi: Angle,
    pub theta: Angle,
}

impl<T> Spherical<T> {
    #[inline]
    pub fn unit_vector(self) -> Vector3<f64> {
        Vector3 {
            x: self.theta.cos() * self.phi.sin(),
            y: self.theta.sin() * self.phi.sin(),
            z: self.phi.cos(),
        }
    }
}

impl<T: Mul<f64, Output = T> + Copy> Spherical<T> {
    #[inline]
    pub fn vector(self) -> Vector3<T> {
        Vector3 {
            x: self.magnitude * self.theta.cos() * self.phi.sin(),
            y: self.magnitude * self.theta.sin() * self.phi.sin(),
            z: self.magnitude * self.phi.cos(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Length;

    #[test]
    #[allow(unused_variables)]
    fn unit_and_vector() {
        let spherical = Spherical {
            magnitude: Length::in_m(2.0),
            phi: Angle::PI / 6.0,
            theta: Angle::PI / 2.0,
        };

        let unit_vector = spherical.unit_vector();
        let vector = spherical.vector();

        // panic!("\n{:#?}\n{:#?}", unit_vector, vector);
    }
}
