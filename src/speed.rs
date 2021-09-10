use super::{Duration, Length};

scalar! {
    struct Speed(f64) {
        fn in_m_per_s(meters_per_second) -> Self;
    }
}

scalar_div! { Length | Duration = Speed }

impl Speed {
    pub const C: Speed = Speed::in_m_per_s(299792458.0);
}
