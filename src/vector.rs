use crate::{New, Sqrt};
use std::ops::*;

macro_rules! sum {
    (
        $e:expr $(,)?
    ) => {
        $e
    };
    (
        $e0:expr, $( $e:expr $(,)? )*
    ) => {
        $e0 + sum!{ $( $e, )*}
    };
}

macro_rules! impl_op {
    (
        $op:ident::$op_fn:ident for $v:ident { $( $f:ident, )* }
    ) => {
        impl<T, Rhs> $op<Rhs> for $v<T>
        where
            T: $op<Rhs>,
            Rhs: Copy,
        {
            type Output = $v<T::Output>;

            #[inline]
            fn $op_fn(self, rhs: Rhs) -> Self::Output {
                $v {
                    $(
                        $f: self.$f.$op_fn(rhs),
                    )*
                }
            }
        }

        impl<T, Rhs> $op<Rhs> for &$v<T>
        where
            T: $op<Rhs> + Copy,
            Rhs: Copy,
        {
            type Output = $v<T::Output>;

            #[inline]
            fn $op_fn(self, rhs: Rhs) -> Self::Output {
                $v {
                    $(
                        $f: self.$f.$op_fn(rhs),
                    )*
                }
            }
        }

        impl<T> $op<$v<T>> for f64
        where
            f64: $op<T>,
        {
            type Output = $v<<f64 as $op<T>>::Output>;
            #[inline]
            fn $op_fn(self, rhs: $v<T>) -> Self::Output {
                $v {
                    $(
                        $f: self.$op_fn(rhs.$f),
                    )*
                }
            }
        }

        impl<T> $op<$v<T>> for &f64
        where
            f64: $op<$v<T>>,
        {
            type Output = <f64 as $op<$v<T>>>::Output;
            #[inline]
            fn $op_fn(self, rhs: $v<T>) -> Self::Output {
                (*self).$op_fn(rhs)
            }
        }

        impl<T> $op<&$v<T>> for f64
        where
            T: Copy,
            f64: $op<$v<T>>,
        {
            type Output = <f64 as $op<$v<T>>>::Output;
            #[inline]
            fn $op_fn(self, rhs: &$v<T>) -> Self::Output {
                self.$op_fn(*rhs)
            }
        }

        impl<T> $op<&$v<T>> for &f64
        where
            T: Copy,
            f64: $op<$v<T>>,
        {
            type Output = <f64 as $op<$v<T>>>::Output;
            #[inline]
            fn $op_fn(self, rhs: &$v<T>) -> Self::Output {
                (*self).$op_fn(*rhs)
            }
        }
    };
}

macro_rules! impl_op_self {
    (
        $op:ident::$op_fn:ident for $v:ident { $( $f:ident, )* }
    ) => {
        impl<T: $op> $op<$v<T>> for $v <T> {
            type Output = $v<T::Output>;

            #[inline]
            fn $op_fn(self, rhs: $v<T>) -> Self::Output {
                $v {
                    $( $f: self.$f.$op_fn(rhs.$f), )*
                }
            }
        }

        impl<T: $op + Copy> $op<&$v<T>> for $v<T> {
            type Output = $v<T::Output>;

            #[inline]
            fn $op_fn(self, rhs: &$v<T>) -> Self::Output {
                self.$op_fn(*rhs)
            }
        }

        impl<T: $op + Copy> $op<$v<T>> for &$v<T> {
            type Output = $v<T::Output>;

            #[inline]
            fn $op_fn(self, rhs: $v<T>) -> Self::Output {
                (*self).$op_fn(rhs)
            }
        }

        impl<T: $op + Copy> $op<&$v<T>> for &$v<T> {
            type Output = $v<T::Output>;

            #[inline]
            fn $op_fn(self, rhs: &$v<T>) -> Self::Output {
                (*self).$op_fn(*rhs)
            }
        }
    };
}

macro_rules! impl_op_assign {
    (
        $op_assign:ident::$op_fn:ident for $v:ident { $( $f:ident, )* }
    ) => {
        impl<T, Rhs> $op_assign<Rhs> for $v<T>
        where
            T: $op_assign<Rhs>, Rhs: Copy
        {
            #[inline]
            fn $op_fn(&mut self, rhs: Rhs) {
                $( self.$f.$op_fn(rhs); )*
            }
        }
    };
}

macro_rules! impl_op_assign_self {
    (
        $op_assign:ident::$op_fn:ident for $v:ident { $( $f:ident, )* }
    ) => {
        impl<T: $op_assign> $op_assign for $v<T> {
            #[inline]
            fn $op_fn(&mut self, rhs: $v<T>) {
                $( self.$f.$op_fn(rhs.$f); )*
            }
        }

        impl<T: $op_assign + Copy> $op_assign<&$v<T>> for $v<T> {
            #[inline]
            fn $op_fn(&mut self, rhs: &$v<T>) {
                $( self.$f.$op_fn(rhs.$f); )*
            }
        }
    };
}

