use crate::{Area, Duration, Energy, Force, Speed};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn power_coversion() {
        use crate::{J, M, N, S};
        assert_eq!(Power::in_watts(1.0), J / S);
        assert_eq!(Power::in_watts(1.0), N * M / S);
    }
}
