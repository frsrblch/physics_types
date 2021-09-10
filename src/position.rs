use crate::{Length, Vector2};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Position(Vector2<Length>);

impl Position {
    #[inline]
    pub const fn in_m(x: f64, y: f64) -> Self {
        Self(Vector2::in_m(x, y))
    }

    #[inline]
    pub const fn in_ly(x: f64, y: f64) -> Self {
        Self::in_m(x * Self::M_PER_LY, y * Self::M_PER_LY)
    }

    const M_PER_LY: f64 = 9.460_730_472_580_8e15;
}

#[rustfmt::skip]
impl const From<Vector2<Length>> for Position {
    #[inline]
    fn from(value: Vector2<Length>) -> Self {
        Self(value)
    }
}

impl Add<Vector2<Length>> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: Vector2<Length>) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<&Vector2<Length>> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: &Vector2<Length>) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl AddAssign<Vector2<Length>> for Position {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2<Length>) {
        self.0.add_assign(rhs);
    }
}

impl AddAssign<&Vector2<Length>> for Position {
    #[inline]
    fn add_assign(&mut self, rhs: &Vector2<Length>) {
        self.0.add_assign(rhs)
    }
}

impl Sub<Vector2<Length>> for Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: Vector2<Length>) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl Sub<&Vector2<Length>> for Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: &Vector2<Length>) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl Sub<Vector2<Length>> for &Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: Vector2<Length>) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl Sub<&Vector2<Length>> for &Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: &Vector2<Length>) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl SubAssign<Vector2<Length>> for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector2<Length>) {
        self.0.sub_assign(rhs);
    }
}

impl SubAssign<&Vector2<Length>> for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vector2<Length>) {
        self.0.sub_assign(rhs);
    }
}

impl Sub for Position {
    type Output = Vector2<Length>;
    #[inline]
    fn sub(self, rhs: Position) -> Vector2<Length> {
        self.0.sub(rhs.0)
    }
}

impl Sub<&Position> for Position {
    type Output = Vector2<Length>;
    #[inline]
    fn sub(self, rhs: &Position) -> Vector2<Length> {
        self.0.sub(rhs.0)
    }
}

impl Sub<Position> for &Position {
    type Output = Vector2<Length>;
    #[inline]
    fn sub(self, rhs: Position) -> Vector2<Length> {
        self.0.sub(rhs.0)
    }
}

impl Sub<&Position> for &Position {
    type Output = Vector2<Length>;
    #[inline]
    fn sub(self, rhs: &Position) -> Vector2<Length> {
        self.0.sub(rhs.0)
    }
}
