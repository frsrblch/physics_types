use crate::{Area, Duration, Mass, New, Squared};

pub const M: Length = Length::in_m(1.0);
pub const KM: Length = Length::in_m(1e3);
pub const AU: Length = Length::in_m(1.495978707e11);
pub const LY: Length = Length::in_m(9.460_730_472_580_8e15);

pub type Radius = Length;
pub type Distance = crate::Vector2<Length>;

scalar! {
    struct Length(f64) {
        fn in_m(meters) -> Self;
    }
}

scalar! {
    struct LengthInv(f64) {
        fn in_m_inv(meters_inv) -> Self;
    }
}

scalar_div!(Length | Area = LengthInv);
scalar_div!(f64 | Length = LengthInv);

impl Length {
    #[inline]
    pub fn of_orbit(mass: Mass, period: Duration) -> Self {
        use crate::constants::G;
        use std::f64::consts::TAU;

        const G_OVER_TAU_SQUARED: f64 = G / TAU.squared();
        const ONE_THIRD: f64 = 1.0 / 3.0;

        let meters_cubed = G_OVER_TAU_SQUARED * mass.value * period.value * period.value;
        let meters = meters_cubed.powf(ONE_THIRD);
        Length::in_m(meters)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn orbit_radius() {
        // source: https://en.wikipedia.org/wiki/Orbital_period#Small_body_orbiting_a_central_body
        let expected = Length::in_m(1.0807);
        let actual = Radius::of_orbit(crate::Mass::in_kg(100.0), Duration::in_hr(24.0));
        assert!((expected - actual).abs().value < 0.0001);
    }
}
