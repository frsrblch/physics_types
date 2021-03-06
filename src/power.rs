use crate::time::Frequency;
use crate::{Area, Duration, Energy, Force, Radius, Speed, Squared, Temperature};

/// https://en.wikipedia.org/wiki/Stefan%E2%80%93Boltzmann_law
/// Units: W / m2 / K4
const SIGMA: f64 = 5.670374e-8;

scalar! {
    struct Power(f64) {
        fn in_watts(watts) -> Self;
    }
}

impl Power {
    /// Calculate power output of a black-body.
    /// Divide by the distance squared to determine the flux density
    /// https://en.wikipedia.org/wiki/Black-body_radiation
    pub fn blackbody(temp: Temperature, radius: Radius) -> Self {
        let t_sqr = temp.value * temp.value;
        let t_qrt = t_sqr * t_sqr;
        let r_sqr = radius.value * radius.value;
        Power::in_watts(t_qrt * r_sqr * SIGMA)
    }
}

scalar_div!(Energy | Duration = Power);
scalar_div!(Power | Frequency = Energy);
scalar_div!(Power | Speed = Force);

scalar! {
    struct FluxDensity(f64) {
        fn in_w_per_m2(watts_per_meter_squared) -> Self;
    }
}

scalar_div!(Power | Area = FluxDensity);

impl FluxDensity {
    pub fn in_orbit(star_temp: Temperature, star_radius: Radius, orbit_radius: Radius) -> Self {
        Power::blackbody(star_temp, star_radius) / orbit_radius.squared()
    }

    pub fn blackbody(temp: Temperature) -> Self {
        let t_sqr = temp.value * temp.value;
        let t_qrt = t_sqr * t_sqr;
        FluxDensity::in_w_per_m2(SIGMA * t_qrt)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn power_coversion() {
        use crate::{J, M, N, S};
        assert_eq!(Power::in_watts(1.0), J / S);
        assert_eq!(Power::in_watts(1.0), N * M / S);
    }

    #[test]
    fn flux_density_in_orbit() {
        use crate::{AU, K, KM};
        let fd = FluxDensity::in_orbit(5772.0 * K, 695.7e3 * KM, AU);
        assert!((fd.value - 1361.16).abs() < 0.1);
    }
}
