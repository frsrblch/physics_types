use crate::Scalar;
use std::cmp::Ordering;
use std::ops::{Mul, Not};

#[derive(Debug, Default, Copy, Clone)]
pub struct UnitInterval(f64);

impl PartialOrd for UnitInterval {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnitInterval {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialEq for UnitInterval {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for UnitInterval {}

impl From<f64> for UnitInterval {
    #[inline]
    fn from(value: f64) -> Self {
        Self::clamp(value)
    }
}

impl From<UnitInterval> for f64 {
    #[inline]
    fn from(value: UnitInterval) -> Self {
        value.0
    }
}

impl Mul for UnitInterval {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl Mul<&Self> for UnitInterval {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        self.mul(*rhs)
    }
}

impl Not for UnitInterval {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(1.0 - self.0)
    }
}

impl UnitInterval {
    #[inline]
    pub fn clamp(value: f64) -> Self {
        Self(value.max(0.0).min(1.0))
    }

    #[inline]
    pub fn f64(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct UnitVector {
    x: f64,
    y: f64,
}

impl UnitVector {
    #[inline]
    pub fn new<T>(x: T, y: T) -> Option<Self>
    where
        T: Into<f64>,
    {
        let x = x.into();
        let y = y.into();

        debug_assert!(x.is_finite() && y.is_finite());

        if x == 0.0 && y == 0.0 {
            return None;
        }

        let magnitude = (x * x + y * y).sqrt();
        let x = x / magnitude;
        let y = y / magnitude;

        Some(Self { x, y })
    }

    #[inline]
    pub const fn new_unchecked(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> f64 {
        self.y
    }
}

impl<T> Mul<T> for UnitVector
where
    T: Copy + Scalar + Mul<f64, Output = T>,
{
    type Output = T::Vector;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        T::vector(rhs * self.x, rhs * self.y)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct UnitVector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl UnitVector3 {
    #[inline]
    pub fn new<T>(x: T, y: T, z: T) -> Option<Self>
    where
        T: Into<f64>,
    {
        let x = x.into();
        let y = y.into();
        let z = z.into();

        debug_assert!(x.is_finite() && y.is_finite() && z.is_finite());

        if x == 0.0 && y == 0.0 && z == 0.0 {
            return None;
        }

        let magnitude = (x * x + y * y + z * z).sqrt();
        let x = x / magnitude;
        let y = y / magnitude;
        let z = z / magnitude;

        Some(Self { x, y, z })
    }

    #[inline]
    pub const fn new_unchecked(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub const fn z(&self) -> f64 {
        self.z
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_interval_clamp() {
        assert_eq!(UnitInterval(0.0), UnitInterval::clamp(f64::NAN));
        assert_eq!(UnitInterval(0.0), UnitInterval::clamp(f64::NEG_INFINITY));
        assert_eq!(UnitInterval(0.0), UnitInterval::clamp(0.0));
        assert_eq!(UnitInterval(0.0), UnitInterval::clamp(-0.1));
        assert_eq!(UnitInterval(0.5), UnitInterval::clamp(0.5));
        assert_eq!(UnitInterval(1.0), UnitInterval::clamp(1.0));
        assert_eq!(UnitInterval(1.0), UnitInterval::clamp(1.1));
        assert_eq!(UnitInterval(1.0), UnitInterval::clamp(f64::INFINITY));
    }

    #[test]
    fn unit_vector_new() {
        assert_eq!(
            Some(UnitVector::new_unchecked(1.0, 0.0)),
            UnitVector::new(2.0, 0.0)
        );
        assert_eq!(
            Some(UnitVector::new_unchecked(
                1.0 / 2f64.sqrt(),
                1.0 / 2f64.sqrt()
            )),
            UnitVector::new(5.0, 5.0)
        );
    }
}
