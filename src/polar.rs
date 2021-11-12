use crate::angle::Angle;
use crate::vector::Vector2;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Polar<T> {
    pub magnitude: T,
    pub angle: Angle,
}

impl<T: Mul<f64, Output = T> + Copy> Polar<T> {
    #[inline]
    pub fn euclidean(self) -> Vector2<T> {
        Vector2::from_angle_and_magnitude(self.angle, self.magnitude)
    }
}

impl<T: Mul<Rhs>, Rhs> Mul<Rhs> for Polar<T> {
    type Output = Polar<T::Output>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Polar {
            magnitude: self.magnitude.mul(rhs),
            angle: self.angle,
        }
    }
}

impl<T> Mul<Polar<T>> for f64
where
    f64: Mul<T>,
{
    type Output = Polar<<f64 as Mul<T>>::Output>;

    fn mul(self, rhs: Polar<T>) -> Self::Output {
        Polar {
            magnitude: self.mul(rhs.magnitude),
            angle: rhs.angle,
        }
    }
}

impl<T> Mul<Polar<T>> for &f64
where
    f64: Mul<T>,
{
    type Output = Polar<<f64 as Mul<T>>::Output>;

    fn mul(self, rhs: Polar<T>) -> Self::Output {
        Polar {
            magnitude: (*self).mul(rhs.magnitude),
            angle: rhs.angle,
        }
    }
}

impl<T: MulAssign<Rhs>, Rhs> MulAssign<Rhs> for Polar<T> {
    fn mul_assign(&mut self, rhs: Rhs) {
        self.magnitude.mul_assign(rhs);
    }
}

impl<T: Div<Rhs>, Rhs> Div<Rhs> for Polar<T> {
    type Output = Polar<T::Output>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Polar {
            magnitude: self.magnitude.div(rhs),
            angle: self.angle,
        }
    }
}

impl<T: DivAssign<Rhs>, Rhs> DivAssign<Rhs> for Polar<T> {
    fn div_assign(&mut self, rhs: Rhs) {
        self.magnitude.div_assign(rhs);
    }
}

impl<T: Add<U>, U> Add<Polar<U>> for Polar<T> {
    type Output = Polar<T::Output>;

    fn add(self, rhs: Polar<U>) -> Self::Output {
        Polar {
            magnitude: self.magnitude + rhs.magnitude,
            angle: self.angle + rhs.angle,
        }
    }
}

impl<T: Sub<U>, U> Sub<Polar<U>> for Polar<T> {
    type Output = Polar<T::Output>;

    fn sub(self, rhs: Polar<U>) -> Self::Output {
        Polar {
            magnitude: self.magnitude - rhs.magnitude,
            angle: self.angle - rhs.angle,
        }
    }
}
