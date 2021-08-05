use crate::{Duration, Energy, Force, Speed};

scalar! {
    struct Power(f64) {
        fn in_watts(watts) -> Self;
    }
}

scalar_div!(Energy | Duration = Power);
scalar_div!(Power | Speed = Force);

#[test]
fn power_coversion() {
    use crate::{J, M, N, S};
    assert_eq!(Power::in_watts(1.0), J / S);
    assert_eq!(Power::in_watts(1.0), N * M / S);
}
