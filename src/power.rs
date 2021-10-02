use crate::{Area, Duration, Energy, Force, Radius, Speed, Squared, Temperature};

scalar! {
    struct Power(f64) {
        fn in_watts(watts) -> Self;
    }
}

scalar_div!(Energy | Duration = Power);
scalar_div!(Power | Speed = Force);

scalar! {
    struct FluxDensity(f64) {
        fn in_w_per_m2(watts_per_meter_squared) -> Self;
    }
}

scalar_div!(Power | Area = FluxDensity);

impl FluxDensity {
    /// https://en.wikipedia.org/wiki/Black-body_radiation
    pub fn in_orbit(star_temp: Temperature, star_radius: Radius, orbit_radius: Radius) -> Self {
        /// https://en.wikipedia.org/wiki/Stefan%E2%80%93Boltzmann_law
        /// Units: W / m2 / K4
        const σ: f64 = 5.670374e-8;
        let t_sqr = star_temp.value * star_temp.value;
        let t_qrt = t_sqr * t_sqr;
        let r_sqr = star_radius.value * star_radius.value;
        Power::in_watts(t_qrt * r_sqr * σ) / orbit_radius.squared()
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
        let fd = FluxDensity::in_orbit(5772.0 * K, 695.7e3 * KM, 1.0 * AU);
        assert!((fd.value - 1361.16).abs() < 0.1);
    }
}
