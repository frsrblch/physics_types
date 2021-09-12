use crate::{Distance, Vector2};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Position(Distance);

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
impl const From<Distance> for Position {
    #[inline]
    fn from(value: Distance) -> Self {
        Self(value)
    }
}

impl Add<Distance> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: Distance) -> Self::Output {
        self.0.add(rhs).into()
    }
}

impl Add<&Distance> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: &Distance) -> Self::Output {
        self.0.add(rhs).into()
    }
}

impl Add<Distance> for &Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: Distance) -> Self::Output {
        self.0.add(rhs).into()
    }
}

impl Add<&Distance> for &Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: &Distance) -> Self::Output {
        self.0.add(rhs).into()
    }
}

impl AddAssign<Distance> for Position {
    #[inline]
    fn add_assign(&mut self, rhs: Distance) {
        self.0.add_assign(rhs);
    }
}

impl AddAssign<&Distance> for Position {
    #[inline]
    fn add_assign(&mut self, rhs: &Distance) {
        self.0.add_assign(rhs)
    }
}

impl Sub<Distance> for Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: Distance) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl Sub<&Distance> for Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: &Distance) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl Sub<Distance> for &Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: Distance) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl Sub<&Distance> for &Position {
    type Output = Position;
    #[inline]
    fn sub(self, rhs: &Distance) -> Self::Output {
        self.0.sub(rhs).into()
    }
}

impl SubAssign<Distance> for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: Distance) {
        self.0.sub_assign(rhs);
    }
}

impl SubAssign<&Distance> for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: &Distance) {
        self.0.sub_assign(rhs);
    }
}

impl Sub for Position {
    type Output = Distance;
    #[inline]
    fn sub(self, rhs: Position) -> Distance {
        self.0.sub(rhs.0)
    }
}

impl Sub<&Position> for Position {
    type Output = Distance;
    #[inline]
    fn sub(self, rhs: &Position) -> Distance {
        self.0.sub(rhs.0)
    }
}

impl Sub<Position> for &Position {
    type Output = Distance;
    #[inline]
    fn sub(self, rhs: Position) -> Distance {
        self.0.sub(rhs.0)
    }
}

impl Sub<&Position> for &Position {
    type Output = Distance;
    #[inline]
    fn sub(self, rhs: &Position) -> Distance {
        self.0.sub(rhs.0)
    }
}
