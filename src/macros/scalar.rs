#[macro_export]
macro_rules! scalar {
    {
        struct $scalar:ident($base:ty)
    } => {
        #[repr(transparent)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct $scalar {
            pub value: $base,
        }

        impl $scalar {
            #[allow(dead_code)]
            #[inline]
            pub(super) const fn new(value: $base) -> Self {
                debug_assert!(value.is_finite());
                Self { value }
            }

            #[inline]
            pub const fn zero() -> Self {
                Self::new(0.0)
            }

            #[inline]
            pub fn request(&mut self, amount: Self) -> Self {
                debug_assert!(*self >= Self::zero());
                debug_assert!(amount >= Self::zero());

                let result = amount.min(*self);
                *self -= result;
                result
            }

            #[inline]
            pub fn give(&mut self, amount: &mut Self) {
                *self += *amount;
                *amount = Self::zero();
            }

            #[inline]
            pub fn is_none(&self) -> bool {
                *self == Self::zero()
            }

            #[inline]
            pub fn is_some(&self) -> bool {
                !self.is_none()
            }

            #[inline]
            pub fn mul_add_assign(&mut self, a: $base, b: Self) {
                *self = (*self * a) + b;
            }

            #[inline]
            pub const fn value(self) -> $base {
                self.value
            }

            #[inline]
            pub fn abs(self) -> Self {
                Self::new(self.value.abs())
            }
        }

        impl const $crate::Wrapper for $scalar {
            type Inner = $base;
            #[inline]
            fn value(self) -> $base {
                self.value
            }
        }

        impl PartialEq for $scalar {
            #[inline]
            fn eq(&self, rhs: &Self) -> bool {
                self.value == rhs.value
            }
        }

        impl Eq for $scalar {}

        impl PartialOrd for $scalar {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $scalar {
            #[inline]
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                if (self.value - other.value).abs() < <$base>::EPSILON {
                    std::cmp::Ordering::Equal
                } else if self.value < other.value {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        }

        impl From<$scalar> for $base {
            #[inline]
            fn from(value: $scalar) -> $base {
                value.value
            }
        }

        impl const std::ops::Add for $scalar {
            type Output = Self;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self::Output::new(self.value + rhs.value)
            }
        }

        impl const std::ops::Add<&$scalar> for $scalar {
            type Output = Self;
            #[inline]
            fn add(self, rhs: &Self) -> Self::Output {
                self + *rhs
            }
        }

        impl const std::ops::Add<$scalar> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn add(self, rhs: $scalar) -> Self::Output {
                *self + rhs
            }
        }

        impl const std::ops::Add for &$scalar {
            type Output = $scalar;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                *self + *rhs
            }
        }

        impl std::ops::AddAssign for $scalar {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        impl std::ops::AddAssign<&Self> for $scalar {
            #[inline]
            fn add_assign(&mut self, rhs: &Self) {
                self.value += rhs.value;
            }
        }

        impl const std::ops::Sub for $scalar {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output::new(self.value - rhs.value)
            }
        }

        impl const std::ops::Sub<&$scalar> for $scalar {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: &Self) -> Self::Output {
                self - *rhs
            }
        }

        impl const std::ops::Sub<$scalar> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn sub(self, rhs: $scalar) -> Self::Output {
                *self - rhs
            }
        }

        impl const std::ops::Sub for &$scalar {
            type Output = $scalar;
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                *self - *rhs
            }
        }

        impl std::ops::SubAssign for $scalar {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        impl std::ops::SubAssign<&Self> for $scalar {
            #[inline]
            fn sub_assign(&mut self, rhs: &Self) {
                self.value -= rhs.value;
            }
        }

        impl const std::ops::Mul<$base> for $scalar {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output::new(self.value * rhs)
            }
        }

        impl const std::ops::Mul<$base> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn mul(self, rhs: $base) -> Self::Output {
                *self * rhs
            }
        }

        impl const std::ops::Mul<&$base> for $scalar {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: &$base) -> Self::Output {
                self * *rhs
            }
        }

        impl const std::ops::Mul<&$base> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn mul(self, rhs: &$base) -> Self::Output {
                *self * *rhs
            }
        }

        impl const std::ops::Mul<$scalar> for $base {
            type Output = $scalar;
            #[inline]
            fn mul(self, rhs: $scalar) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<&$scalar> for $base {
            type Output = $scalar;
            #[inline]
            fn mul(self, rhs: &$scalar) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<$scalar> for &$base {
            type Output = $scalar;
            #[inline]
            fn mul(self, rhs: $scalar) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<&$scalar> for &$base {
            type Output = $scalar;
            #[inline]
            fn mul(self, rhs: &$scalar) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::MulAssign<$base> for $scalar {
            #[inline]
            fn mul_assign(&mut self, rhs: $base) {
                self.value *= rhs;
            }
        }

        impl std::ops::MulAssign<&$base> for $scalar {
            #[inline]
            fn mul_assign(&mut self, rhs: &$base) {
                self.value *= rhs;
            }
        }

        impl const std::ops::Div<$base> for $scalar {
            type Output = Self;
            #[inline]
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output::new(self.value / rhs)
            }
        }

        impl const std::ops::Div<$base> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn div(self, rhs: $base) -> Self::Output {
                *self / rhs
            }
        }

        impl const std::ops::Div<&$base> for $scalar {
            type Output = Self;
            #[inline]
            fn div(self, rhs: &$base) -> Self::Output {
                self / *rhs
            }
        }

        impl const std::ops::Div<&$base> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn div(self, rhs: &$base) -> Self::Output {
                *self / *rhs
            }
        }

        impl std::ops::DivAssign<$base> for $scalar {
            #[inline]
            fn div_assign(&mut self, rhs: $base) {
                self.value /= rhs;
            }
        }

        impl std::ops::DivAssign<&$base> for $scalar {
            #[inline]
            fn div_assign(&mut self, rhs: &$base) {
                self.value /= rhs;
            }
        }

        impl const std::ops::Div for $scalar {
            type Output = $base;
            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl const std::ops::Div<&$scalar> for $scalar {
            type Output = $base;
            #[inline]
            fn div(self, rhs: &Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl const std::ops::Div<$scalar> for &$scalar {
            type Output = $base;
            #[inline]
            fn div(self, rhs: $scalar) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl const std::ops::Div for &$scalar {
            type Output = $base;
            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl const std::ops::Neg for $scalar {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self::Output {
                Self::Output::new(-self.value)
            }
        }

        impl const std::ops::Neg for &$scalar {
            type Output = $scalar;
            #[inline]
            fn neg(self) -> Self::Output {
                -*self
            }
        }

        impl const std::ops::Rem for $scalar {
            type Output = $scalar;
            #[inline]
            fn rem(self, rhs: $scalar) -> Self::Output {
                Self::Output::new(self.value % rhs.value)
            }
        }

        impl const std::ops::Rem<&$scalar> for $scalar {
            type Output = $scalar;
            #[inline]
            fn rem(self, rhs: &$scalar) -> Self::Output {
                self % *rhs
            }
        }

        impl const std::ops::Rem<$scalar> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn rem(self, rhs: $scalar) -> Self::Output {
                *self % rhs
            }
        }

        impl const std::ops::Rem<&$scalar> for &$scalar {
            type Output = $scalar;
            #[inline]
            fn rem(self, rhs: &$scalar) -> Self::Output {
                *self % *rhs
            }
        }

        impl std::iter::Sum<$scalar> for $scalar {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self::new(iter.map(|v| v.value).sum())
            }
        }

        impl<'a> std::iter::Sum<&'a Self> for $scalar {
            #[inline]
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.copied().sum()
            }
        }
    };
    {
        struct $scalar:ident($base:ty) {
            fn $in_unit:ident($unit:ident) -> Self;
        }
    } => {
        scalar!(struct $scalar($base));

        impl $scalar {
            #[inline]
            pub const fn $in_unit($unit: $base) -> Self {
                Self::new($unit)
            }
        }
    };
}
