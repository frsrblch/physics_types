use crate::{Area, Length};

pub const M3: Volume = Volume::in_m3(1.0);

scalar! {
    struct Volume(f64) {
        fn in_m3(meters_cubed) -> Self;
    }
}
scalar_div!(Volume | Area = Length);
