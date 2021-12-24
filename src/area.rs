use crate::{Length, LengthInv, New, Squared};
use std::f64::consts::PI;

pub const M2: Area = Area::in_m2(1.0);

scalar! {
    struct Area(f64) {
        fn in_m2(square_meters) -> Self;
    }
}

scalar! {
    struct AreaInv(f64) {
        fn in_m2_inv(square_meters_inv) -> Self;
    }
}

impl Area {
    #[inline]
    pub fn in_square_km(value: f64) -> Self {
        Self::in_m2(value * 1e6)
    }

    #[inline]
    pub fn of_sphere(radius: Length) -> Self {
        const FOUR_PI: f64 = 4.0 * PI;
        FOUR_PI * radius.squared()
    }

    #[inline]
    pub fn of_circle(radius: Length) -> Self {
        PI * radius.squared()
    }
}

scalar_squared!(Length ^ 2 = Area);
scalar_squared!(LengthInv ^ 2 = AreaInv);

scalar_div!(LengthInv | Length = AreaInv);
scalar_div!(f64 | Area = AreaInv);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Angle, Polar, Vector2};

    #[test]
    fn radius_to_area_inv() {
        let radius = Polar {
            magnitude: Length::in_m(2.0),
            angle: Angle::PI / 6.0,
        };
        let inv_area =
            Vector2::from_angle_and_magnitude(radius.angle, 1.0 / radius.magnitude.squared());
        let surface_norm = Vector2::in_m2(0.2, 0.3); // pretend there's a z-value being ignored
        let _dot: f64 = inv_area.dot(surface_norm);
    }
}
