use crate::{Acceleration, Mass};

pub const N: Force = Force::in_newtons(1.0);

scalar! {
    struct Force(f64) {
        fn in_newtons(newtons) -> Self;
    }
}

scalar_div!(Force | Acceleration = Mass);

#[test]
fn force_conversion() {
    use crate::{KG, M, S};
    assert_eq!(Force::in_newtons(1.0), M / S / S * KG)
}
