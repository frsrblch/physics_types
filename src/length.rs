use crate::{Duration, Mass};

pub const M: Length = Length::in_m(1.0);
pub const KM: Length = Length::in_m(1e3);
pub const AU: Length = Length::in_m(1.495978707e11);
pub const LY: Length = Length::in_m(9.4607e15);

scalar! {
    struct Length(f64) {
        fn in_m(meters) -> Self;
    }
}

impl Length {
    #[inline]
    pub fn of_orbit(mass: Mass, period: Duration) -> Self {
        use crate::constants::G;
        use std::f64::consts::TAU;

        const G_OVER_TAU_SQUARED: f64 = G / (TAU * TAU);
        const ONE_THIRD: f64 = 1.0 / 3.0;

        Length::in_m(
            (G_OVER_TAU_SQUARED * mass.value * period.value * period.value).powf(ONE_THIRD),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Vector2;

    #[test]
    fn vector_and_scalar() {
        let len = 4.0 * M;
        let dist = Vector2::in_m(2.0, 3.0);

        assert_eq!(Length::in_m(4.0), len);
        assert_eq!(Vector2::in_m(2.0, 3.0), dist)
    }

    #[test]
    fn orbit_radius() {
        // source: https://en.wikipedia.org/wiki/Orbital_period#Small_body_orbiting_a_central_body
        let expected = Length::in_m(1.0807);
        let actual = Length::of_orbit(crate::Mass::in_kg(100.0), Duration::in_hours(24.0));
        assert!((expected - actual).abs().value < 0.0001);
    }
}
