#[macro_export]
macro_rules! vector3 {
    {
        struct $vector:ident([$scalar:ident; 3]) {
            fn $in_unit:ident($unit:ident: $base:ty) -> Self;
        }
    } => {
        #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $vector {
            pub x: $scalar,
            pub y: $scalar,
            pub z: $scalar,
        }

        impl $vector {
           #[inline]
            pub const fn $in_unit(x: $base, y: $base, z: $base) -> Self {
                Self {
                    x: $scalar::new(x),
                    y: $scalar::new(y),
                    z: $scalar::new(z),
                }
            }

           #[inline]
            pub fn magnitude(self) -> $scalar {
                $scalar::new(self.magnitude_squared_float().sqrt())
            }

           #[inline]
            fn magnitude_squared_float(self) -> $base {
                self.x.value * self.x.value + self.y.value * self.y.value + self.z.value * self.z.value
            }

           #[inline]
            pub fn unit_vector(self) -> Option<$crate::UnitVector3> {
                $crate::UnitVector3::new(self.x, self.y, self.z)
            }
        }

        impl const $crate::Scalar3 for $scalar {
            type Vector = $vector;
            fn vector3(x: Self, y: Self, z: Self) -> $vector {
                $vector{ x, y, z }
            }
        }

        impl const std::ops::Mul<$crate::UnitVector3> for $scalar {
            type Output = $vector;
            fn mul(self, rhs: $crate::UnitVector3) -> Self::Output {
                $crate::Scalar3::vector3(self * rhs.x(), self * rhs.y(), self * rhs.z())
            }
        }

        impl const std::ops::Add<$vector> for $vector {
            type Output = $vector;
            #[inline]
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl const std::ops::Add<$vector> for &$vector {
            type Output = $vector;
            #[inline]
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl const std::ops::Add<&$vector> for $vector {
            type Output = $vector;
            #[inline]
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z
                }
            }
        }

        impl const std::ops::Add<&$vector> for &$vector {
            type Output = $vector;
            #[inline]
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z
                }
            }
        }

        impl const std::ops::Sub<$vector> for $vector {
            type Output = $vector;
            #[inline]
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z
                }
            }
        }

        impl const std::ops::Sub<$vector> for &$vector {
            type Output = $vector;
            #[inline]
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z
                }
            }
        }

        impl const std::ops::Sub<&$vector> for $vector {
            type Output = $vector;
            #[inline]
            fn sub(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z
                }
            }
        }

        impl const std::ops::Sub<&$vector> for &$vector {
            type Output = $vector;
            #[inline]
            fn sub(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z
                }
            }
        }

        impl std::ops::AddAssign<$vector> for $vector {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl std::ops::AddAssign<&$vector> for $vector {
            #[inline]
            fn add_assign(&mut self, rhs: &$vector) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl std::ops::SubAssign<$vector> for $vector {
            #[inline]
            fn sub_assign(&mut self, rhs: $vector) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl std::ops::SubAssign<&$vector> for $vector {
            #[inline]
            fn sub_assign(&mut self, rhs: &$vector) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl const std::ops::Mul<$base> for $vector {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                }
            }
        }

        impl const std::ops::Mul<$base> for &$vector {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                }
            }
        }

        impl const std::ops::Mul<&$base> for $vector {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                    z: self.z * *rhs,
                }
            }
        }

        impl const std::ops::Mul<&$base> for &$vector {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                    z: self.z * *rhs,
                }
            }
        }

        impl const std::ops::Mul<$vector> for $base {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<$vector> for &$base {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<&$vector> for $base {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: &$vector) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<&$vector> for &$base {
            type Output = $vector;
            #[inline]
            fn mul(self, rhs: &$vector) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::MulAssign<$base> for $vector {
            #[inline]
            fn mul_assign(&mut self, rhs: $base) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }

        impl std::ops::MulAssign<&$base> for $vector {
            #[inline]
            fn mul_assign(&mut self, rhs: &$base) {
                *self *= *rhs;
            }
        }

        impl const std::ops::Div<$base> for $vector {
            type Output = $vector;
            #[inline]
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }

        impl const std::ops::Div<$base> for &$vector {
            type Output = $vector;
            #[inline]
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }

        impl const std::ops::Div<&$base> for $vector {
            type Output = $vector;
            #[inline]
            fn div(self, rhs: &$base) -> Self::Output {
                self / *rhs
            }
        }

        impl const std::ops::Div<&$base> for &$vector {
            type Output = $vector;
            #[inline]
            fn div(self, rhs: &$base) -> Self::Output {
                *self / *rhs
            }
        }

        impl std::ops::DivAssign<$base> for $vector {
            #[inline]
            fn div_assign(&mut self, rhs: $base) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }

        impl std::ops::DivAssign<&$base> for $vector {
            #[inline]
            fn div_assign(&mut self, rhs: &$base) {
                *self /= *rhs;
            }
        }

        impl const std::ops::Neg for $vector {
            type Output = $vector;
            #[inline]
            fn neg(self) -> Self::Output {
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        impl const std::ops::Neg for &$vector {
            type Output = $vector;
            #[inline]
            fn neg(self) -> Self::Output {
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }
    };
}
