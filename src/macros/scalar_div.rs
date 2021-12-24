/// impl std::ops::{Add, Sub, Mul, Div} conversion for the given scalar types.
/// The vertical bar is used in place of a '/', which is not allowed in macros.
#[macro_export]
macro_rules! scalar_div {
    { $num:ty | $den:ty = $res:ty } => {
        impl const std::ops::Div<$den> for $num {
            type Output = $res;
            #[inline]
            fn div(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Div<$den> for &$num {
            type Output = $res;
            #[inline]
            fn div(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Div<&$den> for $num {
            type Output = $res;
            #[inline]
            fn div(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Div<&$den> for &$num {
            type Output = $res;
            #[inline]
            fn div(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Div<$res> for $num {
            type Output = $den;
            #[inline]
            fn div(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Div<$res> for &$num {
            type Output = $den;
            #[inline]
            fn div(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Div<&$res> for $num {
            type Output = $den;
            #[inline]
            fn div(self, rhs: &$res) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Div<&$res> for &$num {
            type Output = $den;
            #[inline]
            fn div(self, rhs: &$res) -> Self::Output {
                Self::Output::new(self.value() / rhs.value())
            }
        }

        impl const std::ops::Mul<$den> for $res {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        impl const std::ops::Mul<$den> for &$res {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        impl const std::ops::Mul<&$den> for $res {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        impl const std::ops::Mul<&$den> for &$res {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        impl const std::ops::Mul<$res> for $den {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        impl const std::ops::Mul<$res> for &$den {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        impl const std::ops::Mul<&$res> for $den {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: &$res) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        impl const std::ops::Mul<&$res> for &$den {
            type Output = $num;
            #[inline]
            fn mul(self, rhs: &$res) -> Self::Output {
                Self::Output::new(self.value() * rhs.value())
            }
        }

        paste::item! {
            #[test]
            fn [<$num:snake _ $den:snake _ $res:snake _conversion_tests>] () {
                let numerator = $num::new(6.0);
                let denominator = $den::new(2.0);
                let result = $res::new(3.0);

                assert_eq!(result, numerator / denominator);
                assert_eq!(numerator, result * denominator);
                assert_eq!(numerator, denominator * result);
                assert_eq!(denominator, numerator / result);
            }
        }
    };
}
