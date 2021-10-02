use crate::{Area, Force};

scalar! {
    struct Pressure(f64) {
        fn in_pa(pascals) -> Self;
    }
}

impl Pressure {
    pub const fn in_kpa(kilopascals: f64) -> Self {
        Self::in_pa(kilopascals * 1e3)
    }
}

scalar_div!(Force | Area = Pressure);
