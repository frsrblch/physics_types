use crate::{Force, Length};

pub const J: Energy = Energy::in_joules(1.0);

scalar! {
    struct Energy(f64) {
        fn in_joules(joules) -> Self;
    }
}

scalar_div!(Energy | Length = Force);

#[test]
fn energy_conversion() {
    use crate::{M, N};
    assert_eq!(Energy::in_joules(1.0), N * M)
}