macro_rules! impl_op_self_only {
    (
                $op:ident::$op_fn:ident for $v:ident { $( $f:ident, )* }
    ) => {
        impl<T: $op> $op for $v<T> {
            type Output = $v<T::Output>;
            #[inline]
            fn $op_fn(self) -> Self::Output {
                $v {
                    $(
                        $f: self.$f.$op_fn(),
                    )*
                }
            }
        }

        impl<T: $op + Copy> $op for &$v<T> {
            type Output = $v<T::Output>;
            #[inline]
            fn $op_fn(self) -> Self::Output {
                $v {
                    $(
                        $f: self.$f.$op_fn(),
                    )*
                }
            }
        }
    }
}

macro_rules! vector {
    (
        struct $v:ident {
            $( $f:ident $(,)? )*
        }
    ) => {
        impl<T, F> $v <T>
        where
            T: New<Value = F> + Copy,
            F: Add<F, Output = F> + Mul<F, Output = F> + Sqrt<Output = F> + Copy,
        {
            #[inline]
            pub fn magnitude(&self) -> T {
                T::new(self.magnitude_squared_inner().sqrt())
            }

            #[inline]
            fn magnitude_squared_inner(&self) -> F {
                $(
                    let $f = self.$f.value();
                )*
                sum! { $( $f*$f, )* }
            }
        }

        impl<T, F> $v <T>
        where
            T: New<Value = F> + Default + PartialEq + Div<T, Output = F> + Copy,
            F: Add<F, Output = F> + Mul<F, Output = F> + Sqrt<Output = F> + Copy,
        {
            #[inline]
            pub fn unit_vector(&self) -> Option<$v<F>> {
                if *self == Self::default() {
                    None
                } else {
                    let magnitude = self.magnitude();
                    Some(*self / magnitude)
                }
            }
        }

        impl<T> $v <T>
        {
            #[inline]
            pub fn dot<U, V>(self, rhs: $v <U>) -> V
            where
                T: Mul<U, Output = V>,
                V: Add<V, Output = V>,
            {
                sum! {
                    $( self.$f * rhs.$f, )*
                }
            }
        }

        impl<T, U> $v <T>
        where
            T: Mul<T, Output = U> + Copy,
            U: Add<U, Output = U>,
        {
            #[inline]
            pub fn magnitude_squared(&self) -> U {
                sum! {
                    $( self.$f * self.$f, )*
                }
            }
        }

        impl_op_self! {
            Add::add for $v { $( $f, )* }
        }

        impl_op_self! {
            Sub::sub for $v { $( $f, )* }
        }

        impl_op! {
            Mul::mul for $v { $( $f, )* }
        }

        impl_op! {
            Div::div for $v { $( $f, )* }
        }

        impl_op_assign! {
            MulAssign::mul_assign for $v { $( $f, )* }
        }

        impl_op_assign! {
            DivAssign::div_assign for $v { $( $f, )* }
        }

        impl_op_assign_self! {
            AddAssign::add_assign for $v { $( $f, )* }
        }

        impl_op_assign_self! {
            SubAssign::sub_assign for $v { $( $f, )* }
        }

        impl_op_self_only! {
            Neg::neg for $v { $( $f, )* }
        }

        impl_op_self_only! {
            Not::not for $v { $( $f, )* }
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Vector2<X, Y = X> {
    pub x: X,
    pub y: Y,
}

vector! {
    struct Vector2 { x, y }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Vector3<X, Y = X, Z = X> {
    pub x: X,
    pub y: Y,
    pub z: Z,
}

vector! {
    struct Vector3 { x, y, z }
}

use crate::Angle;
impl<T: Mul<f64, Output = T> + Copy> Vector3<T, Angle, Angle> {
    pub fn euclidean(self) -> Vector3<T> {
        Vector3 {
            x: self.x * self.y.cos() * self.z.sin(),
            y: self.x * self.y.sin() * self.z.sin(),
            z: self.x * self.z.cos(),
        }
    }
}

impl<T: Default> From<Vector2<T>> for Vector3<T> {
    #[inline]
    fn from(vector2: Vector2<T>) -> Self {
        Vector3 {
            x: vector2.x,
            y: vector2.y,
            z: T::default(),
        }
    }
}

impl<T> Vector2<T>
where
    T: Copy + Mul<f64, Output = T>,
{
    /// Returns the position vector given an angle and a radius
    ///
    ///  # Arguments
    ///
    /// * `angle` - as measured anticlockwise from the positive x-axis
    /// * `magnitude` - length of the resulting vector
    #[inline]
    pub fn from_angle_and_magnitude(angle: crate::Angle, magnitude: T) -> Self {
        let x = magnitude * angle.cos();
        let y = magnitude * angle.sin();
        Self { x, y }
    }
}

impl<T> Vector3<T> {
    #[inline]
    pub fn project_xy(self) -> Vector2<T> {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Area, Duration, Length};

    #[test]
    fn sum_macro() {
        assert_eq!(1u32, sum!(1u32));
        assert_eq!(2u32, sum!(1u32, 1u32));
        assert_eq!(2u32, sum!(1u32, 1u32,));
        assert_eq!(3u32, sum!(1u32, 1u32, 1u32));
    }

    #[test]
    fn length_magnitude() {
        let length = Vector2::in_m(1.0, 2.0);

        assert_eq!(Area::in_m2(5.0), length.magnitude_squared());
        assert_eq!(Length::in_m(5f64.sqrt()), length.magnitude());
    }

    #[test]
    fn vector_size() {
        use std::mem::size_of;

        assert_eq!(8, size_of::<Vector2<f32>>());
        assert_eq!(12, size_of::<Vector3<f32>>());
        assert_eq!(16, size_of::<Vector2<f64>>());
        assert_eq!(24, size_of::<Vector3<f64>>());
    }

    #[test]
    fn vector_add() {
        let a = Vector2 {
            x: Length::in_m(1.0),
            y: Length::in_m(2.0),
        };
        let b = Vector2 {
            x: Length::in_m(2.0),
            y: Length::in_m(3.0),
        };
        let c = Vector2 {
            x: Length::in_m(3.0),
            y: Length::in_m(5.0),
        };

        assert_eq!(c, a + b);
        assert_eq!(c, a + &b);
        assert_eq!(c, &a + b);
        assert_eq!(c, &a + &b);
    }

    #[test]
    fn vector_sub() {
        let a = Vector2 {
            x: Length::in_m(3.0),
            y: Length::in_m(5.0),
        };
        let b = Vector2 {
            x: Length::in_m(2.0),
            y: Length::in_m(3.0),
        };
        let c = Vector2 {
            x: Length::in_m(1.0),
            y: Length::in_m(2.0),
        };

        assert_eq!(c, a - b);
        assert_eq!(c, a - &b);
        assert_eq!(c, &a - b);
        assert_eq!(c, &a - &b);
    }

    #[test]
    fn vector_add_assign() {
        let get_a = || Vector2 {
            x: Length::in_m(1.0),
            y: Length::in_m(2.0),
        };

        let b = Vector2 {
            x: Length::in_m(2.0),
            y: Length::in_m(3.0),
        };

        let c = Vector2 {
            x: Length::in_m(3.0),
            y: Length::in_m(5.0),
        };

        let mut a = get_a();
        a.add_assign(b);
        assert_eq!(c, a);

        let mut a = get_a();
        a.add_assign(&b);
        assert_eq!(c, a);
    }

    #[test]
    fn vector_sub_assign() {
        let get_a = || Vector2 {
            x: Length::in_m(3.0),
            y: Length::in_m(5.0),
        };

        let b = Vector2 {
            x: Length::in_m(2.0),
            y: Length::in_m(3.0),
        };

        let c = Vector2 {
            x: Length::in_m(1.0),
            y: Length::in_m(2.0),
        };

        let mut a = get_a();
        a.sub_assign(b);
        assert_eq!(c, a);

        let mut a = get_a();
        a.sub_assign(&b);
        assert_eq!(c, a);
    }

    #[test]
    fn vector_mul_scalar() {
        let v = Vector2::in_m_per_s(2.0, 3.0);
        let dt = Duration::in_s(4.0);
        let d = Vector2::in_m(8.0, 12.0);

        assert_eq!(d, v * dt);
        assert_eq!(d, v * &dt);
        assert_eq!(d, &v * dt);
        assert_eq!(d, &v * &dt);
    }

    #[test]
    fn vector_div_scalar() {
        let d = Vector2::in_m(8.0, 12.0);
        let dt = Duration::in_s(4.0);
        let v = Vector2::in_m_per_s(2.0, 3.0);

        assert_eq!(v, d / dt);
        assert_eq!(v, d / &dt);
        assert_eq!(v, &d / dt);
        assert_eq!(v, &d / &dt);
    }

    #[test]
    fn vector_mul_assign() {
        let mut d = Vector2::in_m(1.0, 2.0);
        let d2 = Vector2::in_m(2.0, 4.0);

        d *= 2.0;

        assert_eq!(d2, d);
    }

    #[test]
    fn vector_div_assign() {
        let mut d2 = Vector2::in_m(2.0, 4.0);
        let d = Vector2::in_m(1.0, 2.0);

        d2 /= 2.0;

        assert_eq!(d2, d);
    }

    #[test]
    fn unit_vector() {
        assert_eq!(None, Vector2::in_m(0.0, 0.0).unit_vector());
        assert_eq!(
            Some(Vector2 { x: 1.0, y: 0.0 }),
            Vector2::in_m(2.0, 0.0).unit_vector()
        );
    }
}
